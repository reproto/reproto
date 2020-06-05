//! Action to build documentation.

use crate::utils::{load_manifest, simple_config};
use clap::{App, ArgMatches, SubCommand};
use core::errors::Result;
use core::Reporter;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    doc::shared_options(SubCommand::with_name("doc").about("Generate documentation"))
}

pub fn entry(reporter: &mut dyn Reporter, matches: &ArgMatches) -> Result<()> {
    let manifest = load_manifest(matches)?;
    let mut resolver = env::resolver(&manifest)?;
    let session = simple_config(&manifest, reporter, resolver.as_mut())?;
    doc::compile(session, matches, manifest).map_err(Into::into)
}
