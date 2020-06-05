use clap::ArgMatches;
use core::errors::{Error, Result, ResultExt};
use core::{
    CoreFlavor, Flavor, Reporter, Resolved, ResolvedByPrefix, Resolver, RpChannel, RpFile,
    RpPackage, RpPackageFormat, RpRequiredPackage, RpVersionedPackage, Source, SourceDiagnostics,
    Version,
};
use manifest::{self, Lang, Language, Manifest, Publish};
use repository::Repository;
use semck;
use std::fmt;
use std::fs::File;
use std::path::Path;
use trans::Session;

/// Load the manifest based on commandline arguments.
pub fn load_manifest<'a>(m: &ArgMatches<'a>) -> Result<Manifest> {
    let mut manifest = manifest::Manifest::default();

    let path = m
        .value_of("manifest-path")
        .map::<Result<&Path>, _>(|p| Ok(Path::new(p)))
        .unwrap_or_else(|| Ok(Path::new(env::MANIFEST_NAME)))?;

    manifest.path = Some(path.to_owned());

    if let Some(lang) = m.value_of("lang") {
        let lang =
            Language::parse(lang).ok_or_else(|| format!("not a valid language: {}", lang))?;
        manifest.lang = Some(env::convert_lang(lang));
    }

    if path.is_file() {
        log::debug!("reading manifest: {}", path.display());
        manifest.from_yaml(File::open(&path)?, env::convert_lang)?;
    }

    matches_to_manifest(&mut manifest, m)?;
    return Ok(manifest);

    /// Populate manifest with overrides and extensions from the command line.
    fn matches_to_manifest(manifest: &mut Manifest, m: &ArgMatches) -> Result<()> {
        manifest.paths.extend(
            m.values_of("path")
                .into_iter()
                .flat_map(|it| it)
                .map(Path::new)
                .map(ToOwned::to_owned),
        );

        if let Some(files) = m.values_of("file") {
            for file in files {
                match file {
                    // read from stdin
                    "-" => manifest.stdin = true,
                    // read from file
                    file => {
                        let file = manifest::File::from_path(Path::new(file));
                        manifest.files.get_or_insert_with(Vec::new).push(file);
                    }
                }
            }
        }

        // TODO: we want to be able to load modules, even when we don't have a path.
        if let Some(path) = manifest.path.as_ref() {
            if let Some(lang) = manifest.lang.as_ref() {
                for module in m.values_of("module").into_iter().flat_map(|it| it) {
                    let module = lang.string_spec(path, module)?;
                    manifest.modules.get_or_insert_with(Vec::new).push(module);
                }
            }
        }

        for package in m.values_of("package").into_iter().flat_map(|it| it) {
            let parsed = RpRequiredPackage::parse(package);

            let parsed =
                parsed.chain_err(|| format!("failed to parse --package argument: {}", package))?;

            manifest.packages.get_or_insert_with(Vec::new).push(parsed);
        }

        if let Some(package_prefix) = m.value_of("package-prefix").map(RpPackage::parse) {
            manifest.package_prefix = Some(package_prefix);
        }

        if let Some(id_converter) = m.value_of("id-converter") {
            manifest.id_converter = Some(id_converter.to_string());
        }

        // override output path
        if let Some(out) = m.value_of("out").map(Path::new) {
            manifest.output = Some(out.to_owned());
        }

        matches_to_repository(&mut manifest.repository, m)?;
        return Ok(());
    }

    /// Populate the repository structure from CLI arguments.
    ///
    /// CLI arguments take precedence.
    fn matches_to_repository(repository: &mut manifest::Repository, m: &ArgMatches) -> Result<()> {
        repository.no_repository = repository.no_repository || m.is_present("no-repository");

        if let Some(objects) = m.value_of("objects").map(ToOwned::to_owned) {
            repository.objects = Some(objects);
        }

        if let Some(index) = m.value_of("index").map(ToOwned::to_owned) {
            repository.index = Some(index);
        }

        Ok(())
    }
}

pub fn session<'a>(
    lang: Box<dyn Lang>,
    manifest: &Manifest,
    reporter: &'a mut dyn Reporter,
    resolver: &'a mut dyn Resolver,
) -> Result<Session<'a, CoreFlavor>> {
    session_with_hook(lang, manifest, reporter, resolver, |_| Ok(()))
}

/// Setup session.
pub fn session_with_hook<'a, F: 'static>(
    lang: Box<dyn Lang>,
    manifest: &Manifest,
    reporter: &'a mut dyn Reporter,
    resolver: &'a mut dyn Resolver,
    path_hook: F,
) -> Result<Session<'a, CoreFlavor>>
where
    F: Fn(&Path) -> Result<()>,
{
    let package_prefix = manifest.package_prefix.clone();

    let mut session = lang
        .into_session(package_prefix, reporter, resolver)?
        .with_path_hook(path_hook);

    let mut errors: Vec<Error> = Vec::new();

    let mut stdin = manifest.stdin;

    if manifest.is_build_empty() {
        stdin = true;
    }

    for s in manifest.resolve(session.resolver)? {
        let manifest::Source { package, source } = s;

        match session.import_source(source.clone(), Some(package.clone())) {
            Err(e) => errors.push(e.into()),
            _ => {}
        }
    }

    if stdin {
        log::debug!("Reading file to build from stdin");

        let source = Source::stdin();

        if let Err(e) = session.import_source(source, None) {
            errors.push(e.into());
        }
    }

    if let Err(e) = session.verify() {
        errors.push(e.into());
    }

    if !errors.is_empty() {
        return Err(Error::new("error when building").with_suppressed(errors));
    }

    Ok(session)
}

/// Argument match.
pub struct Match {
    pub version: Version,
    pub source: Source,
    pub package: RpPackage,
}

/// Setup matches from a publish manifest.
pub fn publish_matches<'a, I>(
    resolver: &mut dyn Resolver,
    version_override: Option<&Version>,
    publish: I,
) -> Result<Vec<Match>>
where
    I: IntoIterator<Item = &'a Publish>,
{
    let mut results = Vec::new();

    for publish in publish.into_iter() {
        let resolved = resolver.resolve_by_prefix(&publish.package)?;

        if resolved.is_empty() {
            return Err(format!("no matching packages found for: {}", publish.package).into());
        }

        for ResolvedByPrefix { package, source } in resolved {
            // only publish un-versioned.
            if package.version.is_some() {
                log::warn!(
                    "not publishing versioned package `{}` from {}",
                    package,
                    source
                );
                continue;
            }

            let version = version_override.unwrap_or(&publish.version).clone();
            results.push(Match {
                version,
                source,
                package: package.package,
            });
        }
    }

    Ok(results)
}

pub fn matches<'a, I>(
    resolver: &mut dyn Resolver,
    version_override: Option<&Version>,
    packages: I,
) -> Result<Vec<Match>>
where
    I: IntoIterator<Item = &'a RpRequiredPackage>,
{
    let mut results = Vec::new();

    for package in packages.into_iter() {
        let mut resolved = resolver.resolve(package)?.into_iter();

        let first = resolved
            .next()
            .ok_or_else(|| format!("no package matching: {}", package))?;

        if let Some(next) = resolved.next() {
            log::warn!("matched: {}", first);
            log::warn!("    and: {}", next);

            while let Some(next) = resolved.next() {
                log::warn!("    and: {}", next);
            }

            return Err("more than one matching package found".into());
        }

        let Resolved { version, source } = first;

        let version = version_override
            .cloned()
            .or(version)
            .ok_or_else(|| format!("no version for package: {}", package.package))?;

        results.push(Match {
            version,
            source,
            package: package.package.clone(),
        });
    }

    Ok(results)
}

pub fn semck_check(
    errors: &mut Vec<Error>,
    repository: &mut Repository,
    session: &mut Session<CoreFlavor>,
    version_to: &Version,
    source_to: &Source,
    package_to: &RpVersionedPackage,
    file_to: &RpFile<CoreFlavor>,
) -> Result<()> {
    // perform semck verification
    if let Some(d) = repository
        .all(&package_to.package)?
        .into_iter()
        .filter(|d| d.version <= *version_to && !d.version.is_prerelease())
        .last()
    {
        log::debug!("Checking semantics of {} -> {}", d.version, version_to);

        let current = repository
            .get_object(&d)?
            .ok_or_else(|| format!("No object found for deployment: {:?}", d))?;

        let name = RpPackageFormat(&package_to.package, Some(&d.version)).to_string();
        let current = current.with_name(name);

        let package_from =
            RpVersionedPackage::new(package_to.package.clone(), Some(d.version.clone()));
        let file_from = session.load_source(current.clone(), &package_from)?;

        let violations = semck::check((&d.version, &file_from), (&version_to, file_to))?;

        if !violations.is_empty() {
            errors.push(Error::new(format!(
                "Encountered {} semck violation(s)",
                violations.len()
            )));

            let mut diag = SourceDiagnostics::new();

            for v in violations {
                handle_violation(&mut diag, &current, source_to, v)?;
            }

            session.reporter.source_diagnostics(diag);
        }
    }

    return Ok(());

    fn handle_violation(
        diag: &mut SourceDiagnostics,
        current: &Source,
        source_to: &Source,
        violation: semck::Violation,
    ) -> Result<()> {
        use semck::Violation::*;

        match violation {
            DeclRemoved(c, reg) => {
                diag.err(
                    current,
                    reg,
                    format!("{}: declaration removed", c.describe()),
                );
            }
            DeclAdded(c, reg) => {
                diag.err(
                    source_to,
                    reg,
                    format!("{}: declaration added", c.describe()),
                );
            }
            RemoveField(c, field) => {
                diag.err(current, field, format!("{}: field removed", c.describe()));
            }
            RemoveVariant(c, field) => {
                diag.err(current, field, format!("{}: variant removed", c.describe()));
            }
            AddField(c, field) => {
                diag.err(source_to, field, format!("{}: field added", c.describe()));
            }
            AddVariant(c, field) => {
                diag.err(source_to, field, format!("{}: variant added", c.describe()));
            }
            FieldTypeChange(c, from_type, from, to_type, to) => {
                diag.err(
                    source_to,
                    to,
                    format!("{}: type changed to `{}`", c.describe(), to_type),
                );
                diag.info(current, from, format!("from `{}`", from_type));
            }
            FieldNameChange(c, from_name, from, to_name, to) => {
                diag.err(
                    source_to,
                    to,
                    format!("{}: name changed to `{}`", c.describe(), to_name),
                );
                diag.info(current, from, format!("from `{}`", from_name));
            }
            VariantOrdinalChange(c, from_ordinal, from, to_ordinal, to) => {
                diag.err(
                    source_to,
                    to,
                    format!("{}: ordinal changed to `{}`", c.describe(), to_ordinal),
                );
                diag.info(current, from, format!("from `{}`", from_ordinal));
            }
            FieldRequiredChange(c, from, to) => {
                diag.err(
                    source_to,
                    to,
                    format!("{}: field changed to be required`", c.describe(),),
                );
                diag.info(current, from, "from here");
            }
            AddRequiredField(c, field) => {
                diag.err(
                    source_to,
                    field,
                    format!("{}: required field added", c.describe()),
                );
            }
            FieldModifierChange(c, from, to) => {
                diag.err(
                    source_to,
                    to,
                    format!("{}: field modifier changed", c.describe()),
                );
                diag.info(current, from, "from here");
            }
            AddEndpoint(c, span) => {
                diag.err(source_to, span, format!("{}: endpoint added", c.describe()));
            }
            RemoveEndpoint(c, span) => {
                diag.err(current, span, format!("{}: endpoint removed", c.describe()));
            }
            EndpointRequestChange(c, from_channel, from, to_channel, to) => {
                diag.err(
                    source_to,
                    to,
                    format!(
                        "{}: request type changed to `{}`",
                        c.describe(),
                        FmtChannel(to_channel.as_ref())
                    ),
                );
                diag.info(
                    current,
                    from,
                    format!("from `{}`", FmtChannel(from_channel.as_ref())),
                );
            }
            EndpointResponseChange(c, from_channel, from, to_channel, to) => {
                diag.err(
                    source_to,
                    to,
                    format!(
                        "{}: response type changed to `{}`",
                        c.describe(),
                        FmtChannel(to_channel.as_ref())
                    ),
                );
                diag.err(
                    current,
                    from,
                    format!("from `{}`", FmtChannel(from_channel.as_ref())),
                );
            }
        }

        return Ok(());

        /// Helper struct to display information on channels.
        struct FmtChannel<'a, F: 'static>(Option<&'a RpChannel<F>>)
        where
            F: Flavor;

        impl<'a, F: 'static> fmt::Display for FmtChannel<'a, F>
        where
            F: Flavor,
        {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                match self.0 {
                    None => write!(fmt, "*empty*"),
                    Some(channel) => write!(fmt, "{}", channel),
                }
            }
        }
    }
}

/// Setup a basic session falling back to `NoLang` unless one is specified.
pub fn simple_config<'a>(
    manifest: &Manifest,
    reporter: &'a mut dyn Reporter,
    resolver: &'a mut dyn Resolver,
) -> Result<Session<'a, CoreFlavor>> {
    let lang = manifest.lang_or_nolang();
    let session = session(lang, &manifest, reporter, resolver)?;
    Ok(session)
}
