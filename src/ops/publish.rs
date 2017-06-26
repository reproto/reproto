use reproto_core::Version;
use std::fmt;
use std::path::PathBuf;
use super::*;

struct DisplayMatch<'a>(&'a (Option<Version>, PathBuf));

impl<'a> fmt::Display for DisplayMatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = &self.0;

        if let Some(ref version) = inner.0 {
            write!(f, "{}@{}", inner.1.display(), version)
        } else {
            write!(f, "{} (no version)", inner.1.display())
        }
    }
}

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("publish").about("Publish specifications");
    let out = path_base(out);
    let out = out.arg(Arg::with_name("package").multiple(true));

    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let repository = setup_repository(matches)?.ok_or_else(|| "could not setup repository")?;
    let resolver = setup_path_resolver(matches)?
        .ok_or_else(|| "could not setup path resolver")?;
    let packages = setup_packages(matches)?;

    for package in packages {
        let results = resolver.resolve(&package)?;

        let mut it = results.into_iter();
        let first = it.next()
            .ok_or_else(|| format!("no matching packages found for: {}", &package))?;

        if let Some(next) = it.next() {
            warn!("matched: {}", DisplayMatch(&first));
            warn!("    and: {}", DisplayMatch(&next));

            while let Some(next) = it.next() {
                warn!("    and: {}", DisplayMatch(&next));
            }

            return Err("more than one matching package found".into());
        }

        let (version, path) = first;
        let version =
            version.ok_or_else(|| format!("{}: package without a version", path.display()))?;

        info!("publishing: {}@{} (from {})",
              package.package,
              version,
              path.display());

        repository.publish(&path, &package.package, &version)?;
    }

    Ok(())
}
