use core::errors::Result;
use core::{Flavor, RpDecl, RpFile, RpName, RpReg};
use linked_hash_map::LinkedHashMap;
use std::collections::{BTreeMap, LinkedList};

/// An environment that has been translated into a target environment.
pub struct Translated<F: 'static>
where
    F: Flavor,
{
    /// Registered types.
    decls: LinkedHashMap<RpName<F>, RpReg>,
    /// Files and associated declarations.
    files: BTreeMap<F::Package, RpFile<F>>,
}

impl<F: 'static> Translated<F>
where
    F: Flavor,
{
    pub fn new(
        decls: LinkedHashMap<RpName<F>, RpReg>,
        files: BTreeMap<F::Package, RpFile<F>>,
    ) -> Self {
        Self { decls, files }
    }

    /// Lookup the declaration matching the given name.
    ///
    /// Returns the registered reference, if present.
    pub fn lookup<'a>(&'a self, name: &RpName<F>) -> Result<&'a RpReg> {
        let key = name.clone().without_prefix();

        if let Some(registered) = self.decls.get(&key) {
            return Ok(registered);
        }

        return Err(format!("no such type: {}", name).into());
    }

    /// Lookup the declaration matching the given name.
    pub fn lookup_decl<'a>(&'a self, name: &RpName<F>) -> Result<&'a RpDecl<F>> {
        let file = match self.files.get(&name.package) {
            Some(file) => file,
            None => return Err(format!("no such type: {}", name).into()),
        };

        match file.decl_by_path(name.path.iter().map(|s| s.as_str())) {
            Some(decl) => Ok(decl),
            None => Err(format!("no such type: {}", name).into()),
        }
    }

    /// Iterate over all files.
    pub fn for_each_file<'a>(&'a self) -> impl Iterator<Item = (&'a F::Package, &'a RpFile<F>)> {
        self.files.iter()
    }

    /// Iterate over top level declarations of all registered objects.
    pub fn toplevel_decl_iter<'a>(&'a self) -> impl Iterator<Item = &'a RpDecl<F>> {
        let values = self
            .files
            .values()
            .flat_map(|f| f.decls.iter())
            .collect::<Vec<_>>();

        values.into_iter()
    }

    /// Walks the entire tree of declarations recursively of all registered objects.
    pub fn decl_iter(&self) -> DeclIter<F> {
        let mut queue = LinkedList::new();
        queue.extend(self.files.values().flat_map(|f| f.decls.iter()));
        DeclIter { queue: queue }
    }
}

/// Iterator over all declarations in a file.
pub struct DeclIter<'a, F: 'static>
where
    F: Flavor,
{
    queue: LinkedList<&'a RpDecl<F>>,
}

impl<'a, F: 'static> Iterator for DeclIter<'a, F>
where
    F: Flavor,
{
    type Item = &'a RpDecl<F>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(decl) = self.queue.pop_front() {
            self.queue.extend(decl.decls());
            Some(decl)
        } else {
            None
        }
    }
}
