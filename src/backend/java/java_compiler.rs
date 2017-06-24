use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use super::*;

pub struct JavaCompiler<'a> {
    pub out_path: PathBuf,
    pub backend: &'a JavaBackend,
}

impl<'a> JavaCompiler<'a> {
    pub fn compile(&self) -> Result<()> {
        self.process_files(|full_path, type_id, decl| {
            debug!("+class: {}", full_path.display());

            if let Some(out_dir) = full_path.parent() {
                if !out_dir.is_dir() {
                    debug!("+dir: {}", out_dir.display());
                    fs::create_dir_all(&out_dir)?;
                }
            }

            let file_spec = self.backend.build_file_spec(type_id, decl)?;

            let mut out = String::new();
            file_spec.format(&mut out)?;

            let mut f = File::create(full_path)?;
            f.write_all(&out.into_bytes())?;
            f.flush()?;

            Ok(())
        })
    }

    fn process_files<F>(&self, mut consumer: F) -> Result<()>
        where F: FnMut(PathBuf, &RpTypeId, &RpDecl) -> Result<()>
    {
        let root_dir = &self.out_path;

        // Process all types discovered so far.
        for (ref type_id, ref decl) in &self.backend.env.decls {
            let out_dir = self.backend
                .java_package(&type_id.package)
                .parts
                .iter()
                .fold(root_dir.clone(), |current, next| current.join(next));

            let full_path = out_dir.join(format!("{}.java", decl.name()));
            consumer(full_path, type_id, decl)?;
        }

        Ok(())
    }
}
