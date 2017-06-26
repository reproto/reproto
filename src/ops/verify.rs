use super::*;

fn base<'a, 'b>(name: &str) -> App<'a, 'b> {
    let out = SubCommand::with_name(name);
    let out = compiler_base(out).about("Verify .reproto specifications");
    out
}

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("verify").about("Verify .reproto specifications");
    let out = out.subcommand(doc::verify_options(base("doc")));
    let out = out.subcommand(java::verify_options(base("java")));
    let out = out.subcommand(js::verify_options(base("js")));
    let out = out.subcommand(python::verify_options(base("python")));
    let out = out.subcommand(rust::verify_options(base("rust")));
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    let env = setup_env(matches)?;
    let options = setup_options(matches)?;

    let result = match name {
        "doc" => doc::verify(env, options, matches),
        "java" => java::verify(env, options, matches),
        "js" => js::verify(env, options, matches),
        "json" => json::verify(env, options, matches),
        "python" => python::verify(env, options, matches),
        "rust" => rust::verify(env, options, matches),
        _ => unreachable!("bad subcommand"),
    };

    Ok(result?)
}
