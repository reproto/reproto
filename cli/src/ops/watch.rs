//! build command

use crate::output::Output;
use clap::{App, Arg, ArgMatches, SubCommand};
use reproto_core::errors::Result;
use reproto_core::{Filesystem, Reporter};
use std::rc::Rc;

pub fn options<'a>() -> App<'a> {
    let out = SubCommand::with_name("watch")
        .about("Setup a file watcher that automatically builds specifications");

    let out = out.arg(
        Arg::with_name("lang")
            .long("lang")
            .takes_value(true)
            .help("Language to build for"),
    );

    let out = out.arg(
        Arg::with_name("delete")
            .long("delete")
            .help("Delete files that are no longer part of the build"),
    );

    out
}

#[cfg(not(feature = "notify"))]
pub fn entry(_: Rc<Context>, _: &mut Reporter, _: &ArgMatches, _: &Output) -> Result<()> {
    Err("`watch` command is not supported: `notify` feature is disabled".into())
}

#[cfg(feature = "notify")]
pub fn entry(fs: &dyn Filesystem, matches: &ArgMatches, output: &dyn Output) -> Result<()> {
    use crate::utils::{load_manifest, session_with_hook};
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // files discovered by the environment
    let paths: Rc<RefCell<HashSet<PathBuf>>> = Rc::new(RefCell::new(HashSet::new()));
    // files to watch
    let mut watching: HashSet<PathBuf> = HashSet::new();

    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(0))?;

    let written_files: Rc<RefCell<HashSet<PathBuf>>> = Rc::new(RefCell::new(HashSet::new()));
    let written_dirs: Rc<RefCell<HashSet<PathBuf>>> = Rc::new(RefCell::new(HashSet::new()));

    // current set of known files
    let mut files = HashSet::new();

    let delete = matches.try_contains_id("delete").unwrap_or_default();

    let mut reporter = Vec::new();

    loop {
        log::info!("updating project");

        reporter.clear();

        let update = match try_compile(
            fs,
            &mut reporter,
            matches,
            &paths,
            &written_files,
            &written_dirs,
        ) {
            Err(e) => {
                if reporter.is_empty() {
                    output.handle_error(&e, None)?;
                } else {
                    output.handle_context(&reporter)?;
                }

                false
            }
            Ok(_) => true,
        };

        // if last build _was_ successful, delete files.
        // Only actually delete things if `--delete` is specified.
        if update {
            let mut written_files = written_files.try_borrow_mut()?;

            let mut removed = Vec::new();

            files.extend(written_files.iter().cloned());

            for p in written_files.symmetric_difference(&files) {
                removed.push(p.to_owned());

                if delete {
                    log::debug!("deleting: {}", p.display());
                    drain_removed_file(p)?;
                } else {
                    log::warn!("not deleting: {} (`--delete` is not enabled)", p.display());
                }
            }

            for f in removed {
                files.remove(&f);
            }

            written_files.clear();
            let mut dirs = written_dirs.try_borrow_mut()?;

            if delete {
                drain_created_dirs(&mut dirs)?;
            } else {
                for d in dirs.iter() {
                    log::warn!("not deleting: {} (`--delete` is not enabled)", d.display());
                }
            }
        }

        // move additional watch paths, unless an error occurred.
        if update {
            let mut paths = paths.try_borrow_mut()?;

            for p in watching.symmetric_difference(&paths) {
                log::debug!("watch: {}", p.display());

                if let Some(parent) = p.parent() {
                    watcher.watch(parent, RecursiveMode::NonRecursive)?;
                }

                watcher.watch(p, RecursiveMode::NonRecursive)?;
            }

            for p in watching.difference(&paths) {
                log::debug!("unwatch: {}", p.display());

                if let Some(parent) = p.parent() {
                    watcher.unwatch(parent)?;
                }

                watcher.unwatch(p)?;
            }

            watching.clear();
            watching.extend(paths.drain());
        }

        if watching.is_empty() {
            log::info!("Nothing being watched, exiting...");
            break;
        }

        let mut sleep = false;

        loop {
            use notify::DebouncedEvent::*;

            match rx.recv() {
                Ok(e) => match e {
                    // special handling to avoid debouncing?
                    NoticeRemove(p) | Remove(p) => {
                        sleep = true;

                        if watching.contains(&p) {
                            break;
                        }
                    }
                    NoticeWrite(p) | Create(p) | Write(p) | Chmod(p) => {
                        if watching.contains(&p) {
                            break;
                        }
                    }
                    Rescan => {
                        break;
                    }
                    Rename(from, to) => {
                        sleep = true;

                        if watching.contains(&from) || watching.contains(&to) {
                            break;
                        }
                    }
                    Error(e, p) => {
                        if let Some(p) = p {
                            return Err(
                                format!("error watching path: {}: {}", p.display(), e).into()
                            );
                        }

                        return Err(e.into());
                    }
                },
                Err(e) => return Err(e.into()),
            }
        }

        if sleep {
            // Sleep to permit file changes to settle...
            thread::sleep(Duration::from_secs(1));
        }

        // try to extra eventsto bounce less...
        loop {
            match rx.try_recv() {
                Err(mpsc::TryRecvError::Empty) => break,
                Err(e) => return Err(e.into()),
                Ok(_) => {}
            }
        }
    }

    return Ok(());

    /// Remove a file and all it's parent directories if empty.
    fn drain_removed_file(file: &Path) -> Result<()> {
        if file.is_file() {
            fs::remove_file(&file)?;
        }

        // drain directory, if empty

        let mut parent = file.parent();

        while let Some(p) = parent {
            // will fail unless directory is empty.
            match fs::remove_dir(p) {
                Err(_) => break,
                Ok(_) => {}
            }

            parent = p.parent();
        }

        Ok(())
    }

    /// Remove directories.
    fn drain_created_dirs(dirs: &mut HashSet<PathBuf>) -> Result<()> {
        for d in dirs.drain() {
            log::debug!("deleting: {}", d.display());
            // we don't care if it succeeds or not.
            let _ = fs::remove_dir(d);
        }

        Ok(())
    }

    fn try_compile(
        fs: &dyn Filesystem,
        reporter: &mut dyn Reporter,
        matches: &ArgMatches,
        paths: &Rc<RefCell<HashSet<PathBuf>>>,
        added_files: &Rc<RefCell<HashSet<PathBuf>>>,
        added_dirs: &Rc<RefCell<HashSet<PathBuf>>>,
    ) -> Result<()> {
        let fs = stalker::StalkerFilesystem::new(fs, added_files.clone(), added_dirs.clone());

        let manifest = load_manifest(matches)?;
        let lang = manifest.lang().ok_or_else(|| "no language to build for")?;

        if let Some(path) = manifest.path.as_ref() {
            let path = path
                .to_owned()
                .canonicalize()
                .map_err(|e| format!("{}: {}", path.display(), e))?;
            paths.try_borrow_mut()?.insert(path);
        }

        let local_paths = paths.clone();

        let manifest = load_manifest(matches)?;
        let mut resolver = env::resolver(&manifest)?;

        let session = session_with_hook(
            lang.copy(),
            &manifest,
            reporter,
            resolver.as_mut(),
            move |p| {
                let p = p
                    .to_owned()
                    .canonicalize()
                    .map_err(|e| format!("{}: {}", p.display(), e))?;
                local_paths.try_borrow_mut()?.insert(p);
                Ok(())
            },
        )?;

        let handle = fs.open_root(manifest.output.as_ref().map(AsRef::as_ref))?;
        lang.compile(handle.as_ref(), session, manifest)?;
        Ok(())
    }
}

#[cfg(feature = "notify")]
mod stalker {
    use reproto_core::errors::Result;
    use reproto_core::{Filesystem, Handle, RelativePath};
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::io;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    /// A filesystem implementation that keeps track of files which have been opened for writing.
    pub struct StalkerFilesystem<'a> {
        delegate: &'a dyn Filesystem,
        files: Rc<RefCell<HashSet<PathBuf>>>,
        dirs: Rc<RefCell<HashSet<PathBuf>>>,
    }

    impl<'a> StalkerFilesystem<'a> {
        pub fn new(
            delegate: &'a dyn Filesystem,
            files: Rc<RefCell<HashSet<PathBuf>>>,
            dirs: Rc<RefCell<HashSet<PathBuf>>>,
        ) -> StalkerFilesystem {
            Self {
                delegate,
                files,
                dirs,
            }
        }
    }

    impl<'a> Filesystem for StalkerFilesystem<'a> {
        fn open_root(&self, root: Option<&Path>) -> Result<Box<dyn Handle>> {
            let delegate = self.delegate.open_root(root.clone())?;

            return Ok(Box::new(StalkerHandle {
                delegate,
                root: root.clone().map(ToOwned::to_owned),
                files: self.files.clone(),
                dirs: self.dirs.clone(),
            }));
        }
    }

    /// A handle that captures files into a RefCell.
    struct StalkerHandle {
        delegate: Box<dyn Handle>,
        root: Option<PathBuf>,
        files: Rc<RefCell<HashSet<PathBuf>>>,
        dirs: Rc<RefCell<HashSet<PathBuf>>>,
    }

    impl Handle for StalkerHandle {
        fn is_dir(&self, path: &RelativePath) -> bool {
            self.delegate.is_dir(path)
        }

        fn is_file(&self, path: &RelativePath) -> bool {
            self.delegate.is_file(path)
        }

        fn create_dir_all(&self, path: &RelativePath) -> Result<()> {
            if let Some(root) = self.root.as_ref() {
                let mut dirs = self.dirs.try_borrow_mut()?;
                dirs.insert(path.to_path(root));
            }

            self.delegate.create_dir_all(path)
        }

        fn create(&self, path: &RelativePath) -> Result<Box<dyn io::Write>> {
            match self.delegate.create(path) {
                Ok(w) => {
                    if let Some(root) = self.root.as_ref() {
                        let mut files = self.files.try_borrow_mut()?;
                        files.insert(path.to_path(root));
                    }

                    Ok(w)
                }
                r => r,
            }
        }
    }
}
