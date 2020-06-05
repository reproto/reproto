use crate::lang_yaml::LangYaml;
use anyhow::{format_err, Context as _, Result};
use relative_path::RelativePathBuf;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language {
    pub name: String,
    pub lang: String,
    pub output: RelativePathBuf,
    pub package_prefix: Option<String>,
    pub instances: Vec<Instance>,
    pub no_project: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Instance {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Languages {
    pub languages: Vec<Language>,
}

pub fn discover_languages(path: &Path) -> Result<Languages> {
    let mut languages = Languages::default();

    let languages_path = path.join("languages");

    if !languages_path.is_dir() {
        log::warn!("languages directory missing: {}", languages_path.display());
        return Ok(languages);
    }

    for entry in fs::read_dir(&languages_path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let lang_yaml_path = path.join("lang.yaml");

        if !lang_yaml_path.is_file() {
            continue;
        }

        let lang_yaml = LangYaml::load_path(&lang_yaml_path)
            .with_context(|| format_err!("failed to load: {}", lang_yaml_path.display()))?;
        languages.languages.push(lang_yaml.into_lang());
    }

    Ok(languages)
}
