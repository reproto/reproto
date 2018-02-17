//! build command

use build_spec::{manifest_compile, manifest_preamble};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::Context;
use core::errors::*;
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
    use manifest::Language::*;

    let preamble = manifest_preamble(matches)?;

    let language = preamble
        .language
        .as_ref()
        .cloned()
        .or_else(|| matches.value_of("lang").and_then(Language::parse))
        .ok_or_else(|| "no language specified either through manifest or cli (--lang)")?;

    match language {
        Java => manifest_compile::<::java::JavaLang, _>(ctx, matches, preamble, ::java::compile),
        Js => manifest_compile::<::js::JsLang, _>(ctx, matches, preamble, ::js::compile),
        Json => manifest_compile::<::json::JsonLang, _>(ctx, matches, preamble, ::json::compile),
        Python => {
            manifest_compile::<::python::PythonLang, _>(ctx, matches, preamble, ::python::compile)
        }
        Rust => manifest_compile::<::rust::RustLang, _>(ctx, matches, preamble, ::rust::compile),
    }?;

    Ok(())
}
