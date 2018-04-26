//! Function to initialize a new project.

use core::errors::Result;
use core::{Handle, RelativePath};

const EXAMPLE: &'static [u8] = include_bytes!("example.reproto");

pub fn initialize(handle: &Handle) -> Result<()> {
    let mut path = RelativePath::new("proto");
    let manifest = RelativePath::new("reproto.toml");

    let mut with_output = true;
    let mut maven = false;
    let mut swift = false;

    let package = vec!["io", "reproto", "example"];

    // looks like a maven project
    if handle.is_file(RelativePath::new("pom.xml")) {
        with_output = false;
        maven = true;
    }

    // looks like a swift project
    if handle.is_file(RelativePath::new("Package.swift")) {
        with_output = false;
        swift = true;
    }

    if !handle.is_file(manifest) {
        info!("Writing Manifest: {}", manifest.display());

        let mut manifest = handle.create(manifest)?;

        if with_output {
            writeln!(manifest, "paths = [")?;
            writeln!(manifest, "  \"{}\"", path.display())?;
            writeln!(manifest, "]")?;
            writeln!(manifest, "output = \"target\"")?;
        }

        if maven {
            writeln!(manifest, "[presets.maven]")?;
            path = RelativePath::new("src/main/reproto");
        }

        if swift {
            writeln!(manifest, "[presets.swift]")?;
        }

        writeln!(manifest, "")?;
        writeln!(manifest, "[packages]")?;
        writeln!(
            manifest,
            "# File: {}/{}.reproto",
            path.display(),
            package.join("/")
        )?;
        writeln!(manifest, "\"{}\" = \"*\"", package.join("."))?;
    }

    let example = package
        .iter()
        .cloned()
        .fold(path.to_owned(), |p, part| p.join(part))
        .with_extension("reproto");

    if let Some(parent) = example.parent() {
        if !handle.is_dir(parent) {
            info!("Creating: {}", parent.display());
            handle.create_dir_all(parent)?;
        }
    }

    if !handle.is_file(&example) {
        info!("Writing: {}", example.display());
        let mut example = handle.create(&example)?;
        example.write_all(EXAMPLE)?;
    }

    Ok(())
}
