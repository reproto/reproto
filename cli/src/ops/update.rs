//! Update action that synchronizes all repositories.

use build_spec::{manifest_preamble, setup_repository};
use clap::{App, ArgMatches, SubCommand};
use core::Context;
use errors::*;
use manifest::{Lang, Manifest};
use repository::Update;
use std::collections::HashSet;
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;
    return do_manifest_use!(ctx, matches, preamble, inner);

    fn inner<L>(_ctx: Rc<Context>, _matches: &ArgMatches, manifest: Manifest<L>) -> Result<()>
    where
        L: Lang,
    {
        let repository = setup_repository(&manifest)?;
        let updates: HashSet<Update> = repository.update()?.into_iter().collect();

        for update in updates {
            update.update()?;
        }

        Ok(())
    }
}
