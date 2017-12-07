//! Plugin infrastructure for Java Backend.

use core::{RpEnumBody, RpInterfaceBody, RpServiceBody};
use genco::Cons;
use genco::java::{Class, Enum, Interface, Method};
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

pub struct ServiceAdded<'a, 'el: 'a> {
    pub backend: &'a JavaBackend,
    pub body: &'el RpServiceBody,
    pub endpoint_names: &'a [Cons<'el>],
    pub spec: &'a mut Interface<'el>,
}

pub struct Configure<'a> {
    pub options: &'a mut JavaOptions,
    pub utils: &'a Rc<Utils>,
}

pub trait Listeners {
    fn configure<'a>(&self, _: Configure<'a>) {}
}
