use super::*;
use genco::Cons;
use genco::java::{Class, Enum, Interface, Method};

pub trait Listeners {
    fn configure(&self, _: &mut JavaOptions) -> Result<()> {
        Ok(())
    }

    fn class_added<'a>(&self, _names: &[Cons<'a>], _spec: &mut Class<'a>) -> Result<()> {
        Ok(())
    }

    fn tuple_added<'a>(&self, _spec: &mut Class<'a>) -> Result<()> {
        Ok(())
    }

    fn enum_added<'el, 'a, 'b, 'c>(
        &self,
        _body: &'el RpEnumBody,
        _spec: &mut Enum<'a>,
        _from_value: &mut Method<'b>,
        _to_value: &mut Method<'c>,
    ) -> Result<()> {
        Ok(())
    }

    fn interface_added<'a>(
        &self,
        _interface: &'a RpInterfaceBody,
        _spec: &mut Interface<'a>,
    ) -> Result<()> {
        Ok(())
    }

    fn sub_type_added<'a>(
        &self,
        _interface: &'a RpInterfaceBody,
        _sub_type: &'a RpSubType,
        _spec: &mut Class<'a>,
    ) -> Result<()> {
        Ok(())
    }
}

/// A vector of listeners is a valid listener.
impl Listeners for Vec<Box<Listeners>> {
    fn configure(&self, options: &mut JavaOptions) -> Result<()> {
        for i in self {
            i.configure(options)?;
        }

        Ok(())
    }

    fn class_added<'a>(&self, names: &[Cons<'a>], spec: &mut Class<'a>) -> Result<()> {
        for i in self {
            i.class_added(names, spec)?;
        }

        Ok(())
    }

    fn tuple_added<'a>(&self, spec: &mut Class<'a>) -> Result<()> {
        for i in self {
            i.tuple_added(spec)?;
        }

        Ok(())
    }

    fn enum_added<'el, 'a, 'b, 'c>(
        &self,
        body: &'el RpEnumBody,
        spec: &mut Enum<'a>,
        from_value: &mut Method<'b>,
        to_value: &mut Method<'c>,
    ) -> Result<()> {
        for i in self {
            i.enum_added(body, spec, from_value, to_value)?;
        }

        Ok(())
    }

    fn interface_added<'a>(
        &self,
        interface: &'a RpInterfaceBody,
        spec: &mut Interface<'a>,
    ) -> Result<()> {
        for i in self {
            i.interface_added(interface, spec)?;
        }

        Ok(())
    }

    fn sub_type_added<'a>(
        &self,
        interface: &'a RpInterfaceBody,
        sub_type: &'a RpSubType,
        spec: &mut Class<'a>,
    ) -> Result<()> {
        for i in self {
            i.sub_type_added(interface, sub_type, spec)?;
        }

        Ok(())
    }
}
