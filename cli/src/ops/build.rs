//! build command

use crate::utils::{load_manifest, session};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::errors::Result;
use core::{Filesystem, Reporter};

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("build").about("Build specifications");

    let out = out.arg(
        Arg::with_name("lang")
            .long("lang")
            .takes_value(true)
            .help("Language to build for"),
    );

    out
}

pub fn entry(fs: &dyn Filesystem, reporter: &mut dyn Reporter, matches: &ArgMatches) -> Result<()> {
    let manifest = load_manifest(matches)?;
    let lang = manifest.lang().ok_or_else(|| {
        "no language to build for, either specify in manifest under `language` or `--lang`"
    })?;

    let mut resolver = env::resolver(&manifest)?;
    let handle = fs.open_root(manifest.output.as_ref().map(AsRef::as_ref))?;
    let session = session(lang.copy(), &manifest, reporter, resolver.as_mut())?;
    lang.compile(handle.as_ref(), session, manifest)?;
    Ok(())
}
