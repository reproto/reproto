use anyhow::{format_err, Context as _, Result};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct SuiteYaml {
    /// Only enable suite for these specified languages.
    #[serde(default)]
    enabled: HashSet<String>,
}

impl SuiteYaml {
    /// Load spec from the given path.
    pub fn load_path(path: &Path) -> Result<Self> {
        let f = fs::File::open(path)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

#[derive(Debug, Clone)]
pub struct Suite {
    pub(crate) dir: PathBuf,
    /// The path to the proto directory.
    pub(crate) proto_path: PathBuf,
    /// Name of suite.
    pub(crate) name: String,
    /// JSON files in suite.
    pub(crate) json: Vec<PathBuf>,
    /// proto files in suite.
    pub(crate) proto: Vec<PathBuf>,
    /// Packages to build.
    pub(crate) packages: Vec<String>,
    /// Languages suite is enabled for.
    pub(crate) enabled: Option<HashSet<String>>,
}

impl Suite {
    /// Check if suite supports the given language.
    ///
    /// Unless a set of enabled languages is configured, all languages are supported.
    pub(crate) fn supports_language(&self, lang: &str) -> bool {
        match &self.enabled {
            Some(enabled) => enabled.contains(lang),
            None => true,
        }
    }
}

pub fn discover_suites(root: &Path) -> Result<Vec<Suite>> {
    let suites = root.join("suites");

    if !suites.is_dir() {
        return Ok(vec![]);
    }

    let mut out = Vec::new();

    for entry in fs::read_dir(suites)? {
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| format_err!("failed to convert directory to string"))?;

        let path = entry.path();

        let mut json = Vec::new();
        let mut proto = Vec::new();
        let mut enabled = None;

        let suite_yaml_path = path.join("suite.yaml");
        let input_path = path.join("input");
        let proto_path = path.join("proto");

        if suite_yaml_path.is_file() {
            log::trace!("loading: {}", suite_yaml_path.display());

            let spec = SuiteYaml::load_path(&suite_yaml_path)
                .with_context(|| format_err!("failed to load: {}", suite_yaml_path.display()))?;

            enabled
                .get_or_insert_with(HashSet::new)
                .extend(spec.enabled);
        }

        if input_path.is_dir() {
            read_with_extension(&input_path, "json", &mut json)?;
        }

        if proto_path.is_dir() {
            read_with_extension(&proto_path, "proto", &mut proto)?;
        }

        let packages = vec![String::from("test")];

        out.push(Suite {
            dir: entry.path(),
            proto_path,
            name,
            json,
            proto,
            packages,
            enabled,
        });
    }

    Ok(out)
}

/// Read path with the given `expected` extension and populate out.
fn read_with_extension(dir: &Path, expected: &str, out: &mut Vec<PathBuf>) -> Result<()> {
    for e in fs::read_dir(dir)? {
        let p = e?.path();

        let has_ext = p.extension().map(|ext| ext == expected).unwrap_or(false);

        if p.is_file() && has_ext {
            out.push(p);
        }
    }

    Ok(())
}
