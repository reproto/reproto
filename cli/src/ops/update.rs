//! Update action that synchronizes all repositories.

use crate::utils::load_manifest;
use clap::{App, ArgMatches, SubCommand};
use repository::Update;
use reproto_core::errors::Result;
use std::collections::HashSet;

pub fn options<'a>() -> App<'a> {
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
