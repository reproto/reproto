use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug, Clone)]
pub enum RpRegistered {
    Type(Rc<RpTypeBody>),
    Interface(Rc<RpInterfaceBody>),
    Enum(Rc<RpEnumBody>),
    Tuple(Rc<RpTupleBody>),
    SubType {
        parent: Rc<RpInterfaceBody>,
        sub_type: Rc<RpSubType>,
    },
    EnumConstant {
        parent: Rc<RpEnumBody>,
        variant: Rc<RpEnumVariant>,
    },
    Service(Rc<RpServiceBody>),
}

impl RpRegistered {
    pub fn fields<'a>(&'a self) -> Result<Box<Iterator<Item = &Loc<RpField>> + 'a>> {
        use self::RpRegistered::*;

        let it: Box<Iterator<Item = &Loc<RpField>>> = match *self {
            Type(ref body) => Box::new(body.fields.iter()),
            Tuple(ref body) => Box::new(body.fields.iter()),
            SubType { ref parent, ref sub_type } => {
                Box::new(parent.fields.iter().chain(sub_type.fields.iter()))
            }
            _ => {
                return Err("has no fields".into());
            }
        };

        Ok(it)
    }

    pub fn field_by_ident(&self, ident: &str) -> Result<Option<&Loc<RpField>>> {
        for field in self.fields()? {
            if field.ident() == ident {
                return Ok(Some(field));
            }
        }

        Ok(None)
    }

    pub fn is_assignable_from(&self, other: &RpRegistered) -> bool {
        use self::RpRegistered::*;

        match (self, other) {
            // exact type
            (&Type(ref target), &Type(ref source)) => Rc::ptr_eq(target, source),
            // exact tuple
            (&Tuple(ref target), &Tuple(ref source)) => Rc::ptr_eq(target, source),
            // exact service
            (&Service(ref target), &Service(ref source)) => Rc::ptr_eq(target, source),
            // exact interface, with unknown sub-type.
            (&Interface(ref target), &Interface(ref source)) => Rc::ptr_eq(target, source),
            // exact enum, with unknown value
            (&Enum(ref target), &Enum(ref source)) => Rc::ptr_eq(target, source),
            // sub-type to parent
            (&Interface(ref target), &SubType { parent: ref source, sub_type: _ }) => {
                Rc::ptr_eq(target, source)
            }
            // enum constant to parent type
            (&Enum(ref target), &EnumConstant { parent: ref source, variant: _ }) => {
                Rc::ptr_eq(target, source)
            }
            // exact matching sub-type
            (&SubType { parent: ref target_parent, sub_type: ref target },
             &SubType { parent: ref source_parent, sub_type: ref source }) => {
                Rc::ptr_eq(target_parent, source_parent) && Rc::ptr_eq(target, source)
            }
            // exact matching constant
            (&EnumConstant { parent: ref target_parent, variant: ref target },
             &EnumConstant { parent: ref source_parent, variant: ref source }) => {
                Rc::ptr_eq(target_parent, source_parent) && Rc::ptr_eq(target, source)
            }
            _ => false,
        }
    }

    pub fn display(&self) -> String {
        use self::RpRegistered::*;

        match *self {
            Type(ref body) => format!("type {}", body.name.to_owned()),
            Interface(ref body) => format!("interface {}", body.name.to_owned()),
            Enum(ref body) => format!("enum {}", body.name.to_owned()),
            Tuple(ref body) => format!("tuple {}", body.name.to_owned()),
            Service(ref body) => format!("service {}", body.name.to_owned()),
            SubType { ref parent, ref sub_type } => {
                format!("type {}.{}", parent.name, sub_type.name)
            }
            EnumConstant { ref parent, ref variant } => {
                format!("{}.{}", parent.name, *variant.name)
            }
        }
    }

    pub fn name(&self) -> Vec<&str> {
        use self::RpRegistered::*;

        match *self {
            Type(ref body) => vec![&body.name],
            Interface(ref body) => vec![&body.name],
            Enum(ref body) => vec![&body.name],
            Tuple(ref body) => vec![&body.name],
            Service(ref body) => vec![&body.name],
            SubType { ref parent, ref sub_type } => vec![&parent.name, &sub_type.name],
            EnumConstant { ref parent, ref variant } => vec![&parent.name, &variant.name],
        }
    }
}
