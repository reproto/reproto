//! Helper utilities for processors.

use reproto_core::errors::Result;
use reproto_core::flavored::*;

pub trait Processor {
    /// Extract endpoint request.
    ///
    /// Errors if more than one argument is present.
    fn endpoint_request<'a>(
        &self,
        endpoint: &'a RpEndpoint,
    ) -> Result<Option<(&'a str, &'a RpChannel)>> {
        let mut it = endpoint.arguments.iter();

        if let Some(arg) = it.next() {
            return Ok(Some((arg.ident.as_str(), &arg.channel)));
        }

        Ok(None)
    }
}
