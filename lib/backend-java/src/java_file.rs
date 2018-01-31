//! Helper component to build Java files.

use backend::errors::*;
use genco::{IoFmt, Java, Tokens, WriteTokens};
use genco::java::Extra;
use std::fs::{self, File};
use std::path::Path;

pub struct JavaFile<'el, F> {
    package: &'el str,
    class_name: &'el str,
    builder: F,
}

impl<'el, F> JavaFile<'el, F>
where
    F: FnOnce(&mut Tokens<'el, Java<'el>>) -> Result<()>,
{
    pub fn new(package: &'el str, class_name: &'el str, builder: F) -> JavaFile<'el, F> {
        JavaFile {
            package: package,
            class_name: class_name,
            builder: builder,
        }
    }

    pub fn process(self, out_path: &Path) -> Result<()> {
        let parts = self.package.split('.').collect::<Vec<_>>();

        let client_path = parts
            .iter()
            .cloned()
            .fold(out_path.to_owned(), |p, part| p.join(part));

        if !client_path.is_dir() {
            debug!("+dir: {}", client_path.display());
            fs::create_dir_all(&client_path)?;
        }

        let client_path = client_path.join(format!("{}.java", self.class_name));

        let mut file: Tokens<Java> = Tokens::new();
        (self.builder)(&mut file)?;

        let mut extra = Extra::default();
        extra.package(self.package);

        debug!("+class: {}", client_path.display());
        IoFmt(&mut File::create(client_path)?).write_file(file, &mut extra)?;

        Ok(())
    }
}
