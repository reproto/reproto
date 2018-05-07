//! build command

use build_spec::{environment, load_manifest};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::{Filesystem, Reporter};
use core::errors::Result;
use env;

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

pub fn entry(fs: &Filesystem, reporter: &mut Reporter, matches: &ArgMatches) -> Result<()> {
    let manifest = load_manifest(matches)?;
    let lang = manifest.lang().ok_or_else(|| {
        "no language to build for, either specify in manifest under `language` or `--lang`"
    })?;

    let mut resolver = env::resolver(&manifest)?;
    let handle = fs.open_root(manifest.output.as_ref().map(AsRef::as_ref))?;
    let env = environment(lang.copy(), &manifest, reporter, resolver.as_mut())?;
    lang.compile(handle.as_ref(), env, manifest)?;
    Ok(())
}
