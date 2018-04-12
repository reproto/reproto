use core::errors::Result;
use flavored::RpServiceBody;
use genco::{Python, Tokens};
use std::rc::Rc;

pub struct ServiceAdded<'a, 'el: 'a> {
    pub body: &'el RpServiceBody,
    pub type_body: &'a mut Tokens<'el, Python<'el>>,
}

/// Generate service-based code.
pub trait ServiceCodegen {
    fn generate(&self, e: ServiceAdded) -> Result<()>;
}

impl<T> ServiceCodegen for Rc<T>
where
    T: ServiceCodegen,
{
    fn generate(&self, e: ServiceAdded) -> Result<()> {
        self.as_ref().generate(e)
    }
}
