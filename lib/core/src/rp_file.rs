//! File declarations

use errors::Result;
use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use std::collections::VecDeque;
use {Diagnostics, Flavor, RpDecl, Span, Translate, Translator, Version};

/// Information about an enabled feature.
#[derive(Debug, Clone, Serialize)]
pub struct EnabledFeature {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
#[serde(
    bound = "F: Serialize, F::Field: Serialize, F::Endpoint: Serialize, F::Package: Serialize, \
             F::Name: Serialize, F::EnumType: Serialize"
)]
pub struct RpFile<F: 'static>
where
    F: Flavor,
{
    /// File-level comments.
    pub comment: Vec<String>,
    /// The schema version in use.
    pub version: Version,
    /// Features enabled and where they are enabled.
    pub features: LinkedHashMap<&'static str, EnabledFeature>,
    /// All nested declarations.
    pub decls: Vec<RpDecl<F>>,
    /// references to the local idents of the declarations.
    pub decl_idents: LinkedHashMap<String, usize>,
}

/// Iterator over all declarations in a file.
#[allow(linkedlist)]
pub struct ForEachDecl<'a, F: 'static>
where
    F: Flavor,
{
    queue: VecDeque<&'a RpDecl<F>>,
}

impl<'a, F: 'static> Iterator for ForEachDecl<'a, F>
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

impl<F: 'static> RpFile<F>
where
    F: Flavor,
{
    /// Iterate over all declarations in file.
    pub fn for_each_decl(&self) -> ForEachDecl<F> {
        let mut queue = VecDeque::new();
        queue.extend(self.decls.iter());
        ForEachDecl { queue }
    }

    /// Lookup a single declaration from its path.
    pub fn decl_by_path<'a, 's>(
        &'a self,
        mut path: impl Iterator<Item = &'s str>,
    ) -> Option<&'a RpDecl<F>> {
        let first = match path.next() {
            Some(first) => first,
            None => return None,
        };

        let mut decl = match self.decl_idents.get(first) {
            Some(index) => self.decls.get(*index),
            None => None,
        };

        for step in path {
            let next = match decl.as_ref() {
                Some(decl) => decl.decl_by_ident(step),
                None => return None,
            };

            decl = next;
        }

        decl
    }
}

impl<F: 'static, T> Translate<T> for RpFile<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = RpFile<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpFile<T::Target>> {
        Ok(RpFile {
            comment: self.comment,
            version: self.version,
            features: self.features,
            decls: self.decls.translate(diag, translator)?,
            decl_idents: self.decl_idents,
        })
    }
}
