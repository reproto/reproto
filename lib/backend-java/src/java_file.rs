//! Helper component to build Java files.

use crate::core::errors::*;
use crate::core::{Handle, RelativePathBuf};
use crate::flavored::RpPackage;
use genco::java::Extra;
use genco::{IoFmt, Java, Tokens, WriteTokens};

pub struct JavaFile<'el, F> {
    package: RpPackage,
    class_name: &'el str,
    builder: F,
}

impl<'el, F> JavaFile<'el, F>
where
    F: FnOnce(&mut Tokens<'el, Java<'el>>) -> Result<()>,
{
    pub fn new(package: RpPackage, class_name: &'el str, builder: F) -> JavaFile<'el, F> {
        JavaFile {
            package: package,
            class_name: class_name,
            builder: builder,
        }
    }

    pub fn process(self, handle: &dyn Handle) -> Result<()> {
        let package = self.package.join(".");

        let path = self
            .package
            .parts()
            .cloned()
            .fold(RelativePathBuf::new(), |p, part| p.join(part));

        if !handle.is_dir(&path) {
            debug!("+dir: {}", path);
            handle.create_dir_all(&path)?;
        }

        let path = path.join(format!("{}.java", self.class_name));

        let mut file: Tokens<Java> = Tokens::new();
        (self.builder)(&mut file)?;

        let mut extra = Extra::default();
        extra.package(package);

        debug!("+class: {}", path);
        IoFmt(&mut handle.create(&path)?.as_mut()).write_file(file, &mut extra)?;

        Ok(())
    }
}
