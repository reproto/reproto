use build_spec::{Match, manifest_preamble, semck_check, setup_environment, setup_matches,
                 setup_path_resolver, setup_publish_matches, setup_repository};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::{Context, RpRequiredPackage, Version};
use errors::*;
use manifest::{Lang, Manifest};
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("publish").about("Publish specifications");

    let out = out.arg(Arg::with_name("force").long("force").help(
        "Force a publish, \
         even if it already \
         exists",
    ));

    let out = out.arg(Arg::with_name("pretend").long("pretend").help(
        "Pretend to \
         publish",
    ));

    let out = out.arg(Arg::with_name("no-semck").long("no-semck").help(
        "Disable Semantic \
         Checks",
    ));

    let out = out.arg(
        Arg::with_name("version")
            .long("version")
            .takes_value(true)
            .help("Override published version with argument"),
    );

    let out = out.arg(Arg::with_name("package").multiple(true));

    out
}

pub fn entry(ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;
    return do_manifest_use!(ctx, matches, preamble, inner);

    fn inner<L>(ctx: Rc<Context>, matches: &ArgMatches, manifest: Manifest<L>) -> Result<()>
    where
        L: Lang,
    {
        let mut env = setup_environment(ctx, &manifest)?;

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

        let mut results = Vec::new();

        results.extend(setup_publish_matches(
            manifest_resolver.as_mut(),
            version_override.as_ref(),
            &manifest.publish,
        )?);

        // packages to publish from the commandline
        let packages: Vec<RpRequiredPackage> = matches
            .values_of("package")
            .into_iter()
            .flat_map(|it| it)
            .map(|p| RpRequiredPackage::parse(p).map_err(Into::into))
            .collect::<Result<_>>()?;

        results.extend(setup_matches(
            manifest_resolver.as_mut(),
            version_override.as_ref(),
            &packages,
        )?);

        let force = matches.is_present("force");
        let pretend = matches.is_present("pretend");
        let no_semck = matches.is_present("no-semck");

        let mut repository = setup_repository(&manifest)?;

        // errors that would prevent publishing
        let mut semck_errors = Vec::new();

        for m in &results {
            semck_check(&mut semck_errors, &mut repository, &mut env, &m)?;
        }

        if semck_errors.len() > 0 {
            if !no_semck {
                semck_errors.push("Hint: Use `--no-semck` to disable semantic checking".into());
                return Err(ErrorKind::Errors(semck_errors).into());
            } else {
                warn!("{} errors skipped (--no-semck)", semck_errors.len());
            }
        }

        for m in results {
            let Match(version, object, package) = m;

            if pretend {
                info!(
                    "(pretend) publishing: {}@{} (from {})",
                    package,
                    version,
                    object
                );
            } else {
                info!("publishing: {}@{} (from {})", package, version, object);
                repository.publish(&object, &package, &version, force)?;
            }
        }

        Ok(())
    }
}
