use core::errors::Result;
use core::{
    Flavor, Handle, RelativePath, RelativePathBuf, RpDecl, RpEnumBody, RpInterfaceBody, RpName,
    RpPackage, RpServiceBody, RpTupleBody, RpTypeBody, Spanned,
};
use std::cmp;
use std::collections::{btree_map, BTreeMap};
use std::fmt;

pub trait Name<F>: Clone + fmt::Debug + cmp::Eq
where
    F: Flavor,
{
    /// Access the package for the name.
    fn package(&self) -> &F::Package;
}

/// Implementation for default name translation.
impl<F> Name<F> for Spanned<RpName<F>>
where
    F: Flavor,
{
    fn package(&self) -> &F::Package {
        &self.package
    }
}

pub trait PackageProcessor<'el, F: 'static, N: 'static>
where
    Self: 'el + Sized,
    F: Flavor<Name = N, Package = RpPackage>,
    N: Name<F>,
{
    type Out: Default;
    type DeclIter: Iterator<Item = &'el RpDecl<F>>;

    /// Access the extension for processing.
    fn ext(&self) -> &str;

    /// Iterate over all existing declarations.
    fn decl_iter(&self) -> Self::DeclIter;

    fn handle(&self) -> &'el dyn Handle;

    fn default_process(&self, _: &mut Self::Out, name: &'el F::Name) -> Result<()> {
        log::warn!("not supported: {:?}", name);
        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody<F>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody<F>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody<F>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody<F>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el RpServiceBody<F>) -> Result<()> {
        self.default_process(out, &body.name)
    }

    fn populate_files(&self) -> Result<BTreeMap<F::Package, Self::Out>> {
        self.do_populate_files(|_, _, _| Ok(()))
    }

    fn do_populate_files<C>(&self, mut callback: C) -> Result<BTreeMap<F::Package, Self::Out>>
    where
        C: FnMut(&'el RpDecl<F>, bool, &mut Self::Out) -> Result<()>,
    {
        use self::RpDecl::*;

        let mut files = BTreeMap::new();

        // Process all types discovered so far.
        for decl in self.decl_iter() {
            let (new, out) = match files.entry(decl.name().package().clone()) {
                btree_map::Entry::Vacant(e) => (true, e.insert(Default::default())),
                btree_map::Entry::Occupied(e) => (false, e.into_mut()),
            };

            callback(decl, new, out)?;

            match *decl {
                Interface(ref b) => self.process_interface(out, b)?,
                Type(ref b) => self.process_type(out, b)?,
                Tuple(ref b) => self.process_tuple(out, b)?,
                Enum(ref b) => self.process_enum(out, b)?,
                Service(ref b) => self.process_service(out, b)?,
            }
        }

        Ok(files)
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<RelativePathBuf> {
        let mut full_path = package
            .parts()
            .fold(RelativePathBuf::new(), |a, b| a.join(b));
        full_path.set_extension(self.ext());
        Ok(full_path)
    }

    fn setup_module_path(&self, package: &RpPackage) -> Result<RelativePathBuf> {
        let handle = self.handle();
        let full_path = self.resolve_full_path(package)?;

        {
            let parent = full_path.parent().unwrap_or(RelativePath::new("."));

            if !handle.is_dir(&parent) {
                log::debug!("+dir: {}", parent);
                handle.create_dir_all(&parent)?;
            }
        }

        Ok(full_path)
    }
}
