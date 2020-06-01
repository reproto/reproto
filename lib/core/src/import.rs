//! Trait used to handle imports.

use crate::errors::Result;
use crate::{RpRequiredPackage, RpVersionedPackage};

pub trait Import {
    /// Perform the import.
    fn import(&mut self, _: &RpRequiredPackage) -> Result<Option<RpVersionedPackage>>;
}

/// no-op implementation.
impl Import for () {
    fn import(&mut self, _: &RpRequiredPackage) -> Result<Option<RpVersionedPackage>> {
        Ok(None)
    }
}
