//! Update action that synchronizes all repositories.

use manifest::{Lang, Manifest};
use ops::imports::*;
use repository::Update;
use std::collections::HashSet;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;
    return do_manifest_use!(matches, preamble, inner);

    fn inner<L>(_matches: &ArgMatches, manifest: Manifest<L>) -> Result<()>
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
