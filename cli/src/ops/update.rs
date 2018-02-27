//! Update action that synchronizes all repositories.

use build_spec::{convert_lang, manifest, manifest_preamble, repository};
use clap::{App, ArgMatches, SubCommand};
use core::Context;
use core::errors::*;
use manifest::NoLang;
use repository::Update;
use std::collections::HashSet;
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(_ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;

    let lang = preamble
        .language
        .as_ref()
        .map(|l| convert_lang(*l))
        .unwrap_or_else(|| Box::new(NoLang));

    let manifest = manifest(lang.as_ref(), matches, preamble)?;

    let repository = repository(&manifest)?;
    let updates: HashSet<Update> = repository.update()?.into_iter().collect();

    for update in updates {
        update.update()?;
    }

    Ok(())
}
