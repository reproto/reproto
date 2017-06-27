use super::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("update").about("Update local repository");
    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let repository = setup_repository(matches)?.ok_or_else(|| "could not setup repository")?;
    repository.update()?;
    Ok(())
}
