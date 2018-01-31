//! Plugin infrastructure for Java Backend.

use core::{RpEnumBody, RpInterfaceBody, RpServiceBody};
use genco::{Cons, Java};
use genco::java::{Argument, Class, Enum, Interface, Method};
use java_backend::JavaBackend;
use java_options::JavaOptions;
use std::rc::Rc;
use utils::Utils;

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
    pub backend: &'a JavaBackend,
    pub body: &'el RpServiceBody,
    pub extra: &'a [EndpointExtra<'el>],
    pub spec: &'a mut Interface<'el>,
}

pub struct Configure<'a> {
    pub options: &'a mut JavaOptions,
    pub utils: &'a Rc<Utils>,
}

pub trait Listeners {
    fn configure<'a>(&self, _: Configure<'a>) {}
}
