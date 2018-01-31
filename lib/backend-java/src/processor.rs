//! Helper utilities for processors.

use backend::errors::*;
use core::{RpChannel, RpEndpoint, RpPackage, RpVersionedPackage};

pub trait Processor {
    /// Build the java package of a given package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    fn java_package(&self, pkg: &RpVersionedPackage) -> RpPackage {
        pkg.as_package(|version| format!("_{}", version).replace(".", "_").replace("-", "_"))
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
            if let Some(&(ref other, _)) = it.next() {
                return Err(ErrorKind::Pos(
                    "more than one argument".to_string(),
                    other.pos().into(),
                ).into());
            }

            let channel = first.as_ref().take();
            return Ok(Some((name.as_str(), channel)));
        }

        Ok(None)
    }
}
