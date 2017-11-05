use super::imports::*;
use core::Manifest;
use std::path::{Path, PathBuf};

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

fn setup_compiler_options(manifest: &Manifest, matches: &ArgMatches) -> Result<CompilerOptions> {
    // output path as specified in manifest.
    let manifest_out = manifest.output.as_ref().map(PathBuf::as_path);

    // final output path
    let out_path = matches
        .value_of("out")
        .map(Path::new)
        .or(manifest_out)
        .ok_or("--out <dir>, or `output` key in manifest is required")?;

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

    let (manifest, env) = setup_env(matches)?;
    let options = setup_options(matches)?;
    let compiler_options = setup_compiler_options(&manifest, matches)?;

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
