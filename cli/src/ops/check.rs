use super::imports::*;
use core::Version;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("check").about("Check specifications");

    let out = out.arg(
        Arg::with_name("version")
            .long("version")
            .takes_value(true)
            .help("Override published version with argument"),
    );

    let out = out.arg(Arg::with_name("package").multiple(true));

    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let manifest = setup_manifest(matches)?;
    let mut env = setup_env(&manifest)?;

    let mut manifest_resolver = setup_path_resolver(&manifest)?.ok_or_else(|| {
        "could not setup manifest resolver"
    })?;

    let version_override = if let Some(version) = matches.value_of("version") {
        Some(Version::parse(version).map_err(|e| {
            format!("not a valid version: {}: {}", version, e)
        })?)
    } else {
        None
    };

    let packages: Vec<RpRequiredPackage> = matches
        .values_of("package")
        .into_iter()
        .flat_map(|it| it)
        .map(|p| RpRequiredPackage::parse(p).map_err(Into::into))
        .collect::<Result<_>>()?;

    let mut results = Vec::new();

    results.extend(setup_publish_matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        &manifest.packages,
    )?);

    results.extend(setup_matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        &packages,
    )?);

    let mut repository = setup_repository(&manifest)?;

    let mut errors = Vec::new();

    for m in results {
        semck_check(&mut errors, &mut repository, &mut env, &m)?;
    }

    if errors.len() > 0 {
        return Err(ErrorKind::Errors(errors).into());
    }

    Ok(())
}
