use crate::utils::{load_manifest, matches, publish_matches, semck_check, simple_config, Match};
use clap::{App, Arg, ArgMatches, SubCommand};
use reproto_core::errors::{Error, Result};
use reproto_core::{Diagnostics, Reporter, RpRequiredPackage, RpVersionedPackage, Version};

pub fn options<'a>() -> App<'a> {
    let out = SubCommand::with_name("publish").about("Publish specifications");

    let out = out.arg(
        Arg::with_name("force")
            .long("force")
            .help("Force a publish, even if it already exists"),
    );

    let out = out.arg(
        Arg::with_name("pretend")
            .long("pretend")
            .help("Pretend to publish"),
    );

    let out = out.arg(
        Arg::with_name("no-semck")
            .long("no-semck")
            .help("Disable Semantic Checks"),
    );

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
        Some(
            Version::parse(version)
                .map_err(|e| format!("not a valid version: {}: {}", version, e))?,
        )
    } else {
        None
    };

    let mut results = Vec::new();

    results.extend(publish_matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        manifest.publish.as_ref().iter().flat_map(|p| p.iter()),
    )?);

    // packages to publish from the commandline
    let packages: Vec<RpRequiredPackage> = m
        .try_get_many::<String>("package")
        .ok()
        .flatten()
        .into_iter()
        .flatten()
        .map(|p| RpRequiredPackage::parse(p))
        .collect::<Result<_>>()?;

    results.extend(matches(
        manifest_resolver.as_mut(),
        version_override.as_ref(),
        &packages,
    )?);

    let force = m.try_contains_id("force").unwrap_or_default();
    let pretend = m.try_contains_id("pretend").unwrap_or_default();
    let no_semck = m.try_contains_id("no-semck").unwrap_or_default();

    let mut repository = env::repository(&manifest)?;

    // errors that would prevent publishing
    let mut semck_errors = Vec::new();
    let mut feature_ok = true;

    for m in &results {
        let Match {
            ref version,
            ref source,
            ref package,
        } = m;

        let package = RpVersionedPackage::new(package.clone(), Some(version.clone()));
        let file = session.load_source(source.clone(), &package)?;

        semck_check(
            &mut semck_errors,
            &mut repository,
            &mut session,
            version,
            source,
            &package,
            &file,
        )?;

        if !file.features.is_empty() {
            let mut diag = Diagnostics::new(source.clone());

            for enabled in file.features.values() {
                diag.err(enabled.span, "cannot publish schema with enabled features");
            }

            session.reporter.diagnostics(diag);
            feature_ok = false;
        }
    }

    let semck_ok = !no_semck || semck_errors.is_empty();

    if !semck_ok && !feature_ok {
        if !no_semck {
            semck_errors.push("Hint: Use `--no-semck` to disable semantic checking".into());
        }

        return Err(Error::new("Validation errors").with_suppressed(semck_errors));
    }

    for m in results {
        let Match {
            version,
            source,
            package,
        } = m;

        if pretend {
            log::info!(
                "(pretend) publishing: {}@{} (from {})",
                package,
                version,
                source
            );
        } else {
            log::info!("publishing: {}@{} (from {})", package, version, source);
            repository.publish(&source, &package, &version, force)?;
        }
    }

    Ok(())
}
