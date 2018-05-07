//! Action to build documentation.

use build_spec::{load_manifest, simple_config};
use clap::{App, ArgMatches, SubCommand};
use core::Reporter;
use core::errors::*;
use env;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    ::doc::shared_options(SubCommand::with_name("doc").about("Generate documentation"))
}

pub fn entry(reporter: &mut Reporter, matches: &ArgMatches) -> Result<()> {
    let manifest = load_manifest(matches)?;
    let mut resolver = env::resolver(&manifest)?;
    let env = simple_config(&manifest, reporter, resolver.as_mut())?;
    ::doc::compile(env, matches, manifest).map_err(Into::into)
}
