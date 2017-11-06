use manifest::Language;
use ops::imports::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("verify").about("Verify specifications");

    let out = out.arg(Arg::with_name("lang").long("lang").takes_value(true).help(
        "Language \
         to verify \
         for",
    ));

    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    use self::Language::*;

    let (manifest, env) = setup_env(matches)?;
    let options = setup_options(&manifest, matches)?;

    let language = manifest
        .language
        .or_else(|| matches.value_of("lang").and_then(Language::parse))
        .ok_or_else(|| {
            "no language specified either through manifest or cli (--lang)"
        })?;

    let result = match language {
        Java => java::verify(env, options, matches),
        Js => js::verify(env, options, matches),
        Json => json::verify(env, options, matches),
        Python => python::verify(env, options, matches),
        Rust => rust::verify(env, options, matches),
    };

    Ok(result?)
}
