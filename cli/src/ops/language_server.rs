//! Update action that synchronizes all repositories.

#[cfg(feature = "languageserver")]
extern crate reproto_languageserver as ls;

use build_spec::{convert_lang, manifest, manifest_preamble};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::Context;
use core::errors::Result;
use manifest::Language;
use std::io;
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("language-server").about("Run the language server for reproto");

    let out = out.arg(
        Arg::with_name("lang")
            .long("lang")
            .takes_value(true)
            .help("Language to build for"),
    );

    out
}

pub fn entry(_ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;

    let language = preamble
        .language
        .as_ref()
        .cloned()
        .or_else(|| matches.value_of("lang").and_then(Language::parse))
        .ok_or_else(|| "no language specified either through manifest or cli (--lang)")?;

    let lang = convert_lang(language);

    let _manifest = manifest(lang.as_ref(), matches, preamble)?;

    #[cfg(feature = "languageserver")]
    {
        ls::server(io::stdin(), io::stdout())?;
        Ok(())
    }

    #[cfg(not(feature = "languageserver"))]
    {
        Err("languageserver feature is not enabled".into())
    }
}
