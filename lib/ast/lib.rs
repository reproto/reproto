extern crate reproto_core;

use reproto_core::{Loc, OptionEntry, RpModifier, RpNumber, RpPackage, WithPos};
use std::ops;

/// Items can be commented and have attributes.
///
/// This is an intermediate structure used to return these properties.
///
/// ```ignore
/// /// This is a comment.
/// #[foo]
/// #[foo(value = "hello")]
/// <item>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Item<'input, T> {
    pub comment: Vec<&'input str>,
    pub attributes: Vec<Loc<Attribute<'input>>>,
    pub item: Loc<T>,
}

/// Item derefs into target.
impl<'input, T> ops::Deref for Item<'input, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item.value()
    }
}

impl<'input, T> Item<'input, T> {
    pub fn map<F, E: WithPos, U>(self, f: F) -> Result<Loc<U>, E>
    where
        F: FnOnce(Vec<&'input str>, Vec<Loc<Attribute<'input>>>, T) -> Result<U, E>,
    {
        let (value, pos) = self.item.take_pair();

        match f(self.comment, self.attributes, value) {
            Ok(o) => Ok(Loc::new(o, pos)),
            Err(e) => Err(e.with_pos(pos)),
        }
    }
}

/// Name value pair.
///
/// Is associated with attributes:
///
/// ```ignore
/// #[attribute(name = <value>)]
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum AttributeItem<'input> {
    Word(Loc<&'input str>),
    NameValue {
        name: Loc<&'input str>,
        value: Loc<Value<'input>>,
    },
}

/// An attribute.
///
/// Attributes are metadata associated with elements.
///
/// ```ignore
/// #[word]
/// ```
///
/// or:
///
/// ```ignore
/// #[name_value(foo = <value>, bar = <value>)]
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Attribute<'input> {
    Word(Loc<&'input str>),
    List(Loc<&'input str>, Vec<AttributeItem<'input>>),
}

/// A type.
///
/// For example: `u32`, `::Relative::Name`, or `bytes`.
#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Double,
    Float,
    Signed { size: usize },
    Unsigned { size: usize },
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

/// Any kind of declaration.
#[derive(Debug, PartialEq, Eq)]
pub enum Decl<'input> {
    Type(Item<'input, TypeBody<'input>>),
    Tuple(Item<'input, TupleBody<'input>>),
    Interface(Item<'input, InterfaceBody<'input>>),
    Enum(Item<'input, EnumBody<'input>>),
    Service(Item<'input, ServiceBody<'input>>),
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

/// The body of an enum declaration.
///
/// ```ignore
/// enum <name> as <ty> {
///   <variants>
///
///   <members>
/// }
/// ```
///
/// Note: members must only be options.
#[derive(Debug, PartialEq, Eq)]
pub struct EnumBody<'input> {
    pub name: &'input str,
    pub ty: Option<Loc<Type>>,
    pub variants: Vec<Item<'input, EnumVariant<'input>>>,
    pub members: Vec<EnumMember<'input>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant<'input> {
    pub name: Loc<&'input str>,
    pub argument: Option<Loc<Value<'input>>>,
}

/// A member in a tuple, type, or interface.
#[derive(Debug, PartialEq, Eq)]
pub enum EnumMember<'input> {
    Code(Loc<Code<'input>>),
    Option(Loc<OptionDecl<'input>>),
    InnerDecl(Decl<'input>),
}

/// A field.
///
/// ```ignore
/// <name><modifier>: <ty> as <field_as>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Field<'input> {
    pub modifier: RpModifier,
    pub name: &'input str,
    pub ty: Type,
    pub field_as: Option<String>,
}

/// A file.
///
/// ```ignore
/// <uses>
///
/// <options>
///
/// <decls>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct File<'input> {
    pub comment: Vec<&'input str>,
    pub options: Vec<Loc<OptionDecl<'input>>>,
    pub uses: Vec<Loc<UseDecl<'input>>>,
    pub decls: Vec<Decl<'input>>,
}

impl<'input> Field<'input> {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }
}

/// A name.
///
/// Either:
///
/// ```ignore
/// ::Relative::Name
/// ```
///
/// Or:
///
/// ```ignore
/// <prefix::>Absolute::Name
/// ```
///
/// Note: prefixes names are _always_ imported with `UseDecl`.
#[derive(Debug, PartialEq, Eq)]
pub enum Name {
    Relative { parts: Vec<String> },
    Absolute {
        prefix: Option<String>,
        parts: Vec<String>,
    },
}

/// The body of an interface declaration
///
/// ```ignore
/// interface <name> {
///   <members>
///   <sub_types>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct InterfaceBody<'input> {
    pub name: &'input str,
    pub members: Vec<TypeMember<'input>>,
    pub sub_types: Vec<Item<'input, SubType<'input>>>,
}

/// A contextual code-block.
#[derive(Debug, PartialEq, Eq)]
pub struct Code<'input> {
    pub context: &'input str,
    pub content: Vec<&'input str>,
}

/// A member in a tuple, type, or interface.
#[derive(Debug, PartialEq, Eq)]
pub enum TypeMember<'input> {
    Field(Item<'input, Field<'input>>),
    Code(Loc<Code<'input>>),
    Option(Loc<OptionDecl<'input>>),
    InnerDecl(Decl<'input>),
}

/// An option declaration.
///
/// ```ignore
/// option <name> = <value>;
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct OptionDecl<'input> {
    pub name: &'input str,
    pub value: Loc<Value<'input>>,
}

impl<'input> OptionEntry for OptionDecl<'input> {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_string(&self) -> Result<String, &'static str> {
        match *self.value.value() {
            Value::String(ref string) => Ok(string.to_string()),
            _ => Err("expected string"),
        }
    }

    fn as_number(&self) -> Result<RpNumber, &'static str> {
        match *self.value.value() {
            Value::Number(ref number) => Ok(number.clone()),
            _ => Err("expected number"),
        }
    }

    fn as_identifier(&self) -> Result<String, &'static str> {
        match *self.value.value() {
            Value::Identifier(ref identifier) => Ok(identifier.to_string()),
            _ => Err("expected identifier"),
        }
    }
}

/// The body of a service declaration.
///
/// ```ignore
/// service <name> {
///   <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct ServiceBody<'input> {
    pub name: &'input str,
    pub members: Vec<ServiceMember<'input>>,
}

/// A member of a service declaration.
#[derive(Debug, PartialEq, Eq)]
pub enum ServiceMember<'input> {
    Endpoint(Item<'input, Endpoint<'input>>),
    Option(Loc<OptionDecl<'input>>),
    InnerDecl(Decl<'input>),
}

/// An endpoint
///
/// ```ignore
/// <id>(<arguments>) -> <response> as <alias> {
///   <options>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Endpoint<'input> {
    pub id: Loc<&'input str>,
    pub alias: Option<String>,
    pub options: Vec<Loc<OptionDecl<'input>>>,
    pub arguments: Vec<(Loc<&'input str>, Loc<Channel>)>,
    pub response: Option<Loc<Channel>>,
}

/// Describes how data is transferred over a channel.
///
/// ```ignore
/// Unary(stream <ty>)
/// Streaming(<ty>)
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    /// Single send.
    Unary { ty: Type },
    /// Multiple sends.
    Streaming { ty: Type },
}

/// The body of a sub-type
///
/// ```ignore
/// <name> as <alias> {
///     <members>
/// }
/// ```
/// Sub-types in interface declarations.
#[derive(Debug, PartialEq, Eq)]
pub struct SubType<'input> {
    pub name: Loc<&'input str>,
    pub members: Vec<TypeMember<'input>>,
    pub alias: Option<Loc<Value<'input>>>,
}

/// The body of a tuple
///
/// ```ignore
/// tuple <name> {
///     <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct TupleBody<'input> {
    pub name: &'input str,
    pub members: Vec<TypeMember<'input>>,
}

/// The body of a type
///
/// ```ignore
/// type <name> {
///     <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct TypeBody<'input> {
    pub name: &'input str,
    pub members: Vec<TypeMember<'input>>,
}

/// A use declaration
///
/// ```ignore
/// use <package> "<version req> as <alias>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct UseDecl<'input> {
    pub package: Loc<RpPackage>,
    pub range: Option<Loc<String>>,
    pub alias: Option<Loc<&'input str>>,
}

/// A literal value
///
/// For example, `"string"`, `42.0`, and `foo`.
#[derive(Debug, PartialEq, Eq)]
pub enum Value<'input> {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(&'input str),
    Array(Vec<Loc<Value<'input>>>),
    Type(Type),
}

/// A part of a step.
#[derive(Debug, PartialEq, Eq)]
pub enum PathPart<'input> {
    Variable(&'input str),
    Segment(String),
}

/// A step in a path specification.
#[derive(Debug, PartialEq, Eq)]
pub struct PathStep<'input> {
    pub parts: Vec<PathPart<'input>>,
}

/// A path specification.
#[derive(Debug, PartialEq, Eq)]
pub struct PathSpec<'input> {
    pub steps: Vec<PathStep<'input>>,
}
