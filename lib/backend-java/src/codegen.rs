/// Code generator for the given path.
use core::errors::Result;
use listeners::{ClassAdded, EnumAdded, InterfaceAdded, ServiceAdded, TupleAdded};
use std::path::Path;
use std::rc::Rc;

pub trait Codegen {
    /// Build the given piece of code in the given path.
    fn generate(&self, out_path: &Path) -> Result<()>;
}

impl<T> Codegen for Rc<T>
where
    T: Codegen,
{
    fn generate(&self, out_path: &Path) -> Result<()> {
        self.as_ref().generate(out_path)
    }
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

/// Generate class-based code.
pub trait ClassCodegen {
    fn generate(&self, e: ClassAdded) -> Result<()>;
}

impl<T> ClassCodegen for Rc<T>
where
    T: ClassCodegen,
{
    fn generate(&self, e: ClassAdded) -> Result<()> {
        self.as_ref().generate(e)
    }
}

/// Generate tuple-based code.
pub trait TupleCodegen {
    fn generate(&self, e: TupleAdded) -> Result<()>;
}

impl<T> TupleCodegen for Rc<T>
where
    T: TupleCodegen,
{
    fn generate(&self, e: TupleAdded) -> Result<()> {
        self.as_ref().generate(e)
    }
}

/// Generate interface-based code.
pub trait InterfaceCodegen {
    fn generate(&self, e: InterfaceAdded) -> Result<()>;
}

impl<T> InterfaceCodegen for Rc<T>
where
    T: InterfaceCodegen,
{
    fn generate(&self, e: InterfaceAdded) -> Result<()> {
        self.as_ref().generate(e)
    }
}

/// Generate enum-based code.
pub trait EnumCodegen {
    fn generate(&self, e: EnumAdded) -> Result<()>;
}

impl<T> EnumCodegen for Rc<T>
where
    T: EnumCodegen,
{
    fn generate(&self, e: EnumAdded) -> Result<()> {
        self.as_ref().generate(e)
    }
}
