extern crate reproto_core as core;
extern crate reproto_lexer as lexer;

use core::{Loc, RpNumber, Span};
use std::borrow::Cow;
use std::fmt;
use std::ops;

/// A type reference, if arguments are present it is specialized.
///
/// ```ignore
/// <ident> "<" <arguments> ">"
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference<'input> {
    pub ident: Loc<Cow<'input, str>>,
    pub type_arguments: Vec<Loc<Type<'input>>>,
}

impl<'input> From<Loc<Cow<'input, str>>> for TypeReference<'input> {
    fn from(ident: Loc<Cow<'input, str>>) -> Self {
        TypeReference {
            ident,
            type_arguments: vec![],
        }
    }
}

impl<'input> fmt::Display for TypeReference<'input> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.type_arguments.is_empty() {
            return self.ident.fmt(fmt);
        }

        let mut it = self.type_arguments.iter().peekable();

        self.ident.fmt(fmt)?;

        "<".fmt(fmt)?;

        while let Some(argument) = it.next() {
            argument.fmt(fmt)?;

            if it.peek().is_some() {
                ", ".fmt(fmt)?;
            }
        }

        ">".fmt(fmt)?;

        Ok(())
    }
}

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
    pub comment: Vec<Cow<'input, str>>,
    pub attributes: Vec<Loc<Attribute<'input>>>,
    pub item: Loc<T>,
}

/// Item derefs into target.
impl<'input, T> ops::Deref for Item<'input, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Loc::borrow(&self.item)
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
    Word(Loc<Value<'input>>),
    NameValue {
        name: Loc<Cow<'input, str>>,
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
    Word(Loc<Cow<'input, str>>),
    List(Loc<Cow<'input, str>>, Vec<AttributeItem<'input>>),
}

/// A type.
///
/// For example: `u32`, `::Relative::Name`, or `bytes`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'input> {
    Double,
    Float,
    Signed {
        size: usize,
    },
    Unsigned {
        size: usize,
    },
    Boolean,
    String,
    Bytes,
    Any,
    /// ISO-8601 for date and time.
    DateTime,
    /// A generic argument, like `T`.
    Argument {
        argument: Loc<Cow<'input, str>>,
    },
    Name {
        name: Loc<Name<'input>>,
    },
    Array {
        inner: Box<Loc<Type<'input>>>,
    },
    Map {
        key: Box<Loc<Type<'input>>>,
        value: Box<Loc<Type<'input>>>,
    },
    /// A complete error.
    Error,
}

impl<'input> fmt::Display for Type<'input> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Type::*;

        match *self {
            Double => "double".fmt(fmt),
            Float => "float".fmt(fmt),
            Signed { ref size } => write!(fmt, "i{}", size),
            Unsigned { ref size } => write!(fmt, "u{}", size),
            Boolean => "boolean".fmt(fmt),
            String => "string".fmt(fmt),
            Bytes => "bytes".fmt(fmt),
            Any => "any".fmt(fmt),
            DateTime => "datetime".fmt(fmt),
            Argument { ref argument } => argument.fmt(fmt),
            Name { ref name } => name.fmt(fmt),
            Array { ref inner } => inner.fmt(fmt),
            Map { ref key, ref value } => write!(fmt, "{{{}: {}}}", key, value),
            Error => "*error*".fmt(fmt),
        }
    }
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
    /// Get the local name for the declaration.
    pub fn name(&self) -> Loc<&str> {
        use self::Decl::*;

        let name = match *self {
            Type(ref body) => &body.name,
            Tuple(ref body) => &body.name,
            Interface(ref body) => &body.name,
            Enum(ref body) => &body.name,
            Service(ref body) => &body.name,
        };

        Loc::map(Loc::as_ref(name), |n| n.as_ref())
    }

    /// Get all the sub-declarations of this declaraiton.
    pub fn decls(&self) -> impl Iterator<Item = &Decl<'input>> {
        use self::Decl::*;

        let out: Vec<_> = match *self {
            Type(ref body) => body.decls().collect(),
            Tuple(ref body) => body.decls().collect(),
            Interface(ref body) => body.decls().collect(),
            Enum(_) => vec![],
            Service(ref body) => body.decls().collect(),
        };

        out.into_iter()
    }

    /// Comment.
    pub fn comment(&self) -> &[Cow<'input, str>] {
        use self::Decl::*;

        match *self {
            Type(ref body) => &body.comment,
            Tuple(ref body) => &body.comment,
            Interface(ref body) => &body.comment,
            Enum(ref body) => &body.comment,
            Service(ref body) => &body.comment,
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
    pub name: Loc<Cow<'input, str>>,
    pub ty: Loc<Type<'input>>,
    pub variants: Vec<Item<'input, EnumVariant<'input>>>,
    pub members: Vec<EnumMember<'input>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant<'input> {
    pub name: Loc<Cow<'input, str>>,
    pub argument: Option<Loc<Value<'input>>>,
}

/// A member in a tuple, type, or interface.
#[derive(Debug, PartialEq, Eq)]
pub enum EnumMember<'input> {
    Code(Loc<Code<'input>>),
}

/// A field.
///
/// ```ignore
/// <name><modifier>: <ty> as <field_as>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Field<'input> {
    pub required: bool,
    pub name: Cow<'input, str>,
    pub ty: Loc<Type<'input>>,
    pub field_as: Option<String>,
    /// If the end-of-line indicator present.
    /// A `false` value should indicate an error.
    pub endl: bool,
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
    pub comment: Vec<Cow<'input, str>>,
    pub attributes: Vec<Loc<Attribute<'input>>>,
    pub uses: Vec<Loc<UseDecl<'input>>>,
    pub decls: Vec<Decl<'input>>,
}

impl<'input> Field<'input> {
    pub fn is_optional(&self) -> bool {
        !self.required
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Name<'input> {
    Relative {
        path: Vec<Loc<TypeReference<'input>>>,
    },
    Absolute {
        prefix: Option<Loc<Cow<'input, str>>>,
        path: Vec<Loc<TypeReference<'input>>>,
    },
}

impl<'input> fmt::Display for Name<'input> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Name::*;

        let path = match *self {
            Relative { ref path } => path,
            Absolute {
                ref prefix,
                ref path,
            } => {
                if let Some(ref prefix) = *prefix {
                    write!(fmt, "{}", prefix)?;
                }

                path
            }
        };

        let mut it = path.iter().peekable();

        while let Some(part) = it.next() {
            part.fmt(fmt)?;

            if it.peek().is_some() {
                "::".fmt(fmt)?;
            }
        }

        Ok(())
    }
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
    pub name: Loc<Cow<'input, str>>,
    pub members: Vec<TypeMember<'input>>,
    pub sub_types: Vec<Item<'input, SubType<'input>>>,
}

impl<'input> InterfaceBody<'input> {
    /// Access all inner declarations.
    fn decls<'a>(&'a self) -> impl Iterator<Item = &'a Decl<'input>> {
        self.members.iter().flat_map(|m| match *m {
            TypeMember::InnerDecl(ref decl) => Some(decl),
            _ => None,
        })
    }

    /// Access all fields.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = &'a Field<'input>> {
        self.members.iter().flat_map(|m| match *m {
            TypeMember::Field(ref field) => Some(Loc::borrow(&field.item)),
            _ => None,
        })
    }
}

/// A contextual code-block.
#[derive(Debug, PartialEq, Eq)]
pub struct Code<'input> {
    pub attributes: Vec<Loc<Attribute<'input>>>,
    pub context: Loc<Cow<'input, str>>,
    pub content: Vec<Cow<'input, str>>,
}

/// A member in a tuple, type, or interface.
#[derive(Debug, PartialEq, Eq)]
pub enum TypeMember<'input> {
    Field(Item<'input, Field<'input>>),
    Code(Loc<Code<'input>>),
    InnerDecl(Decl<'input>),
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
    pub name: Loc<Cow<'input, str>>,
    pub members: Vec<ServiceMember<'input>>,
}

impl<'input> ServiceBody<'input> {
    /// Access all inner declarations.
    fn decls<'a>(&'a self) -> impl Iterator<Item = &'a Decl<'input>> {
        self.members.iter().flat_map(|m| match *m {
            ServiceMember::InnerDecl(ref decl) => Some(decl),
            _ => None,
        })
    }

    /// Access all endpoints.
    pub fn endpoints(&self) -> Vec<&Endpoint<'input>> {
        let mut out = Vec::new();

        for m in &self.members {
            if let ServiceMember::Endpoint(ref endpoint) = *m {
                out.push(Loc::borrow(&endpoint.item));
            }
        }

        out
    }
}

/// A member of a service declaration.
#[derive(Debug, PartialEq, Eq)]
pub enum ServiceMember<'input> {
    Endpoint(Item<'input, Endpoint<'input>>),
    InnerDecl(Decl<'input>),
}

/// The argument in and endpoint.
#[derive(Debug, PartialEq, Eq)]
pub struct EndpointArgument<'input> {
    pub ident: Loc<Cow<'input, str>>,
    pub channel: Loc<Channel<'input>>,
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
    pub id: Loc<Cow<'input, str>>,
    pub alias: Option<String>,
    pub arguments: Vec<EndpointArgument<'input>>,
    pub response: Option<Loc<Channel<'input>>>,
}

/// Describes how data is transferred over a channel.
///
/// ```ignore
/// Unary(stream <ty>)
/// Streaming(<ty>)
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Channel<'input> {
    /// Single send.
    Unary { ty: Loc<Type<'input>> },
    /// Multiple sends.
    Streaming { ty: Loc<Type<'input>> },
}

impl<'input> Channel<'input> {
    /// Access the type of the channel.
    pub fn ty(&self) -> &Loc<Type<'input>> {
        use self::Channel::*;

        match *self {
            Unary { ref ty } => ty,
            Streaming { ref ty } => ty,
        }
    }
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
    pub name: Loc<Cow<'input, str>>,
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
    pub name: Loc<Cow<'input, str>>,
    pub members: Vec<TypeMember<'input>>,
}

impl<'input> TupleBody<'input> {
    /// Access all inner declarations.
    fn decls<'a>(&'a self) -> impl Iterator<Item = &'a Decl<'input>> {
        self.members.iter().flat_map(|m| match *m {
            TypeMember::InnerDecl(ref decl) => Some(decl),
            _ => None,
        })
    }

    /// Access all fields.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = &'a Field<'input>> {
        self.members.iter().flat_map(|m| match *m {
            TypeMember::Field(ref field) => Some(Loc::borrow(&field.item)),
            _ => None,
        })
    }
}

/// The body of a type
///
/// ```ignore
/// "type" <name> ("<" <type_arguments> ">")? {
///     <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct TypeBody<'input> {
    pub name: Loc<Cow<'input, str>>,
    pub members: Vec<TypeMember<'input>>,
    pub type_arguments: Vec<Loc<Cow<'input, str>>>,
}

impl<'input> TypeBody<'input> {
    /// Access all inner declarations.
    fn decls<'a>(&'a self) -> impl Iterator<Item = &'a Decl<'input>> {
        self.members.iter().flat_map(|m| match *m {
            TypeMember::InnerDecl(ref decl) => Some(decl),
            _ => None,
        })
    }

    /// Access all fields.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = &'a Field<'input>> {
        self.members.iter().flat_map(|m| match *m {
            TypeMember::Field(ref field) => Some(Loc::borrow(&field.item)),
            _ => None,
        })
    }
}

/// A package declaration.
#[derive(Debug, PartialEq, Eq)]
pub enum Package<'input> {
    /// A parsed package.
    Package { parts: Vec<Loc<Cow<'input, str>>> },
    /// A recovered error.
    Error,
}

/// A use declaration
///
/// ```ignore
/// use <package> "<range>" as <alias>;
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct UseDecl<'input> {
    pub package: Loc<Package<'input>>,
    pub range: Option<Loc<String>>,
    pub alias: Option<Loc<Cow<'input, str>>>,
    /// If the end-of-line indicator present.
    /// A empty value should indicate an error.
    pub endl: Option<Span>,
}

/// A literal value
///
/// For example, `"string"`, `42.0`, and `foo`.
#[derive(Debug, PartialEq, Eq)]
pub enum Value<'input> {
    String(String),
    Number(RpNumber),
    Identifier(Cow<'input, str>),
    Array(Vec<Loc<Value<'input>>>),
    Name(Loc<Name<'input>>),
}

/// A part of a step.
#[derive(Debug, PartialEq, Eq)]
pub enum PathPart<'input> {
    Variable(Cow<'input, str>),
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::{Loc, Span};

    #[test]
    fn test_type_reference_display() {
        let type_reference = Loc::new(
            TypeReference {
                ident: Loc::new(Cow::from("Foo"), Span::empty()),
                type_arguments: vec![],
            },
            Span::empty(),
        );

        assert_eq!("Foo", type_reference.to_string().as_str());

        let argument = |a| {
            Loc::new(
                Type::Argument {
                    argument: Loc::new(Cow::from(a), Span::empty()),
                },
                Span::empty(),
            )
        };

        let type_reference = Loc::new(
            TypeReference {
                ident: Loc::new(Cow::from("Foo"), Span::empty()),
                type_arguments: vec!["A", "B"].into_iter().map(argument).collect(),
            },
            Span::empty(),
        );

        assert_eq!("Foo<A, B>", type_reference.to_string().as_str());
    }
}
