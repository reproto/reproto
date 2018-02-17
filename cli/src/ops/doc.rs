//! Action to build documentation.

use build_spec::{manifest_preamble, setup_environment};
use clap::{App, ArgMatches, SubCommand};
use core::Context;
use core::errors::*;
use manifest::{Lang, Manifest};
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    ::doc::shared_options(SubCommand::with_name("doc").about("Generate documentation"))
}

pub fn entry(ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;

    return do_manifest_use!(ctx, matches, preamble, inner);

    fn inner<L>(ctx: Rc<Context>, matches: &ArgMatches, manifest: Manifest<L>) -> Result<()>
    where
        L: Lang,
    {
        let env = setup_environment(ctx.clone(), &manifest)?;
        ::doc::compile(env, matches, manifest).map_err(Into::into)
    }
}
