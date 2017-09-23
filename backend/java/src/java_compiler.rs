use super::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct JavaCompiler<'a> {
    pub out_path: PathBuf,
    pub backend: &'a JavaBackend,
}

impl<'a> JavaCompiler<'a> {
    pub fn compile(&self) -> Result<()> {
        self.process_files(|full_path, name, decl| {
            debug!("+class: {}", full_path.display());

            if let Some(out_dir) = full_path.parent() {
                if !out_dir.is_dir() {
                    debug!("+dir: {}", out_dir.display());
                    fs::create_dir_all(&out_dir)?;
                }
            }

            let file_spec = self.backend.build_file_spec(name, decl)?;

            let mut out = String::new();
            file_spec.format(&mut out)?;

            let mut f = File::create(full_path)?;
            f.write_all(&out.into_bytes())?;
            f.flush()?;

            Ok(())
        })
    }

    fn process_files<F>(&self, mut consumer: F) -> Result<()>
    where
        F: FnMut(PathBuf, &RpName, &RpDecl) -> Result<()>,
    {
        let root_dir = &self.out_path;

        self.backend.env.for_each_toplevel_decl(|name, decl| {
            let out_dir = self.backend.java_package(&name.package).parts.iter().fold(
                root_dir.clone(),
                |current, next| current.join(next),
            );

            let full_path = out_dir.join(format!("{}.java", decl.name()));
            consumer(full_path, name.as_ref(), decl.as_ref())
        })?;

        Ok(())
    }
}
