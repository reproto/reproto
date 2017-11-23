//! Propagates scope-specific information to `into_model` transformations.

use super::naming::Naming;
use core::{Context, RpName, RpPackage, RpVersionedPackage};
use std::collections::HashMap;
use std::rc::Rc;

/// Root of the scope.
struct Root {
    ctx: Rc<Context>,
    package_prefix: Option<RpPackage>,
    package: RpVersionedPackage,
    prefixes: HashMap<String, RpVersionedPackage>,
    endpoint_naming: Option<Box<Naming>>,
    field_naming: Option<Box<Naming>>,
}

/// Model of a scope.
enum Inner {
    Root(Rc<Root>),
    Child {
        root: Rc<Root>,
        name: String,
        parent: Rc<Inner>,
    },
}

pub struct Scope(Rc<Inner>);

impl Scope {
    pub fn new(
        ctx: Rc<Context>,
        package_prefix: Option<RpPackage>,
        package: RpVersionedPackage,
        prefixes: HashMap<String, RpVersionedPackage>,
        endpoint_naming: Option<Box<Naming>>,
        field_naming: Option<Box<Naming>>,
    ) -> Scope {
        Scope(Rc::new(Inner::Root(Rc::new(Root {
            ctx: ctx,
            package_prefix: package_prefix,
            package: package,
            prefixes: prefixes,
            endpoint_naming: endpoint_naming,
            field_naming: field_naming,
        }))))
    }

    #[inline(always)]
    fn root(&self) -> &Rc<Root> {
        match *self.0 {
            Inner::Root(ref root) |
            Inner::Child { ref root, .. } => root,
        }
    }

    /// Walk the entire path of the scope.
    pub fn walk(&self) -> ScopeWalker {
        ScopeWalker { current: self.0.clone() }
    }

    /// Create a new child scope.
    pub fn child<S: AsRef<str>>(&self, name: S) -> Scope {
        Scope(Rc::new(Inner::Child {
            root: self.root().clone(),
            name: name.as_ref().to_owned(),
            parent: self.0.clone(),
        }))
    }

    /// Access the error context.
    pub fn ctx(&self) -> &Context {
        self.root().ctx.as_ref()
    }

    /// Lookup what package a given prefix belongs to.
    pub fn lookup_prefix(&self, prefix: &String) -> Option<&RpVersionedPackage> {
        self.root().prefixes.get(prefix)
    }

    /// Get the package that this scope belongs to.
    pub fn package(&self) -> RpVersionedPackage {
        let root = self.root();

        root.package_prefix
            .as_ref()
            .map(|prefix| prefix.join_versioned(&root.package))
            .unwrap_or_else(|| root.package.clone())
    }

    /// Get the current path as a name.
    pub fn as_name(&self) -> RpName {
        let mut parts: Vec<_> = self.walk().collect();
        parts.reverse();

        RpName {
            prefix: None,
            package: self.package(),
            parts: parts,
        }
    }

    /// Access active endpoint naming.
    pub fn endpoint_naming(&self) -> Option<&Naming> {
        self.root().endpoint_naming.as_ref().map(AsRef::as_ref)
    }

    /// Access active field naming.
    pub fn field_naming(&self) -> Option<&Naming> {
        self.root().field_naming.as_ref().map(AsRef::as_ref)
    }
}

/// Walker over all components of the scope.
pub struct ScopeWalker {
    current: Rc<Inner>,
}

impl Iterator for ScopeWalker {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let (next, current) = match *self.current {
            Inner::Root(_) => {
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
    use core::Context;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    pub fn test_scope() {
        let ctx = Rc::new(Context::new());
        let package = RpVersionedPackage::new(RpPackage::empty(), None);
        let prefixes = HashMap::new();
        let s = Scope::new(ctx, None, package, prefixes, None, None);

        let s2 = s.child("foo");
        let s3 = s2.child("bar");

        let parts: Vec<_> = s3.walk().collect();

        assert_eq!(vec!["bar".to_owned(), "foo".to_owned()], parts);
    }
}
