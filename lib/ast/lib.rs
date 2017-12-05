extern crate reproto_core;

use reproto_core::{Loc, OptionEntry, RpModifier, RpNumber, RpPackage};

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

/// The body of an enum declaration.
///
/// ```ignore
/// /// <comment>
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

/// A field.
///
/// ```ignore
/// /// <comment>
/// <name><modifier>: <ty> as <field_as>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Field<'input> {
    pub modifier: RpModifier,
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub ty: Type,
    pub field_as: Option<String>,
}

/// A file.
///
/// ```ignore
/// //! <comment>
///
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
/// /// <comment>
/// interface <name> {
///   <members>
///   <sub_types>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct InterfaceBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
    pub sub_types: Vec<Loc<SubType<'input>>>,
}

/// A member in a tuple, type, or interface.
#[derive(Debug, PartialEq, Eq)]
pub enum Member<'input> {
    Field(Field<'input>),
    Code(&'input str, Vec<&'input str>),
    Option(OptionDecl<'input>),
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
/// /// <comment>
/// service <name> {
///   <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct ServiceBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<ServiceMember<'input>>,
}

/// A member of a service declaration.
#[derive(Debug, PartialEq, Eq)]
pub enum ServiceMember<'input> {
    Endpoint(Loc<Endpoint<'input>>),
    Option(Loc<OptionDecl<'input>>),
    InnerDecl(Loc<Decl<'input>>),
}

/// An endpoint
///
/// ```ignore
/// /// <comment>
/// #[attribute]
/// <id>(<arguments>) -> <response> as <alias> {
///   <options>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Endpoint<'input> {
    pub id: Loc<&'input str>,
    pub comment: Vec<&'input str>,
    /// Attributes associated with the endpoint.
    pub attributes: Vec<Loc<Attribute<'input>>>,
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
/// /// <comment>
/// <name> as <alias> {
///     <members>
/// }
/// ```
/// Sub-types in interface declarations.
#[derive(Debug, PartialEq, Eq)]
pub struct SubType<'input> {
    pub name: Loc<&'input str>,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
    pub alias: Option<Loc<Value<'input>>>,
}

/// The body of a tuple
///
/// ```ignore
/// /// <comment>
/// tuple <name> {
///     <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct TupleBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
}

/// The body of a type
///
/// ```ignore
/// /// <comment>
/// type <name> {
///     <members>
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct TypeBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<Loc<Member<'input>>>,
}

/// A use declaration
///
/// ```ignore
/// use <package> "<version req> as <alias>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct UseDecl<'input> {
    pub package: Loc<RpPackage>,
    pub version_req: Option<Loc<String>>,
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
