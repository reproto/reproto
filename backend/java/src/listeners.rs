//! Plugin infrastructure for Java Backend.

use backend::errors::*;
use core::{RpEnumBody, RpInterfaceBody, RpServiceBody};
use genco::Cons;
use genco::java::{Class, Enum, Interface, Method};
use java_backend::JavaBackend;
use java_options::JavaOptions;

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

pub trait Listeners {
    listeners_vec_default!(configure, JavaOptions);
    listeners_vec_default!(class_added, ClassAdded);
    listeners_vec_default!(tuple_added, TupleAdded);
    listeners_vec_default!(enum_added, EnumAdded);
    listeners_vec_default!(interface_added, InterfaceAdded);
    listeners_vec_default!(service_added, ServiceAdded);
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    listeners_vec!(configure, JavaOptions);
    listeners_vec!(class_added, ClassAdded);
    listeners_vec!(tuple_added, TupleAdded);
    listeners_vec!(enum_added, EnumAdded);
    listeners_vec!(interface_added, InterfaceAdded);
    listeners_vec!(service_added, ServiceAdded);
}
