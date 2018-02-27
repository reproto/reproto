//! Code generator for the given path.

use {Compiler, Options, Utils};
use core::{Handle, RpEnumBody, RpInterfaceBody, RpServiceBody};
use core::errors::Result;
use genco::{Cons, Java};
use genco::java::{Argument, Class, Enum, Interface, Method};
use std::rc::Rc;

pub struct ClassAdded<'a, 'el: 'a> {
    pub names: &'a [Cons<'el>],
    pub spec: &'a mut Class<'el>,
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

pub struct EndpointExtra<'el> {
    pub name: Cons<'el>,
    pub response_ty: Java<'el>,
    pub arguments: Vec<Argument<'el>>,
}

pub struct ServiceAdded<'a, 'el: 'a> {
    pub compiler: &'a Compiler,
    pub body: &'el RpServiceBody,
    pub extra: &'a [EndpointExtra<'el>],
    pub spec: &'a mut Interface<'el>,
}

pub struct Configure<'a> {
    pub options: &'a mut Options,
    pub utils: &'a Rc<Utils>,
}

pub trait Codegen {
    /// Build the given piece of code in the given handle.
    fn generate(&self, handle: &Handle) -> Result<()>;
}

impl<T> Codegen for Rc<T>
where
    T: Codegen,
{
    fn generate(&self, handle: &Handle) -> Result<()> {
        self.as_ref().generate(handle)
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
