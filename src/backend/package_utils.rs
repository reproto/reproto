use super::*;

pub trait PackageUtils {
    /// Identify if a character is unsafe for use in a package name.
    fn package_version_unsafe(c: char) -> bool {
        match c {
            '.' | '-' | '~' => true,
            _ => false,
        }
    }

    /// Default strategy for building the version package.
    fn version_package(input: &Version) -> String {
        format!("_{}", input).replace(Self::package_version_unsafe, "_")
    }

    /// Build the full package of a versioned package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    ///
    /// This uses a relatively safe strategy for encoding the version number. This can be adjusted
    /// by overriding `version_package`.
    fn package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.package_prefix()
            .clone()
            .map(|prefix| prefix.join_versioned(package))
            .unwrap_or_else(|| package.clone())
            .into_package(Self::version_package)
    }

    fn package_prefix(&self) -> &Option<RpPackage>;
}
