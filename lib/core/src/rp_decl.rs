//! Model for declarations

use errors::Result;
use serde::Serialize;
use std::fmt;
use std::vec;
use {Flavor, Loc, RpEnumBody, RpInterfaceBody, RpReg, RpServiceBody, RpSubType, RpTupleBody,
     RpTypeBody, RpVariant, Span, Translate, Translator};

/// Iterator over declarations.
pub struct Decls<'a, F: 'static>
where
    F: Flavor,
{
    iter: vec::IntoIter<&'a RpDecl<F>>,
}

impl<'a, F: 'static> Iterator for Decls<'a, F>
where
    F: Flavor,
{
    type Item = &'a RpDecl<F>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(
    bound = "F: Serialize, F::Field: Serialize, F::Endpoint: Serialize, F::Package: Serialize, \
             F::Name: Serialize"
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpNamed<'a, F: 'static>
where
    F: Flavor,
{
    Type(&'a Loc<RpTypeBody<F>>),
    Tuple(&'a Loc<RpTupleBody<F>>),
    Interface(&'a Loc<RpInterfaceBody<F>>),
    SubType(&'a Loc<RpSubType<F>>),
    Enum(&'a Loc<RpEnumBody<F>>),
    EnumVariant(&'a Loc<RpVariant<F>>),
    Service(&'a Loc<RpServiceBody<F>>),
}

impl<'a, F: 'static> RpNamed<'a, F>
where
    F: Flavor,
{
    /// Get the name of the named element.
    pub fn name(&self) -> &F::Name {
        use self::RpNamed::*;

        match *self {
            Type(body) => &body.name,
            Tuple(tuple) => &tuple.name,
            Interface(interface) => &interface.name,
            SubType(sub_type) => &sub_type.name,
            Enum(en) => &en.name,
            EnumVariant(variant) => &variant.name,
            Service(service) => &service.name,
        }
    }

    /// Get the position of the named element.
    pub fn span(&self) -> &Span {
        use self::RpNamed::*;

        match *self {
            Type(body) => Loc::span(body),
            Tuple(tuple) => Loc::span(tuple),
            Interface(interface) => Loc::span(interface),
            SubType(sub_type) => Loc::span(sub_type),
            Enum(en) => Loc::span(en),
            EnumVariant(variant) => Loc::span(variant),
            Service(service) => Loc::span(service),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(
    bound = "F: Serialize, F::Field: Serialize, F::Endpoint: Serialize, F::Package: Serialize, \
             F::Name: Serialize"
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpDecl<F: 'static>
where
    F: Flavor,
{
    Type(Loc<RpTypeBody<F>>),
    Tuple(Loc<RpTupleBody<F>>),
    Interface(Loc<RpInterfaceBody<F>>),
    Enum(Loc<RpEnumBody<F>>),
    Service(Loc<RpServiceBody<F>>),
}

impl<F: 'static> RpDecl<F>
where
    F: Flavor,
{
    pub fn decls(&self) -> Decls<F> {
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

    pub fn name(&self) -> &F::Name {
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
    pub fn to_reg(&self) -> Vec<(&F::Name, &Span, RpReg)> {
        use self::RpDecl::*;

        let mut out = Vec::new();

        match *self {
            Type(ref ty) => {
                out.push((&ty.name, Loc::span(ty), RpReg::Type));
            }
            Interface(ref interface) => {
                for sub_type in interface.sub_types.iter() {
                    out.push((&sub_type.name, Loc::span(sub_type), RpReg::SubType));
                }

                out.push((&interface.name, Loc::span(interface), RpReg::Interface));
            }
            Enum(ref en) => {
                for variant in &en.variants {
                    out.push((&variant.name, Loc::span(variant), RpReg::EnumVariant));
                }

                out.push((&en.name, Loc::span(en), RpReg::Enum));
            }
            Tuple(ref tuple) => {
                out.push((&tuple.name, Loc::span(tuple), RpReg::Tuple));
            }
            Service(ref service) => {
                out.push((&service.name, Loc::span(service), RpReg::Service));
            }
        }

        out.extend(self.decls().flat_map(|d| d.to_reg()));
        out
    }

    /// Convert a declaration into its names.
    pub fn to_named(&self) -> Vec<RpNamed<F>> {
        use self::RpDecl::*;

        let mut out = Vec::new();

        match *self {
            Type(ref ty) => {
                out.push(RpNamed::Type(ty));
            }
            Interface(ref interface) => {
                for sub_type in interface.sub_types.iter() {
                    out.push(RpNamed::SubType(sub_type));
                }

                out.push(RpNamed::Interface(interface));
            }
            Enum(ref en) => {
                for variant in &en.variants {
                    out.push(RpNamed::EnumVariant(variant));
                }

                out.push(RpNamed::Enum(en));
            }
            Tuple(ref tuple) => {
                out.push(RpNamed::Tuple(tuple));
            }
            Service(ref service) => {
                out.push(RpNamed::Service(service));
            }
        }

        out.extend(self.decls().flat_map(|d| d.to_named()));
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
    pub fn span(&self) -> &Span {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => Loc::span(body),
            Interface(ref body) => Loc::span(body),
            Enum(ref body) => Loc::span(body),
            Tuple(ref body) => Loc::span(body),
            Service(ref body) => Loc::span(body),
        }
    }
}

impl<F: 'static, T> Translate<T> for RpDecl<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpDecl<T::Target>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<RpDecl<T::Target>> {
        use self::RpDecl::*;

        let out = match self {
            Type(body) => Type(body.translate(translator)?),
            Tuple(body) => Tuple(body.translate(translator)?),
            Interface(body) => Interface(body.translate(translator)?),
            Enum(body) => Enum(body.translate(translator)?),
            Service(body) => Service(body.translate(translator)?),
        };

        Ok(out)
    }
}

impl<F: 'static> fmt::Display for RpDecl<F>
where
    F: Flavor,
{
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

impl<'a, F: 'static> From<&'a RpDecl<F>> for Span
where
    F: Flavor,
{
    fn from(value: &'a RpDecl<F>) -> Self {
        value.span().clone()
    }
}
