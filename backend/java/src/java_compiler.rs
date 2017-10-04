use super::*;
use genco::{IoFmt, Java, Tokens, WriteTokens};
use genco::java::Extra;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct JavaCompiler<'a> {
    pub out_path: PathBuf,
    pub backend: &'a JavaBackend,
}

impl<'a> JavaCompiler<'a> {
    pub fn compile(&self) -> Result<()> {
        let root_dir = &self.out_path;

        self.backend.env.for_each_toplevel_decl(|decl| {
            let package = self.backend.java_package(&decl.name().package);
            let package_name = package.parts.join(".");

            let out_dir = package.parts.iter().fold(
                root_dir.clone(),
                |current, next| current.join(next),
            );

            let full_path = out_dir.join(format!("{}.java", decl.local_name()));

            debug!("+class: {}", full_path.display());

            if let Some(out_dir) = full_path.parent() {
                if !out_dir.is_dir() {
                    debug!("+dir: {}", out_dir.display());
                    fs::create_dir_all(&out_dir)?;
                }
            }

            let mut file: Tokens<Java> = Tokens::new();
            let mut extra = Extra::default();
            extra.package(package_name);
            self.backend.process_decl(decl.as_ref(), 0usize, &mut file)?;

            let mut f = File::create(full_path)?;
            IoFmt(&mut f).write_file(file, &mut extra)?;
            f.flush()?;

            Ok(())
        })
    }
}
