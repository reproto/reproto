use core::RpServiceBody;
use core::errors::Result;
use genco::{Python, Tokens};
use python_backend::PythonBackend;
use std::rc::Rc;

pub struct EndpointExtra<'el> {
    pub name: &'el str,
    pub response_ty: Option<(&'el str, Tokens<'el, Python<'el>>)>,
}

pub struct ServiceAdded<'a, 'el: 'a> {
    pub backend: &'a PythonBackend,
    pub body: &'el RpServiceBody,
    pub type_name: Rc<String>,
    pub type_body: &'a mut Tokens<'el, Python<'el>>,
    pub extra: &'a [EndpointExtra<'el>],
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
