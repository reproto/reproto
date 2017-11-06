use super::imports::*;
use manifest::Language;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("compile").about("Compile .reproto specifications");

    let out = out.arg(Arg::with_name("lang").long("lang").takes_value(true).help(
        "Language \
         to compile \
         for",
    ));

    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    use manifest::Language::*;

    let (manifest, env) = setup_env(matches)?;
    let options = setup_options(&manifest, matches)?;
    let compiler_options = setup_compiler_options(&manifest, matches)?;

    let language = manifest
        .language
        .or_else(|| matches.value_of("lang").and_then(Language::parse))
        .ok_or_else(|| {
            "no language specified either through manifest or cli (--language)"
        })?;

    let result = match language {
        Java => java::compile(env, options, compiler_options, matches),
        Js => js::compile(env, options, compiler_options, matches),
        Json => json::compile(env, options, compiler_options, matches),
        Python => python::compile(env, options, compiler_options, matches),
        Rust => rust::compile(env, options, compiler_options, matches),
    };

    Ok(result?)
}
