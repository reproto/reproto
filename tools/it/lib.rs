extern crate diff;
#[macro_use]
extern crate failure;
extern crate relative_path;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate walkdir;

use relative_path::{RelativePath, RelativePathBuf};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;
use std::time::{Duration, Instant};

mod utils;

#[macro_export]
macro_rules! define {
    ($($suite:ident => $blk:block,)*) => {
        $(
        #[allow(unused)]
        fn $suite($suite: &mut $crate::Suite) $blk
        )*

        pub fn entry<'a>(test: &'a str, project: &mut $crate::Project<'a>) {
            $(
            let suite = stringify!($suite).trim_matches('_');
            let mut suite = $crate::Suite::new(
                test, suite, project.do_suite, project.do_project, project.action
            );
            $suite(&mut suite);
            project.suite(suite);
            )*
        }
    }
}

macro_rules! tests {
    ($($name:ident)*) => {
        $(pub mod $name;)*

        pub fn entry(project: &mut $crate::Project) {
            $(
            let test = stringify!($name).trim_matches('_');
            $name::entry(test, project);
            )*
        }
    }
}

tests!(tests);

#[derive(Debug)]
pub struct DurationFmt(pub Duration);

impl fmt::Display for DurationFmt {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let d = self.0;

        let (secs, ns) = (d.as_secs(), d.subsec_nanos());

        if secs > 0 {
            return write!(fmt, "{}.{}s", secs, (ns / 10_000_000) % 100);
        }

        match ns {
            ns if ns / 1_000_000 > 0 => write!(fmt, "{}ms", ns / 1_000_000),
            ns if ns / 1_000 > 0 => write!(fmt, "{}us", ns / 1_000),
            ns => write!(fmt, "{}ns", ns),
        }
    }
}

/// Wrapping the reproto command invocation.
#[derive(Debug, Clone)]
pub struct Reproto {
    /// Path to binary.
    binary: PathBuf,
}

impl Reproto {
    pub fn from_project(cli: PathBuf) -> Result<Reproto, failure::Error> {
        let mut cmd = Command::new("cargo");

        cmd.arg("build");
        cmd.arg("--manifest-path");
        cmd.arg(cli.join("Cargo.toml").display().to_string());
        cmd.arg("--message-format");
        cmd.arg("json");

        let mut child = cmd.stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format_err!("bad exit status: {}", e))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| format_err!("failed to get stdout"))?;

        for doc in json::Deserializer::from_reader(stdout).into_iter() {
            let doc: json::Value = doc?;

            let doc: CargoLine = match CargoLine::deserialize(doc) {
                Ok(doc) => doc,
                Err(_) => continue,
            };

            if doc.target.kind.contains(&CargoKind::Bin) {
                let binary = doc.filenames
                    .into_iter()
                    .next()
                    .ok_or_else(|| format_err!("expected one file name"))?;

                let binary = Path::new(&binary).to_path_buf();
                return Ok(Self::new(binary));
            }
        }

        bail!("could not build binary");

        #[derive(Debug, Deserialize, PartialEq, Eq)]
        pub enum CargoKind {
            #[serde(rename = "bin")]
            Bin,
            #[serde(rename = "lib")]
            Lib,
        }

        #[derive(Debug, Deserialize)]
        pub struct CargoTarget {
            kind: Vec<CargoKind>,
        }

        #[derive(Debug, Deserialize)]
        pub struct CargoLine {
            filenames: Vec<String>,
            target: CargoTarget,
        }
    }

    pub fn new(binary: PathBuf) -> Self {
        Self { binary: binary }
    }

    /// Build a reproto project.
    pub fn build(&self, manifest: Manifest) -> Result<(), failure::Error> {
        if !manifest.path.is_dir() {
            bail!("No such proto path: {}", manifest.path.display());
        }

        let mut cmd = Command::new(&self.binary);

        if false {
            cmd.arg("--debug");
        }

        cmd.arg("build");
        cmd.args(&["--lang", manifest.language.lang()]);

        if let Some(package_prefix) = manifest.package_prefix {
            cmd.args(&["--package-prefix", package_prefix]);
        }

        // Output directory.
        cmd.args(&["-o", manifest.output.display().to_string().as_str()]);
        // Disable using local repository.
        cmd.arg("--no-repository");
        // Path to resolve packages from.
        cmd.args(&["--path", manifest.path.display().to_string().as_str()]);

        for package in manifest.packages {
            cmd.args(&["--package", package.as_str()]);
        }

        cmd.args(manifest.extra);

        let output = cmd.current_dir(manifest.current_dir)
            .output()
            .map_err(|e| format_err!("bad exit status: {}", e))?;

        if !output.status.success() {
            bail!(
                "failed to run reproto on project: {}: {}",
                manifest.current_dir.display(),
                output.status
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Language {
    Csharp,
    Java,
    JavaScript,
    Python,
    Python3,
    Rust,
}

impl Language {
    /// Get the name of the working directory.
    pub fn name(&self) -> &'static str {
        use self::Language::*;

        match *self {
            Csharp => "csharp",
            Java => "java",
            JavaScript => "js",
            Python => "python",
            Python3 => "python3",
            Rust => "rust",
        }
    }

    /// Value to `--lang` argument.
    pub fn lang(&self) -> &'static str {
        use self::Language::*;

        match *self {
            Csharp => "csharp",
            Java => "java",
            JavaScript => "js",
            Python => "python",
            Python3 => "python",
            Rust => "rust",
        }
    }

    /// Output directory for language.
    pub fn output(&self) -> &'static RelativePath {
        use self::Language::*;

        match *self {
            Csharp => RelativePath::new("."),
            Java => RelativePath::new("target/generated-sources/reproto"),
            JavaScript => RelativePath::new("generated"),
            Python => RelativePath::new("generated"),
            Python3 => RelativePath::new("generated"),
            Rust => RelativePath::new("src"),
        }
    }

    /// Package prefix to use
    pub fn package_prefix(&self) -> Option<&'static str> {
        use self::Language::*;

        match *self {
            Rust => Some("generated"),
            _ => None,
        }
    }

    /// Working directory source.
    pub fn source_workdir(&self, root: &Path) -> PathBuf {
        root.join("workdir").join(self.name())
    }

    /// Resolve a directory with this language at the end.
    pub fn path(&self, root: &Path, parts: &[&str]) -> PathBuf {
        let mut out = root.to_owned();

        for p in parts {
            out = out.join(p);
        }

        out.join(self.name())
    }
}

pub trait Runner: Send {
    /// Check if the current runner matches the given filters.
    fn keywords(&self) -> Vec<&str> {
        vec![]
    }

    /// Run the current runner.
    fn run(&self) -> Result<(), failure::Error>;
}

#[derive(Debug)]
pub struct Manifest<'m> {
    /// Path to build packages from.
    path: &'m Path,
    /// Working directory.
    current_dir: &'m Path,
    /// Output directory.
    output: PathBuf,
    /// Language-specific options.
    language: &'m Language,
    /// Build the given packages.
    packages: &'m [String],
    /// Extra arguments.
    extra: &'m [String],
    /// Package prefix to apply.
    package_prefix: Option<&'m str>,
}

#[derive(Debug)]
pub struct ProjectRunner<'a> {
    test: &'a str,
    suite: &'a str,
    /// Path to build packages from.
    path: PathBuf,
    /// Package to build.
    packages: Vec<String>,
    /// Inputs to feed to the project.
    inputs: Vec<PathBuf>,
    /// Source directory from where to build project.
    source_workdir: PathBuf,
    /// Target directory to build project.
    target_workdir: PathBuf,
    /// Current project directory.
    current_dir: PathBuf,
    /// Language-specific options.
    language: &'a Language,
    /// Reproto command wrapper.
    reproto: &'a Reproto,
    /// Extra arguments.
    extra: Vec<String>,
}

impl<'a> ProjectRunner<'a> {
    pub fn manifest<'m>(&'m self) -> Manifest<'m> {
        Manifest {
            path: &self.path,
            current_dir: &self.current_dir,
            output: self.language.output().to_path(&self.target_workdir),
            language: self.language,
            packages: &self.packages,
            extra: &self.extra,
            package_prefix: self.language.package_prefix(),
        }
    }

    fn try_run(&self) -> Result<(), failure::Error> {
        let script = self.target_workdir.join("script.sh");

        utils::copy_dir(&self.source_workdir, &self.target_workdir)?;

        self.reproto.build(self.manifest())?;

        // building project
        let output = Command::new("make")
            .current_dir(&self.target_workdir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|e| {
                format_err!(
                    "failed to build project: {}: {}",
                    self.target_workdir.display(),
                    e
                )
            })?;

        if !output.status.success() {
            bail!(
                "failed to make project: {}: {}",
                self.target_workdir.display(),
                output.status
            );
        }

        if !script.is_file() {
            bail!("missing script.sh entrypoint: {}", script.display());
        }

        let mut child = Command::new(script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut actual: Vec<json::Value> = Vec::new();
        let mut expected: Vec<json::Value> = Vec::new();

        {
            let mut stdin = child.stdin.take().ok_or_else(|| format_err!("no stdin"))?;

            for input in &self.inputs {
                let f = File::open(&input).map_err(|e| format_err!("{}: {}", input.display(), e))?;

                for line in BufReader::new(f).lines() {
                    let line = line?;

                    // skip comments.
                    if line.starts_with('#') {
                        continue;
                    }

                    expected.push(json::from_str(&line)?);
                    writeln!(stdin, "{}", line)?;
                }
            }

            drop(stdin);

            let stdout = child.stdout.take().ok_or_else(|| format_err!("no stdout"))?;

            for line in BufReader::new(stdout).lines() {
                let line = line?;
                actual.push(json::from_str(&line)?);
            }
        }

        child.wait()?;

        if actual.len() != expected.len() {
            bail!(
                "number of JSON documents ({}) do not match expected ({})",
                actual.len(),
                expected.len(),
            );
        }

        let mut errors = Vec::new();

        for (i, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
            if actual == expected {
                continue;
            }

            errors.push(format!(
                "#{} JSON mismatch: {} (actual) != {} (expected)",
                i, actual, expected
            ));
        }

        if !errors.is_empty() {
            bail!("test failed: {}", errors.join(", "));
        }

        Ok(())
    }
}

impl<'a> Runner for ProjectRunner<'a> {
    fn keywords(&self) -> Vec<&str> {
        vec![self.test, self.suite, self.language.name()]
    }

    /// Run the suite.
    fn run(&self) -> Result<(), failure::Error> {
        let before = Instant::now();
        let id = format!("project:{}/{}", self.suite, self.language.name());
        self.try_run().map_err(|e| format_err!("{}: {}", id, e))?;
        let duration = Instant::now() - before;
        println!("done {} ({})", id, DurationFmt(duration));
        Ok(())
    }
}

#[derive(Debug)]
pub struct SuiteRunner<'a> {
    test: &'a str,
    suite: &'a str,
    /// Action to run.
    action: Action,
    /// Path to build packages from.
    path: PathBuf,
    /// Package to build.
    packages: Vec<String>,
    /// Source directory to verify structure against.
    expected_struct: PathBuf,
    /// Target directory to build structure.
    target_struct: PathBuf,
    /// Current directory of project to build.
    current_dir: PathBuf,
    /// Language-specific options.
    language: &'a Language,
    /// Reproto command wrapper.
    reproto: &'a Reproto,
    /// Extra arguments.
    extra: Vec<String>,
}

impl<'a> SuiteRunner<'a> {
    pub fn manifest<'m>(&'m self, output: &'m Path) -> Manifest<'m> {
        Manifest {
            path: &self.path,
            current_dir: &self.current_dir,
            output: output.to_owned(),
            language: self.language,
            packages: &self.packages,
            extra: &self.extra,
            package_prefix: None,
        }
    }

    fn try_run(&self) -> Result<(), failure::Error> {
        use utils::Diff::*;

        match self.action {
            Action::Update => {
                fs::create_dir_all(&self.expected_struct)?;
                self.reproto.build(self.manifest(&self.expected_struct))?;
            }
            Action::Verify => {
                if self.target_struct.is_dir() {
                    // Remove existing directory, and re-recreate it.
                    fs::remove_dir_all(&self.target_struct)?;
                }

                fs::create_dir_all(&self.target_struct)?;
                self.reproto.build(self.manifest(&self.target_struct))?;

                let mut errors = Vec::new();
                utils::diff_recursive(&self.expected_struct, &self.target_struct, &mut errors)?;

                if errors.is_empty() {
                    return Ok(());
                }

                println!(
                    "differences between {} ({}) and {} ({})",
                    self.expected_struct.display(),
                    utils::Location::Source.display(),
                    self.target_struct.display(),
                    utils::Location::Dest.display(),
                );

                for e in errors {
                    match e {
                        MissingDir(loc, dir) => {
                            println!("missing dir in {}:{}", loc.display(), dir.display());
                        }
                        ExpectedDir(loc, file) => {
                            println!("expected dir in {}:{}", loc.display(), file.display());
                        }
                        MissingFile(loc, file) => {
                            println!("missing file in {}:{}", loc.display(), file.display());
                        }
                        ExpectedFile(loc, file) => {
                            println!("expected file in {}:{}", loc.display(), file.display());
                        }
                        Mismatch(src, _, mismatch) => {
                            let added = mismatch
                                .iter()
                                .map(|m| match m {
                                    &diff::Result::Right(_) => 1,
                                    _ => 0,
                                })
                                .sum::<u32>();

                            let removed = mismatch
                                .iter()
                                .map(|m| match m {
                                    &diff::Result::Left(_) => 1,
                                    _ => 0,
                                })
                                .sum::<u32>();

                            println!("{}: +{}, -{}", src.display(), added, removed);

                            for m in mismatch {
                                match m {
                                    diff::Result::Left(l) => println!("-{}", l),
                                    diff::Result::Right(r) => println!("+{}", r),
                                    diff::Result::Both(l, _) => println!(" {}", l),
                                }
                            }
                        }
                    }
                }

                bail!(
                    "differences between {} ({}) and {} ({})",
                    self.expected_struct.display(),
                    utils::Location::Source.display(),
                    self.target_struct.display(),
                    utils::Location::Dest.display(),
                );
            }
        }

        Ok(())
    }
}

impl<'a> Runner for SuiteRunner<'a> {
    fn keywords(&self) -> Vec<&str> {
        vec![self.test, self.suite, self.language.name()]
    }

    /// Run the suite.
    fn run(&self) -> Result<(), failure::Error> {
        let before = Instant::now();
        let id = format!("suite:{}/{}", self.suite, self.language.name());
        self.try_run().map_err(|e| format_err!("{}: {}", id, e))?;
        let duration = Instant::now() - before;
        println!("done {} ({})", id, DurationFmt(duration));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Update the suite.
    Update,
    /// Verify the suite.
    Verify,
}

/// A test suite.
#[derive(Debug)]
pub struct Suite<'a> {
    test: &'a str,
    suite: &'a str,
    /// Run the suite.
    do_suite: bool,
    /// Run the project.
    do_project: bool,
    /// Suite action to apply.
    action: Action,
    /// Proto path to build.
    proto: Vec<RelativePathBuf>,
    inputs: Vec<RelativePathBuf>,
    arguments: HashMap<Language, &'a [&'a str]>,
    /// Build the given packages.
    packages: Vec<String>,
    /// Extract suite from the given directory.
    dir: Option<RelativePathBuf>,
    /// Include only the following languages.
    include: HashSet<Language>,
}

impl<'a> Suite<'a> {
    pub fn new(
        test: &'a str,
        suite: &'a str,
        do_suite: bool,
        do_project: bool,
        action: Action,
    ) -> Self {
        Self {
            test: test,
            suite: suite,
            do_suite: do_suite,
            do_project: do_project,
            action: action,
            proto: Vec::new(),
            inputs: Vec::new(),
            arguments: HashMap::new(),
            packages: vec!["test".to_string()],
            dir: None,
            include: HashSet::new(),
        }
    }

    /// Build only the given languages.
    pub fn include(&mut self, language: Language) {
        self.include.insert(language);
    }

    /// Extract project configuration from the given directory.
    pub fn dir<P: AsRef<RelativePath>>(&mut self, path: P) {
        self.dir = Some(path.as_ref().to_relative_path_buf());
    }

    /// Set the package to build.
    pub fn package<P: AsRef<str>>(&mut self, package: P) {
        self.packages.push(package.as_ref().to_string());
    }

    /// Associate an argument with a given language.
    pub fn arg(&mut self, lang: Language, args: &'a [&'a str]) {
        self.arguments.insert(lang, args);
    }

    /// Hook up another proto file.
    pub fn proto<P: AsRef<RelativePath>>(&mut self, path: P) {
        self.proto.push(path.as_ref().to_owned());
    }

    /// Hook up another input file.
    pub fn input<P: AsRef<RelativePath>>(&mut self, path: P) {
        self.inputs.push(path.as_ref().to_owned());
    }

    /// Setup suite runners for suite.
    pub fn suite_runners(
        self,
        root: &Path,
        project: &'a Project<'a>,
    ) -> Result<Vec<Box<'a + Runner>>, failure::Error> {
        let mut runners: Vec<Box<Runner>> = Vec::new();

        let suite = self.suite;

        let current_dir = self.dir
            .as_ref()
            .map(|p| p.as_ref())
            .unwrap_or_else(|| RelativePath::new(suite))
            .to_path(root);

        if !current_dir.is_dir() {
            bail!("expected directory: {}", current_dir.display());
        }

        let path = current_dir.join("proto");
        let input = current_dir.join("input");

        let mut inputs = Vec::new();

        inputs.extend(json_files(&current_dir)?);
        inputs.extend(files_in_dir(&input)?);

        for language in project.languages {
            if !self.include.is_empty() && !self.include.contains(language) {
                continue;
            }

            let mut extra = Vec::new();

            if let Some(args) = self.arguments.get(language) {
                extra.extend(args.iter().map(|a| a.to_string()));
            }

            if let Some(args) = project.arguments.get(language) {
                extra.extend(args.iter().map(|a| a.to_string()));
            }

            if self.do_project {
                let source_workdir = language.source_workdir(root);
                let target_workdir =
                    language.path(root, &["target", "workdir", self.test, self.suite]);

                runners.push(Box::new(ProjectRunner {
                    test: self.test,
                    suite: self.suite,
                    path: path.clone(),
                    packages: self.packages.clone(),
                    inputs: inputs.clone(),
                    source_workdir: source_workdir,
                    target_workdir: target_workdir,
                    current_dir: current_dir.clone(),
                    language: language,
                    reproto: project.reproto,
                    extra: extra.clone(),
                }));
            }

            if self.do_suite {
                let expected_struct = language.path(root, &["expected", self.test, self.suite]);
                let target_struct =
                    language.path(root, &["target", "struct", self.test, self.suite]);

                runners.push(Box::new(SuiteRunner {
                    test: self.test,
                    suite: self.suite,
                    action: self.action,
                    path: path.clone(),
                    packages: self.packages.clone(),
                    expected_struct: expected_struct,
                    target_struct: target_struct,
                    current_dir: current_dir.clone(),
                    language: language,
                    reproto: project.reproto,
                    extra: extra.clone(),
                }));
            }
        }

        return Ok(runners);

        /// Read path to all JSON files in the given directory.
        fn json_files(dir: &Path) -> Result<Vec<PathBuf>, failure::Error> {
            let mut out = Vec::new();

            for e in fs::read_dir(dir)? {
                let p = e?.path();

                let has_ext = p.extension().map(|ext| ext == "json").unwrap_or(false);

                if p.is_file() && has_ext {
                    out.push(p);
                }
            }

            Ok(out)
        }

        /// Read all files in directory.
        fn files_in_dir(dir: &Path) -> Result<Vec<PathBuf>, failure::Error> {
            let mut out = Vec::new();

            if !dir.is_dir() {
                return Ok(out);
            }

            for e in fs::read_dir(&dir)? {
                let p = e?.path();

                if p.is_file() {
                    out.push(p);
                }
            }

            Ok(out)
        }
    }
}

/// A project, collecting all suites.
#[derive(Debug)]
pub struct Project<'a> {
    languages: &'a [Language],
    reproto: &'a Reproto,
    /// Run the suite.
    do_suite: bool,
    /// Run the project.
    do_project: bool,
    /// Action to run.
    action: Action,
    pub suites: Vec<Suite<'a>>,
    arguments: HashMap<Language, &'a [&'a str]>,
}

impl<'a> Project<'a> {
    pub fn new(
        languages: &'a [Language],
        reproto: &'a Reproto,
        do_suite: bool,
        do_project: bool,
        action: Action,
    ) -> Self {
        Self {
            languages: languages,
            reproto: reproto,
            do_suite: do_suite,
            do_project: do_project,
            action: action,
            suites: Vec::new(),
            arguments: HashMap::new(),
        }
    }

    /// Associate an argument with a given language.
    pub fn arg(&mut self, lang: Language, args: &'a [&'a str]) {
        self.arguments.insert(lang, args);
    }

    /// Hook up another suite.
    pub fn suite(&mut self, suite: Suite<'a>) {
        self.suites.push(suite);
    }
}
