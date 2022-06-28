use manifest::{Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::{CoreFlavor, Handle, RelativePathBuf};
use std::any::Any;
use std::path::Path;
use trans::Session;

#[derive(Clone, Copy, Default, Debug)]
pub struct JsonLang;

impl Lang for JsonLang {
    manifest::lang_base!(JsonModule, compile);
}

#[derive(Debug)]
pub enum JsonModule {}

impl TryFromToml for JsonModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, _manifest: Manifest) -> Result<()> {
    let session = session.translate_default()?;

    let root = RelativePathBuf::from(".");

    for (package, file) in session.for_each_file() {
        let path = package
            .package
            .parts()
            .fold(root.clone(), |path, part| path.join(part));

        let parent = path
            .parent()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| root.clone());

        if !handle.is_dir(&parent) {
            log::debug!("+dir: {}", parent);
            handle.create_dir_all(&parent)?;
        }

        let path = if let Some(version) = package.version.as_ref() {
            let stem = path
                .file_stem()
                .ok_or_else(|| format!("Missing file stem: {}", path))?;

            let file_name = format!("{}-{}.json", stem, version);
            path.with_file_name(file_name)
        } else {
            path.with_extension("json")
        };

        log::debug!("+file: {}", path);
        writeln!(
            handle.create(&path)?,
            "{}",
            serde_json::to_string_pretty(file)?,
        )?;
    }

    Ok(())
}
