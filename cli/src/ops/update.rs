use ops::imports::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(manifest: Manifest, _: &ArgMatches) -> Result<()> {
    let repository = setup_repository(&manifest.repository)?;
    repository.update()?;
    Ok(())
}
