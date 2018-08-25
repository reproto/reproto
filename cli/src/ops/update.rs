//! Update action that synchronizes all repositories.

use clap::{App, ArgMatches, SubCommand};
use core::errors::*;
use env;
use repository::Update;
use std::collections::HashSet;
use utils::load_manifest;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let manifest = load_manifest(matches)?;

    let repository = env::repository(&manifest)?;
    let updates: HashSet<Update> = repository.update()?.into_iter().collect();

    for update in updates {
        update.update()?;
    }

    Ok(())
}
