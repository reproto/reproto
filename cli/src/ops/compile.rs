use super::imports::*;
use std::path::Path;

fn base<'a, 'b>(name: &str) -> App<'a, 'b> {
    let out = SubCommand::with_name(name);

    let out = compiler_base(out).about("Compile .reproto specifications");

    let out = out.arg(
        Arg::with_name("out")
            .long("out")
            .short("o")
            .takes_value(true)
            .help("Output directory."),
    );

    out
}

fn setup_compiler_options(matches: &ArgMatches) -> Result<CompilerOptions> {
    let out_path = matches.value_of("out").ok_or("--out <dir> is required")?;
    let out_path = Path::new(&out_path);

    Ok(CompilerOptions { out_path: out_path.to_owned() })
}

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("compile").about("Compile .reproto specifications");
    let out = out.subcommand(doc::compile_options(base("doc")));
    let out = out.subcommand(java::compile_options(base("java")));
    let out = out.subcommand(js::compile_options(base("js")));
    let out = out.subcommand(json::compile_options(base("json")));
    let out = out.subcommand(python::compile_options(base("python")));
    let out = out.subcommand(rust::compile_options(base("rust")));
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.ok_or_else(|| "no subcommand")?;

    let env = setup_env(matches)?;
    let options = setup_options(matches)?;
    let compiler_options = setup_compiler_options(matches)?;

    let result = match name {
        "doc" => doc::compile(env, options, compiler_options, matches),
        "java" => java::compile(env, options, compiler_options, matches),
        "js" => js::compile(env, options, compiler_options, matches),
        "json" => json::compile(env, options, compiler_options, matches),
        "python" => python::compile(env, options, compiler_options, matches),
        "rust" => rust::compile(env, options, compiler_options, matches),
        _ => unreachable!("bad subcommand"),
    };

    Ok(result?)
}
