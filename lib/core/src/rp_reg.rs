//! Model for registered types.

use {Flavor, Loc, Pos, RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody, RpSubType,
     RpTupleBody, RpTypeBody, RpVariant};
use errors::Result;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum RpReg<F: 'static>
where
    F: Flavor,
{
    Type(Rc<Loc<RpTypeBody<F>>>),
    Tuple(Rc<Loc<RpTupleBody<F>>>),
    Interface(Rc<Loc<RpInterfaceBody<F>>>),
    SubType(Rc<Loc<RpInterfaceBody<F>>>, Rc<Loc<RpSubType<F>>>),
    Enum(Rc<Loc<RpEnumBody<F>>>),
    EnumVariant(Rc<Loc<RpEnumBody<F>>>, Rc<Loc<RpVariant>>),
    Service(Rc<Loc<RpServiceBody<F>>>),
}

impl<F: 'static> RpReg<F>
where
    F: Flavor,
{
    /// Get the name of the registered declaration.
    pub fn name(&self) -> &RpName {
        use self::RpReg::*;

        match *self {
            Type(ref target) => &target.name,
            Tuple(ref target) => &target.name,
            Service(ref target) => &target.name,
            Interface(ref target) => &target.name,
            Enum(ref target) => &target.name,
            SubType(_, ref target) => &target.name,
            EnumVariant(_, ref target) => &target.name,
        }
    }

    /// Get the location of the registered declaration.
    pub fn pos(&self) -> &Pos {
        use self::RpReg::*;

        match *self {
            Type(ref target) => Loc::pos(target),
            Tuple(ref target) => Loc::pos(target),
            Service(ref target) => Loc::pos(target),
            Interface(ref target) => Loc::pos(target),
            Enum(ref target) => Loc::pos(target),
            SubType(_, ref target) => Loc::pos(target),
            EnumVariant(_, ref target) => Loc::pos(target),
        }
    }

    pub fn fields<'a>(&'a self) -> Result<Box<Iterator<Item = &Loc<RpField<F>>> + 'a>> {
        use self::RpReg::*;

        let fields: Box<Iterator<Item = &Loc<RpField<F>>>> = match *self {
            Type(ref target) => Box::new(target.fields.iter()),
            Tuple(ref target) => Box::new(target.fields.iter()),
            Interface(ref target) => Box::new(target.fields.iter()),
            SubType(ref parent, ref target) => {
                Box::new(parent.fields.iter().chain(target.fields.iter()))
            }
            _ => return Err(format!("{}: type doesn't have fields", self).into()),
        };

        Ok(fields)
    }

    pub fn ident<PackageFn, InnerFn>(
        &self,
        name: &RpName,
        package_fn: PackageFn,
        inner_fn: InnerFn,
    ) -> String
    where
        PackageFn: Fn(Vec<&str>) -> String,
        InnerFn: Fn(Vec<&str>) -> String,
    {
        use self::RpReg::*;

        match *self {
            Type(_) | Interface(_) | Enum(_) | Tuple(_) | Service(_) => {
                let p = name.parts.iter().map(String::as_str).collect();
                package_fn(p)
            }
            SubType { .. } | EnumVariant { .. } => {
                let mut v: Vec<&str> = name.parts.iter().map(String::as_str).collect();
                let at = v.len().saturating_sub(2);
                let last = inner_fn(v.split_off(at));

                let mut parts = v.clone();
                parts.push(last.as_str());

                inner_fn(parts)
            }
        }
    }

    /// Get stringy kind of the registered type, if applicable.
    ///
    /// This returns the base kind as the first member of the tuple.
    /// Then the registered type as the second (if applicable).
    pub fn kind(&self) -> (&str, Option<&RpReg<F>>) {
        use self::RpReg::*;

        let result = match *self {
            Type(_) => "type",
            Interface(_) => "interface",
            Enum(_) => "enum",
            Tuple(_) => "tuple",
            Service(_) => "service",
            SubType(_, _) => return ("interface", Some(self)),
            EnumVariant(_, _) => return ("enum", Some(self)),
        };

        // simple case
        (result, None)
    }

    /// Check if registered type is an enum.
    pub fn is_enum(&self) -> bool {
        use self::RpReg::*;

        match *self {
            Enum(_) => true,
            _ => false,
        }
    }
}

impl<F: 'static> fmt::Display for RpReg<F>
where
    F: Flavor,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::RpReg::*;

        match *self {
            Type(ref body) => write!(fmt, "type {}", body.name),
            Interface(ref body) => write!(fmt, "interface {}", body.name),
            Enum(ref body) => write!(fmt, "enum {}", body.name),
            Tuple(ref body) => write!(fmt, "tuple {}", body.name),
            Service(ref body) => write!(fmt, "service {}", body.name),
            SubType(_, ref sub_type) => write!(fmt, "subtype {}", sub_type.name),
            EnumVariant(_, ref variant) => write!(fmt, "variant {}", variant.name),
        }
    }
}
