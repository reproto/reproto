use build_spec::{manifest_preamble, semck_check, setup_environment, setup_matches,
                 setup_path_resolver, setup_publish_matches, setup_repository};
use clap::{App, Arg, ArgMatches, SubCommand};
use core::{Context, RpRequiredPackage, Version};
use errors::*;
use manifest::{Lang, Manifest};
use std::rc::Rc;

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

pub fn entry(ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;
    return do_manifest_use!(ctx, matches, preamble, inner);

    fn inner<L>(ctx: Rc<Context>, matches: &ArgMatches, manifest: Manifest<L>) -> Result<()>
    where
        L: Lang,
    {
        let mut env = setup_environment(ctx.clone(), &manifest)?;

        let mut manifest_resolver =
            setup_path_resolver(&manifest)?.ok_or_else(|| "could not setup manifest resolver")?;

        let version_override = if let Some(version) = matches.value_of("version") {
            Some(Version::parse(version)
                .map_err(|e| format!("not a valid version: {}: {}", version, e))?)
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
            &manifest.publish,
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
}
