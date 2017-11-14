//! Propagates scope-specific information to `into_model` transformations.

use super::naming::Naming;
use core::{RpName, RpPackage, RpVersionedPackage};
use std::collections::HashMap;
use std::rc::Rc;

struct Root {
    pub package_prefix: Option<RpPackage>,
    pub package: RpVersionedPackage,
    pub prefixes: HashMap<String, RpVersionedPackage>,
    pub endpoint_naming: Option<Box<Naming>>,
    pub field_naming: Option<Box<Naming>>,
}

/// Model of a scope.
enum Inner {
    Root { root: Rc<Root> },
    Child {
        root: Rc<Root>,
        name: String,
        parent: Rc<Inner>,
    },
}

pub struct Scope {
    inner: Rc<Inner>,
}

impl Scope {
    pub fn new(
        package_prefix: Option<RpPackage>,
        package: RpVersionedPackage,
        prefixes: HashMap<String, RpVersionedPackage>,
        endpoint_naming: Option<Box<Naming>>,
        field_naming: Option<Box<Naming>>,
    ) -> Scope {
        let root = Rc::new(Root {
            package_prefix: package_prefix,
            package: package,
            prefixes: prefixes,
            endpoint_naming: endpoint_naming,
            field_naming: field_naming,
        });

        let inner_root = Inner::Root { root: root.clone() };

        Scope { inner: Rc::new(inner_root) }
    }

    /// Apply global package prefix.
    fn package_prefix(
        &self,
        package_prefix: &Option<RpPackage>,
        package: &RpVersionedPackage,
    ) -> RpVersionedPackage {
        package_prefix
            .as_ref()
            .map(|prefix| prefix.join_versioned(package))
            .unwrap_or_else(|| package.clone())
    }

    pub fn child<S: AsRef<str>>(&self, name: S) -> Scope {
        let root = match *self.inner {
            Inner::Root { ref root, .. } |
            Inner::Child { ref root, .. } => root.clone(),
        };

        Scope {
            inner: Rc::new(Inner::Child {
                root: root,
                name: name.as_ref().to_owned(),
                parent: self.inner.clone(),
            }),
        }
    }

    pub fn lookup_prefix(&self, prefix: &String) -> Option<&RpVersionedPackage> {
        match *self.inner {
            Inner::Root { ref root, .. } |
            Inner::Child { ref root, .. } => root.prefixes.get(prefix),
        }
    }

    pub fn package(&self) -> RpVersionedPackage {
        match *self.inner {
            Inner::Root { ref root, .. } |
            Inner::Child { ref root, .. } => {
                self.package_prefix(&root.package_prefix, &root.package)
            }
        }
    }

    pub fn walk(&self) -> ScopeWalker {
        ScopeWalker { current: self.inner.clone() }
    }

    pub fn as_name(&self) -> RpName {
        let mut parts: Vec<_> = self.walk().collect();
        parts.reverse();

        RpName {
            prefix: None,
            package: self.package(),
            parts: parts,
        }
    }

    pub fn endpoint_naming(&self) -> Option<&Naming> {
        match *self.inner {
            Inner::Root { ref root, .. } |
            Inner::Child { ref root, .. } => root.endpoint_naming.as_ref().map(AsRef::as_ref),
        }
    }

    pub fn field_naming(&self) -> Option<&Naming> {
        match *self.inner {
            Inner::Root { ref root, .. } |
            Inner::Child { ref root, .. } => root.field_naming.as_ref().map(AsRef::as_ref),
        }
    }
}

pub struct ScopeWalker {
    current: Rc<Inner>,
}

impl Iterator for ScopeWalker {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let (next, current) = match *self.current {
            Inner::Root { .. } => {
                return None;
            }
            Inner::Child {
                ref name,
                ref parent,
                ..
            } => (Some(name.to_owned()), parent.clone()),
        };

        self.current = current;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;
    use core::{RpPackage, RpVersionedPackage};
    use std::collections::HashMap;

    #[test]
    pub fn test_scope() {
        let package = RpVersionedPackage::new(RpPackage::empty(), None);
        let prefixes = HashMap::new();
        let s = Scope::new(None, package, prefixes, None);

        let s2 = s.child("foo");
        let s3 = s2.child("bar");

        let parts: Vec<_> = s3.walk().collect();

        assert_eq!(vec!["bar".to_owned(), "foo".to_owned()], parts);
    }
}
