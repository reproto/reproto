use core::{RpPackage, RpVersionedPackage, Version};

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
    /// This uses a relatively safe strategy for encoding the version number. This can be adjusted
    /// by overriding `version_package`.
    fn package(&self, package: &RpVersionedPackage) -> RpPackage {
        package.into_package(Self::version_package)
    }
}
