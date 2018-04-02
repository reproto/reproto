use core::{AsPackage, Flavor, RpPackage, Version};

pub trait PackageUtils<F: 'static>
where
    F: Flavor,
{
    /// Identify if a character is unsafe for use in a package name.
    fn package_version_unsafe(&self, c: char) -> bool {
        match c {
            '.' | '-' | '~' => true,
            _ => false,
        }
    }

    /// Default strategy for building the version package.
    fn version_package(&self, input: &Version) -> String {
        format!("_{}", input).replace(|c| self.package_version_unsafe(c), "_")
    }

    /// Build the full package of a versioned package.
    ///
    /// This uses a relatively safe strategy for encoding the version number. This can be adjusted
    /// by overriding `version_package`.
    fn package(&self, package: &F::Package) -> RpPackage {
        let out = package.as_package(|version| self.version_package(version));

        if let Some(prefix) = self.package_prefix() {
            return prefix.clone().join_package(out);
        }

        out
    }

    /// Translate the package if required.
    fn translate(&self, package: &F::Package) -> RpPackage {
        package.as_package(|version| self.version_package(version))
    }

    /// Get the package prefix.
    fn package_prefix(&self) -> Option<&RpPackage>;
}
