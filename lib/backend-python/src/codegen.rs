use crate::flavored::RpServiceBody;
use core::errors::Result;
use genco::lang::Python;
use genco::Tokens;
use std::rc::Rc;

pub struct ServiceAdded<'a> {
    pub body: &'a RpServiceBody,
    pub type_body: &'a mut Tokens<Python>,
}

/// Generate service-based code.
pub trait ServiceCodegen {
    fn generate(&self, e: ServiceAdded<'_>) -> Result<()>;
}

impl<T> ServiceCodegen for Rc<T>
where
    T: ServiceCodegen,
{
    fn generate(&self, e: ServiceAdded<'_>) -> Result<()> {
        self.as_ref().generate(e)
    }
}
