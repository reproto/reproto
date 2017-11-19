use super::imports::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    doc::shared_options(SubCommand::with_name("doc").about("Generate documentation"))
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let manifest = setup_manifest(matches)?;
    let env = setup_environment(&manifest)?;
    let options = setup_options(&manifest)?;
    let compiler_options = setup_compiler_options(&manifest, matches)?;

    let out = compiler_options.out_path.clone();

    doc::compile(env, options, compiler_options, matches)?;

    info!("Wrote documentation in: {}", out.display());
    Ok(())
}
