use build_spec as build;
use clap::{App, Arg, ArgMatches, SubCommand};
use core::errors::*;
use core::{Reporter, RpRequiredPackage, RpVersionedPackage, Version};
use env;

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

pub fn entry(reporter: &mut Reporter, matches: &ArgMatches) -> Result<()> {
    let manifest = build::load_manifest(matches)?;
    let mut resolver = env::resolver(&manifest)?;
    let mut env = build::simple_config(&manifest, reporter, resolver.as_mut())?;

    let mut manifest_resolver =
        env::path_resolver(&manifest)?.ok_or_else(|| "could not setup manifest resolver")?;

    let version_override = if let Some(version) = matches.value_of("version") {
        Some(Version::parse(version).map_err(|e| format!("bad version: {}: {}", version, e))?)
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

    results.extend(build::publish_matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        manifest.publish.as_ref().iter().flat_map(|p| p.iter()),
    )?);

    results.extend(build::matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        &packages,
    )?);

    let mut repository = env::repository(&manifest)?;

    let mut errors = Vec::new();

    for m in results {
        let build::Match {
            ref version,
            ref source,
            ref package,
        } = m;

        let package = RpVersionedPackage::new(package.clone(), Some(version.clone()));
        let file = env.load_source(source.clone(), &package)?;

        build::semck_check(
            &mut errors,
            &mut repository,
            &mut env,
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
