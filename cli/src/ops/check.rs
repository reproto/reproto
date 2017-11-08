use super::imports::*;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("check").about("Check specifications");

    let out = out.arg(Arg::with_name("force").long("force").help(
        "Force a check, \
         even if it already \
         exists",
    ));

    let out = out.arg(Arg::with_name("package").multiple(true));

    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let manifest = setup_manifest(matches)?;
    let mut env = setup_env(&manifest)?;

    let mut manifest_resolver = setup_path_resolver(&manifest)?.ok_or_else(|| {
        "could not setup manifest resolver"
    })?;

    let packages: Vec<RpRequiredPackage> = matches
        .values_of("package")
        .into_iter()
        .flat_map(|it| it)
        .map(|p| RpRequiredPackage::parse(p).map_err(Into::into))
        .collect::<Result<_>>()?;

    let mut results = Vec::new();

    results.extend(setup_publish_matches(
        manifest_resolver.as_mut(),
        &manifest.publish,
    )?);

    results.extend(setup_matches(manifest_resolver.as_mut(), &packages)?);

    let force = matches.is_present("force");

    let mut repository = setup_repository(&manifest)?;

    let mut errors = Vec::new();

    for m in results {
        semck_check(&mut errors, &mut repository, &mut env, &m, force)?;
    }

    if errors.len() > 0 {
        return Err(ErrorKind::Errors(errors).into());
    }

    Ok(())
}
