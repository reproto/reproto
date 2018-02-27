//! build command

use build_spec::{convert_lang, environment, manifest, manifest_preamble};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::Context;
use core::errors::Result;
use manifest::Language;
use std::rc::Rc;

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

pub fn entry(ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;

    let language = preamble
        .language
        .as_ref()
        .cloned()
        .or_else(|| matches.value_of("lang").and_then(Language::parse))
        .ok_or_else(|| "no language specified either through manifest or cli (--lang)")?;

    let lang = convert_lang(language);

    let manifest = manifest(lang.as_ref(), matches, preamble)?;
    let env = environment(lang.as_ref(), ctx.clone(), &manifest)?;

    lang.compile(ctx, env, manifest)?;
    Ok(())
}
