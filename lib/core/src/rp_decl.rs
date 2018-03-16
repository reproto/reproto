//! Model for declarations

use super::{Loc, Pos, RpEnumBody, RpInterfaceBody, RpName, RpReg, RpServiceBody, RpTupleBody,
            RpTypeBody};
use std::fmt;
use std::rc::Rc;
use std::vec;

/// Iterator over declarations.
pub struct Decls<'a> {
    iter: vec::IntoIter<&'a RpDecl>,
}

impl<'a> Iterator for Decls<'a> {
    type Item = &'a RpDecl;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpDecl {
    Type(Rc<Loc<RpTypeBody>>),
    Tuple(Rc<Loc<RpTupleBody>>),
    Interface(Rc<Loc<RpInterfaceBody>>),
    Enum(Rc<Loc<RpEnumBody>>),
    Service(Rc<Loc<RpServiceBody>>),
}

impl RpDecl {
    pub fn decls(&self) -> Decls {
        use self::RpDecl::*;

        let iter = match *self {
            Type(ref body) => body.decls.iter().collect::<Vec<_>>().into_iter(),
            Interface(ref body) => {
                let mut decls = body.decls.iter().collect::<Vec<_>>();
                decls.extend(body.sub_types.iter().flat_map(|s| s.decls.iter()));
                decls.into_iter()
            }
            Enum(ref body) => body.decls.iter().collect::<Vec<_>>().into_iter(),
            Tuple(ref body) => body.decls.iter().collect::<Vec<_>>().into_iter(),
            Service(ref body) => body.decls.iter().collect::<Vec<_>>().into_iter(),
        };

        Decls { iter: iter }
    }

    pub fn ident(&self) -> &str {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => body.ident.as_str(),
            Interface(ref body) => body.ident.as_str(),
            Enum(ref body) => body.ident.as_str(),
            Tuple(ref body) => body.ident.as_str(),
            Service(ref body) => body.ident.as_str(),
        }
    }

    pub fn name(&self) -> &RpName {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => &body.name,
            Interface(ref body) => &body.name,
            Enum(ref body) => &body.name,
            Tuple(ref body) => &body.name,
            Service(ref body) => &body.name,
        }
    }

    pub fn comment(&self) -> &[String] {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => &body.comment,
            Interface(ref body) => &body.comment,
            Enum(ref body) => &body.comment,
            Tuple(ref body) => &body.comment,
            Service(ref body) => &body.comment,
        }
    }

    /// Convert a declaration into its registered types.
    pub fn to_reg(&self) -> Vec<RpReg> {
        use self::RpDecl::*;

        let mut out = Vec::new();

        match *self {
            Type(ref ty) => {
                out.push(RpReg::Type(Rc::clone(ty)));
            }
            Interface(ref interface) => {
                for sub_type in interface.sub_types.iter() {
                    out.push(RpReg::SubType(Rc::clone(interface), Rc::clone(sub_type)));
                }

                out.push(RpReg::Interface(Rc::clone(interface)));
            }
            Enum(ref en) => {
                for variant in &en.variants {
                    out.push(RpReg::EnumVariant(Rc::clone(en), Rc::clone(variant)));
                }

                out.push(RpReg::Enum(Rc::clone(en)));
            }
            Tuple(ref tuple) => {
                out.push(RpReg::Tuple(Rc::clone(tuple)));
            }
            Service(ref service) => {
                out.push(RpReg::Service(Rc::clone(service)));
            }
        }

        out.extend(self.decls().flat_map(|d| d.to_reg()));
        out
    }

    /// Get stringy kind of the declaration.
    pub fn kind(&self) -> &str {
        use self::RpDecl::*;

        match *self {
            Type(_) => "type",
            Interface(_) => "interface",
            Enum(_) => "enum",
            Tuple(_) => "tuple",
            Service(_) => "service",
        }
    }

    /// Get the position of the declaration.
    pub fn pos(&self) -> &Pos {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => Loc::pos(body),
            Interface(ref body) => Loc::pos(body),
            Enum(ref body) => Loc::pos(body),
            Tuple(ref body) => Loc::pos(body),
            Service(ref body) => Loc::pos(body),
        }
    }
}

impl fmt::Display for RpDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => write!(f, "type {}", body.name),
            Interface(ref body) => write!(f, "interface {}", body.name),
            Enum(ref body) => write!(f, "enum {}", body.name),
            Tuple(ref body) => write!(f, "tuple {}", body.name),
            Service(ref body) => write!(f, "service {}", body.name),
        }
    }
}
