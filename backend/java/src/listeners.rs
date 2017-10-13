//! Plugin infrastructure for Java Backend.

use backend::errors::*;
use core::{RpEnumBody, RpInterfaceBody};
use genco::Cons;
use genco::java::{Class, Enum, Interface, Method};
use java_options::JavaOptions;

macro_rules! impl_listeners {
    ($fn:ident, $event:ident) => {
        fn $fn(&self, e: &mut $event) -> Result<()> {
            for i in self {
                i.$fn(e)?;
            }

            Ok(())
        }
    }
}

macro_rules! impl_default {
    ($fn:ident, $event:ident) => {
        fn $fn(&self, _: &mut $event) -> Result<()> {
            Ok(())
        }
    }
}

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

pub trait Listeners {
    impl_default!(configure, JavaOptions);
    impl_default!(class_added, ClassAdded);
    impl_default!(tuple_added, TupleAdded);
    impl_default!(enum_added, EnumAdded);
    impl_default!(interface_added, InterfaceAdded);
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    impl_listeners!(configure, JavaOptions);
    impl_listeners!(class_added, ClassAdded);
    impl_listeners!(tuple_added, TupleAdded);
    impl_listeners!(enum_added, EnumAdded);
    impl_listeners!(interface_added, InterfaceAdded);
}
