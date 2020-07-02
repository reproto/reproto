//! Propagates scope-specific information to `into_model` transformations.

use crate::features::{Feature, Features};
use core::errors::Error;
use core::{
    CoreFlavor, Diagnostics, Import, RpName, RpRequiredPackage, RpVersionedPackage, Span, Spanned,
    Version,
};
use naming::Naming;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct Scope<I> {
    /// This is the version that will be considered if a file does not declare any version.
    undeclared_version: Rc<Version>,
    /// This will be set if a file declares a version in use.
    pub declared_version: Option<Version>,
    /// A set of activated features.
    pub activated_features: HashMap<&'static str, Span>,
    /// The complete set of available features.
    pub features: Rc<Features>,
    /// Current package of the file being processed.
    package: RpVersionedPackage,
    /// Language keywords to avoid.
    keywords: Rc<HashMap<String, String>>,
    field_ident_naming: Option<Box<dyn Naming>>,
    endpoint_ident_naming: Option<Box<dyn Naming>>,
    import: I,
    pub endpoint_naming: Option<Box<dyn Naming>>,
    pub field_naming: Option<Box<dyn Naming>>,
    pub prefixes: HashMap<String, RpVersionedPackage>,
    /// Path of the current scope.
    path: Vec<String>,
}

impl<I> Scope<I> {
    pub fn new(
        undeclared_version: Rc<Version>,
        features: Rc<Features>,
        package: RpVersionedPackage,
        keywords: Rc<HashMap<String, String>>,
        field_ident_naming: Option<Box<dyn Naming>>,
        endpoint_ident_naming: Option<Box<dyn Naming>>,
        import: I,
    ) -> Scope<I> {
        Self {
            undeclared_version,
            features,
            declared_version: None,
            activated_features: HashMap::new(),
            package,
            keywords,
            field_ident_naming,
            endpoint_ident_naming,
            import,
            endpoint_naming: None,
            field_naming: None,
            prefixes: HashMap::new(),
            path: vec![],
        }
    }

    /// Current version being processed.
    pub fn version(&self) -> &Version {
        self.declared_version
            .as_ref()
            .unwrap_or(&self.undeclared_version)
    }

    /// Create a new child scope.
    pub fn push<S: AsRef<str>>(&mut self, name: S) {
        self.path.push(name.as_ref().to_string());
    }

    /// Pop the last name component.
    pub fn pop(&mut self) {
        self.path.pop();
    }

    /// Lookup what package a given prefix belongs to.
    pub fn lookup_prefix(&self, prefix: &str) -> Option<RpVersionedPackage> {
        self.prefixes.get(prefix).map(Clone::clone)
    }

    /// Get the package that this scope belongs to.
    pub fn package(&self) -> RpVersionedPackage {
        self.package.clone()
    }

    /// Get the current path as a name.
    pub fn as_name(&self, span: Span) -> Spanned<RpName<CoreFlavor>> {
        Spanned::new(
            RpName {
                prefix: None,
                package: self.package(),
                path: self.path.clone(),
            },
            span,
        )
    }

    /// Access active endpoint naming.
    pub fn endpoint_naming(&self) -> Option<&dyn Naming> {
        self.endpoint_naming.as_ref().map(AsRef::as_ref)
    }

    /// Access active field naming.
    pub fn field_naming(&self) -> Option<&dyn Naming> {
        self.field_naming.as_ref().map(AsRef::as_ref)
    }

    /// Access field identifier naming.
    pub fn field_ident_naming(&self) -> Option<&dyn Naming> {
        self.field_ident_naming.as_ref().map(AsRef::as_ref)
    }

    /// Access endpoint identifier naming.
    pub fn endpoint_ident_naming(&self) -> Option<&dyn Naming> {
        self.endpoint_ident_naming.as_ref().map(AsRef::as_ref)
    }

    /// Lookup if the given identifier matches a language keyword.
    pub fn keyword(&self, identifier: &str) -> Option<&str> {
        self.keywords.get(identifier).map(|s| s.as_str())
    }

    /// Check if a given feature is active and return its specification if it is.
    pub fn feature(&self, name: &'static str) -> Option<&Feature> {
        let feature = match self.features.get(name) {
            Some(feature) => feature,
            None => return None,
        };

        if self.activated_features.contains_key(name) {
            return Some(&feature);
        }

        match feature.stable_at.as_ref() {
            Some(version) => {
                if self.version() >= version {
                    return None;
                }
            }
            None => return None,
        }

        return Some(&feature);
    }

    /// Report an error caused by a feature activation.
    ///
    /// This includes diagnostics for why the feature is active.
    pub fn feature_err(
        &self,
        diag: &mut Diagnostics,
        feature: &Feature,
        span: Span,
        thing: impl fmt::Display,
    ) {
        if let Some(activate_span) = self.activated_features.get(feature.name) {
            diag.err(
                span,
                format!("{} since feature `{}` is active", thing, feature.name),
            );
            diag.info(activate_span, "feature activated here");
            return;
        }

        if let Some(version) = feature.stable_at.as_ref() {
            if self.version() >= version {
                diag.err(
                    span,
                    format!(
                        "{} feature `{}` active since {}",
                        thing, feature.name, version
                    ),
                );
                return;
            }
        }

        diag.err(
            span,
            format!(
                "{} since feature `{}` active through magic",
                thing, feature.name
            ),
        )
    }
}

impl<I> Scope<I>
where
    I: Import,
{
    pub fn import(
        &mut self,
        package: &RpRequiredPackage,
    ) -> Result<Option<RpVersionedPackage>, Error> {
        self.import.import(package)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::Features;
    use crate::scope::Scope;
    use core::{RpPackage, RpVersionedPackage, Version};
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    pub fn test_scope() {
        let package = RpVersionedPackage::new(RpPackage::empty(), None);
        let keywords = Rc::new(HashMap::new());

        let version = Rc::new(Version::new(0, 0, 0));
        let features = Rc::new(Features::new().expect("failed to build features"));
        let mut s = Scope::new(version, features, package, keywords, None, None, ());

        s.push("foo");
        s.push("bar");

        assert_eq!(vec!["foo".to_owned(), "bar".to_owned()], s.path);
    }
}
