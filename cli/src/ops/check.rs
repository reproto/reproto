use crate::utils::{load_manifest, matches, publish_matches, semck_check, simple_config, Match};
use clap::{App, Arg, ArgMatches, SubCommand};
use reproto_core::errors::{Error, Result};
use reproto_core::{Reporter, RpRequiredPackage, RpVersionedPackage, Version};

pub fn options<'a>() -> App<'a> {
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

pub fn entry(reporter: &mut dyn Reporter, m: &ArgMatches) -> Result<()> {
    let manifest = load_manifest(m)?;
    let mut resolver = env::resolver(&manifest)?;
    let mut session = simple_config(&manifest, reporter, resolver.as_mut())?;

    let mut manifest_resolver =
        env::path_resolver(&manifest)?.ok_or_else(|| "could not setup manifest resolver")?;

    let version_override = if let Ok(Some(version)) = m.try_get_one::<String>("version") {
        Some(Version::parse(version).map_err(|e| format!("bad version: {}: {}", version, e))?)
    } else {
        None
    };

    let packages: Vec<RpRequiredPackage> = m
        .try_get_many::<String>("package")
        .ok()
        .flatten()
        .into_iter()
        .flatten()
        .map(|p| RpRequiredPackage::parse(p))
        .collect::<Result<_>>()?;

    let mut results = Vec::new();

    results.extend(publish_matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        manifest.publish.as_ref().iter().flat_map(|p| p.iter()),
    )?);

    results.extend(matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        &packages,
    )?);

    let mut repository = env::repository(&manifest)?;

    let mut errors = Vec::new();

    for m in results {
        let Match {
            ref version,
            ref source,
            ref package,
        } = m;

        let package = RpVersionedPackage::new(package.clone(), Some(version.clone()));
        let file = session.load_source(source.clone(), &package)?;

        semck_check(
            &mut errors,
            &mut repository,
            &mut session,
            version,
            source,
            &package,
            &file,
        )?;
    }

    if errors.len() > 0 {
        return Err(Error::new("Error when checking").with_suppressed(errors));
    }

    Ok(())
}
