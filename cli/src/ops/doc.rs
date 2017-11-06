use super::imports::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    doc::shared_options(SubCommand::with_name("doc").about("Generate documentation"))
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let (manifest, env) = setup_env(matches)?;
    let options = setup_options(&manifest, matches)?;
    let compiler_options = setup_compiler_options(&manifest, matches)?;
    doc::compile(env, options, compiler_options, matches)?;
    Ok(())
}
