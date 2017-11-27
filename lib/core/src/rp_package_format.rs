//! Helper structure to format package information.

use super::{RpPackage, Version};
use std::fmt;

/// Helper structure to format package information.
pub struct RpPackageFormat<'a>(pub &'a RpPackage, pub Option<&'a Version>);

impl<'a> fmt::Display for RpPackageFormat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;

        if let Some(version) = self.1 {
            write!(f, "-{}", version)?;
        }

        Ok(())
    }
}
