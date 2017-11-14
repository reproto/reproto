use core::{Loc, OptionEntry, RpModifier, RpNumber, RpPackage, VersionReq};
use std::result;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Double,
    Float,
    Signed { size: Option<usize> },
    Unsigned { size: Option<usize> },
    Boolean,
    String,
    Bytes,
    Any,
    /// ISO-8601 for date and time.
    DateTime,
    Name { name: Name },
    Array { inner: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decl<'input> {
    Type(Loc<TypeBody<'input>>),
    Tuple(Loc<TupleBody<'input>>),
    Interface(Loc<InterfaceBody<'input>>),
    Enum(Loc<EnumBody<'input>>),
    Service(Loc<ServiceBody<'input>>),
}

impl<'input> Decl<'input> {
    pub fn name(&self) -> &str {
        use self::Decl::*;

        match *self {
            Type(ref body) => &body.name,
            Tuple(ref body) => &body.name,
            Interface(ref body) => &body.name,
            Enum(ref body) => &body.name,
            Service(ref body) => &body.name,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub ty: Option<Loc<Type>>,
    pub variants: Vec<Loc<EnumVariant<'input>>>,
    pub members: Vec<Loc<Member<'input>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant<'input> {
    pub name: Loc<&'input str>,
    pub comment: Vec<&'input str>,
    pub argument: Option<Loc<Value<'input>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field<'input> {
    pub modifier: RpModifier,
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub ty: Type,
    pub field_as: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct File<'input> {
    pub options: Vec<Loc<OptionDecl<'input>>>,
    pub uses: Vec<Loc<UseDecl<'input>>>,
    pub decls: Vec<Loc<Decl<'input>>>,
}

impl<'input> Field<'input> {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Name {
    Relative { parts: Vec<String> },
    Absolute {
        prefix: Option<String>,
        parts: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct InterfaceBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
    pub sub_types: Vec<Loc<SubType<'input>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Member<'input> {
    Field(Field<'input>),
    Code(&'input str, Vec<String>),
    Option(OptionDecl<'input>),
    InnerDecl(Decl<'input>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct OptionDecl<'input> {
    pub name: &'input str,
    pub value: Loc<Value<'input>>,
}

impl<'input> OptionEntry for OptionDecl<'input> {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_string(&self) -> result::Result<String, &'static str> {
        match *self.value.value() {
            Value::String(ref string) => Ok(string.to_string()),
            _ => Err("expected string"),
        }
    }

    fn as_number(&self) -> result::Result<RpNumber, &'static str> {
        match *self.value.value() {
            Value::Number(ref number) => Ok(number.clone()),
            _ => Err("expected number"),
        }
    }

    fn as_identifier(&self) -> result::Result<String, &'static str> {
        match *self.value.value() {
            Value::Identifier(ref identifier) => Ok(identifier.to_string()),
            _ => Err("expected identifier"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PathSegment<'input> {
    Literal { value: Loc<String> },
    Variable {
        name: Loc<&'input str>,
        ty: Loc<Type>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct PathSpec<'input> {
    pub segments: Vec<PathSegment<'input>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ServiceBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub endpoints: Vec<Loc<Endpoint<'input>>>,
}

/// Describes an endpoint.
#[derive(Debug, PartialEq, Eq)]
pub struct Endpoint<'input> {
    pub id: Loc<&'input str>,
    pub comment: Vec<&'input str>,
    pub alias: Option<String>,
    pub options: Vec<Loc<OptionDecl<'input>>>,
    pub request: Option<Channel>,
    pub response: Option<Channel>,
}

/// Describes how data is transferred over a channel.
#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    /// Single send.
    Unary { ty: Loc<Type> },
    /// Multiple sends.
    Streaming { ty: Loc<Type> },
}

/// Sub-types in interface declarations.
#[derive(Debug, PartialEq, Eq)]
pub struct SubType<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UseDecl<'input> {
    pub package: Loc<RpPackage>,
    pub version_req: Option<Loc<VersionReq>>,
    pub alias: Option<Loc<&'input str>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value<'input> {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(&'input str),
    Array(Vec<Loc<Value<'input>>>),
}
