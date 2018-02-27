//! Helper utilities for processors.

use core::{Loc, RpChannel, RpEndpoint, RpPackage, RpVersionedPackage};
use core::errors::Result;

pub trait Processor {
    /// Build the java package of a given package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    fn csharp_package(&self, pkg: &RpVersionedPackage) -> RpPackage {
        pkg.as_package(|version| {
            format!("_{}", version).replace(".", "_").replace("-", "_")
        })
    }

    /// Extract endpoint request.
    ///
    /// Errors if more than one argument is present.
    fn endpoint_request<'a>(
        &self,
        endpoint: &'a RpEndpoint,
    ) -> Result<Option<(&'a str, &'a RpChannel)>> {
        let mut it = endpoint.arguments.values();

        if let Some(&(ref name, ref first)) = it.next() {
            let channel = Loc::value(first);
            return Ok(Some((name.as_str(), channel)));
        }

        Ok(None)
    }
}
