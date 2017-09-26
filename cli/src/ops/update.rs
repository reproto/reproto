use ops::imports::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let repository = setup_repository(matches)?;
    repository.update()?;
    Ok(())
}
