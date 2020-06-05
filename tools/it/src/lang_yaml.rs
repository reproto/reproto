use crate::languages as l;
use anyhow::Result;
use relative_path::RelativePathBuf;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct InstanceBody {
    #[serde(default)]
    args: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct LangYaml {
    name: String,
    lang: String,
    #[serde(default)]
    output: RelativePathBuf,
    #[serde(default)]
    package_prefix: Option<String>,
    #[serde(default)]
    no_project: bool,
    #[serde(default)]
    instances: HashMap<String, InstanceBody>,
    #[serde(flatten)]
    instance_extra: InstanceBody,
}

impl LangYaml {
    /// Open the given path.
    pub(crate) fn load_path(path: &Path) -> Result<Self> {
        let lang_yaml = fs::File::open(path)?;
        Ok(serde_yaml::from_reader(lang_yaml)?)
    }

    pub(crate) fn into_lang(self) -> l::Language {
        let mut instances = Vec::new();

        for (name, mut instance) in self.instances {
            let mut args = self.instance_extra.args.clone();
            args.append(&mut instance.args);
            instances.push(l::Instance { name, args });
        }

        if instances.is_empty() {
            instances.push(l::Instance {
                name: String::from("default"),
                args: self.instance_extra.args.clone(),
            });
        }

        l::Language {
            name: self.name,
            lang: self.lang,
            output: self.output,
            package_prefix: self.package_prefix,
            no_project: self.no_project,
            instances,
        }
    }
}
