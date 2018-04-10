//! Code generator for the given path.

use Options;
use compiler::Compiler;
use core::Handle;
use core::errors::Result;
use flavored::{RpEnumBody, RpInterfaceBody, RpServiceBody};
use genco::java::{Class, Enum, Interface, Method};
use std::rc::Rc;

/// Generate helper implementations for codegen traits.
macro_rules! codegen {
    ($type:tt, $e:ty) => {
        impl<T> $type for Rc<T>
        where
            T: $type,
        {
            fn generate(&self, e: $e) -> Result<()> {
                self.as_ref().generate(e)
            }
        }
    };
}

pub struct GetterAdded<'a, 'el: 'a> {
    pub name: &'el str,
    pub getter: &'a mut Method<'el>,
}

pub struct ClassAdded<'a, 'el: 'a> {
    pub names: &'a [&'el str],
    pub spec: &'a mut Class<'el>,
    pub interface: Option<&'a RpInterfaceBody>,
}

pub struct TupleAdded<'a, 'el: 'a> {
    pub spec: &'a mut Class<'el>,
}

pub struct EnumAdded<'a, 'el: 'a> {
    pub body: &'el RpEnumBody,
    pub spec: &'a mut Enum<'el>,
    pub from_value: &'a mut Method<'el>,
    pub to_value: &'a mut Method<'el>,
}

pub struct InterfaceAdded<'a, 'el: 'a> {
    pub body: &'el RpInterfaceBody,
    pub spec: &'a mut Interface<'el>,
}

pub struct ServiceAdded<'a, 'el: 'a> {
    pub body: &'el RpServiceBody,
    pub spec: &'a mut Interface<'el>,
}

pub struct Configure<'a> {
    pub options: &'a mut Options,
}

pub trait Codegen {
    /// Build the given piece of code in the given handle.
    fn generate(&self, compiler: &Compiler, handle: &Handle) -> Result<()>;
}

/// Generate service-based code.
pub trait ServiceCodegen {
    fn generate(&self, e: ServiceAdded) -> Result<()>;
}

codegen!(ServiceCodegen, ServiceAdded);

/// Generate code for getters.
pub trait GetterCodegen {
    fn generate(&self, e: GetterAdded) -> Result<()>;
}

codegen!(GetterCodegen, GetterAdded);

/// Generate class-based code.
pub trait ClassCodegen {
    fn generate(&self, e: ClassAdded) -> Result<()>;
}

codegen!(ClassCodegen, ClassAdded);

/// Generate tuple-based code.
pub trait TupleCodegen {
    fn generate(&self, e: TupleAdded) -> Result<()>;
}

codegen!(TupleCodegen, TupleAdded);

/// Generate interface-based code.
pub trait InterfaceCodegen {
    fn generate(&self, e: InterfaceAdded) -> Result<()>;
}

codegen!(InterfaceCodegen, InterfaceAdded);

/// Generate enum-based code.
pub trait EnumCodegen {
    fn generate(&self, e: EnumAdded) -> Result<()>;
}

codegen!(EnumCodegen, EnumAdded);
