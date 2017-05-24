use std::collections::HashSet;
use std::collections::BTreeMap;

use super::errors::*;

/// Position relative in file where the declaration is present.
pub type Pos = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum OptionValue {
    String(String),
    Number(i64),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<OptionValue>,
}

impl OptionDecl {
    pub fn new(name: String, values: Vec<OptionValue>) -> OptionDecl {
        OptionDecl {
            name: name,
            values: values,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Options {
    options: Vec<OptionDecl>,
}

impl Options {
    pub fn new(options: Vec<OptionDecl>) -> Options {
        Options { options: options }
    }

    pub fn lookup(&self, name: &str) -> Vec<&OptionValue> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .collect()
    }

    pub fn lookup_nth(&self, name: &str, n: usize) -> Option<&OptionValue> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .nth(n)
    }

    pub fn lookup_string(&self, name: &str) -> Vec<&String> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .flat_map(|v| match *v {
                OptionValue::String(ref s) => Some(s).into_iter(),
                _ => None.into_iter(),
            })
            .collect()
    }

    pub fn lookup_string_nth(&self, name: &str, n: usize) -> Option<&String> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .flat_map(|v| match *v {
                OptionValue::String(ref s) => Some(s).into_iter(),
                _ => None.into_iter(),
            })
            .nth(n)
    }

    pub fn lookup_identifier(&self, name: &str) -> Vec<&String> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .flat_map(|v| match *v {
                OptionValue::Identifier(ref s) => Some(s).into_iter(),
                _ => None.into_iter(),
            })
            .collect()
    }

    pub fn lookup_identifier_nth(&self, name: &str, n: usize) -> Option<&String> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .flat_map(|v| match *v {
                OptionValue::Identifier(ref s) => Some(s).into_iter(),
                _ => None.into_iter(),
            })
            .nth(n)
    }

    pub fn merge(&mut self, other: &Options) {
        self.options.extend(other.options.clone());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Modifier {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Modifiers {
    modifiers: HashSet<Modifier>,
}

impl Modifiers {
    pub fn new(modifiers: HashSet<Modifier>) -> Modifiers {
        Modifiers { modifiers: modifiers }
    }

    pub fn test(&self, modifier: &Modifier) -> bool {
        self.modifiers.contains(modifier)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Double,
    Float,
    I32,
    I64,
    U32,
    U64,
    Bool,
    String,
    Bytes,
    Any,
    UsedType(String, String),
    Custom(String),
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(i64),
    Identifier(String),
    Type(Type),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Package {
    pub parts: Vec<String>,
}

impl Package {
    pub fn new(parts: Vec<String>) -> Package {
        Package { parts: parts }
    }

    pub fn join(&self, other: &Package) -> Package {
        let mut parts = self.parts.clone();
        parts.extend(other.parts.clone());
        Package::new(parts)
    }
}

impl ::std::fmt::Display for Package {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub modifier: Modifier,
    pub name: String,
    pub ty: Type,
    pub id: u32,
}

impl Field {
    pub fn new(modifier: Modifier, name: String, ty: Type, id: u32) -> Field {
        Field {
            modifier: modifier,
            name: name,
            ty: ty,
            id: id,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self.modifier {
            Modifier::Optional => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Member {
    Field(Field, Pos),
    Code(String, Vec<String>, Pos),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubType {
    pub name: String,
    pub options: Options,
    pub members: Vec<Member>,
    pub pos: Pos,
}

pub trait Body {
    fn name(&self) -> &str;
}

impl Body for TupleBody {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Body for TypeBody {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Body for EnumBody {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Body for InterfaceBody {
    fn name(&self) -> &str {
        &self.name
    }
}

impl SubType {
    pub fn new(name: String, options: Options, members: Vec<Member>, pos: Pos) -> SubType {
        SubType {
            name: name,
            options: options,
            members: members,
            pos: pos,
        }
    }

    pub fn merge(&mut self, other: &SubType) -> Result<()> {
        self.options.merge(&other.options);
        self.members.extend(other.members.clone());
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleBody {
    pub name: String,
    pub options: Options,
    pub members: Vec<Member>,
}

impl TupleBody {
    pub fn new(name: String, options: Options, members: Vec<Member>) -> TupleBody {
        TupleBody {
            name: name,
            options: options,
            members: members,
        }
    }

    pub fn merge(&mut self, other: &TupleBody) -> Result<()> {
        self.options.merge(&other.options);
        self.members.extend(other.members.clone());

        return Ok(());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceBody {
    pub name: String,
    pub options: Options,
    pub members: Vec<Member>,
    pub sub_types: BTreeMap<String, SubType>,
}

impl InterfaceBody {
    pub fn new(name: String,
               options: Options,
               members: Vec<Member>,
               sub_types: BTreeMap<String, SubType>)
               -> InterfaceBody {
        InterfaceBody {
            name: name,
            options: options,
            members: members,
            sub_types: sub_types,
        }
    }

    pub fn merge(&mut self, other: &InterfaceBody) -> Result<()> {
        self.options.merge(&other.options);
        self.members.extend(other.members.clone());

        return Ok(());
    }
}

/// type <name> { <members>* }
#[derive(Debug, PartialEq, Clone)]
pub struct TypeBody {
    pub name: String,
    pub options: Options,
    pub members: Vec<Member>,
}

impl TypeBody {
    pub fn new(name: String, options: Options, members: Vec<Member>) -> TypeBody {
        TypeBody {
            name: name,
            options: options,
            members: members,
        }
    }

    pub fn merge(&mut self, other: &TypeBody) -> Result<()> {
        self.options.merge(&other.options);
        self.members.extend(other.members.clone());
        return Ok(());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumBody {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub options: Options,
    pub members: Vec<Member>,
}

impl EnumBody {
    pub fn new(name: String,
               values: Vec<EnumValue>,
               options: Options,
               members: Vec<Member>)
               -> EnumBody {
        EnumBody {
            name: name,
            values: values,
            options: options,
            members: members,
        }
    }

    pub fn merge(&mut self, other: &EnumBody) -> Result<()> {
        self.options.merge(&other.options);
        self.members.extend(other.members.clone());

        return Ok(());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumValue {
    pub name: String,
    pub values: Vec<Literal>,
}

impl EnumValue {
    pub fn new(name: String, values: Vec<Literal>) -> EnumValue {
        EnumValue {
            name: name,
            values: values,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    Type(TypeBody, Pos),
    Tuple(TupleBody, Pos),
    Interface(InterfaceBody, Pos),
    Enum(EnumBody, Pos),
}

impl Decl {
    pub fn name(&self) -> String {
        match *self {
            Decl::Interface(ref interface, _) => interface.name.clone(),
            Decl::Type(ref ty, _) => ty.name.clone(),
            Decl::Tuple(ref ty, _) => ty.name.clone(),
            Decl::Enum(ref ty, _) => ty.name.clone(),
        }
    }

    pub fn pos(&self) -> Pos {
        match *self {
            Decl::Interface(_, pos) => pos.clone(),
            Decl::Type(_, pos) => pos.clone(),
            Decl::Tuple(_, pos) => pos.clone(),
            Decl::Enum(_, pos) => pos.clone(),
        }
    }

    pub fn display(&self) -> String {
        match *self {
            Decl::Interface(ref body, _) => format!("interface {}", body.name),
            Decl::Type(ref body, _) => format!("type {}", body.name),
            Decl::Tuple(ref body, _) => format!("tuple {}", body.name),
            Decl::Enum(ref body, _) => format!("enum {}", body.name),
        }
    }

    pub fn merge(&mut self, other: &Decl) -> Result<()> {
        match *self {
            Decl::Interface(ref mut body, _) => {
                if let Decl::Interface(ref other, _) = *other {
                    return body.merge(other);
                }
            }
            Decl::Type(ref mut body, _) => {
                if let Decl::Type(ref other, _) = *other {
                    return body.merge(other);
                }
            }
            Decl::Tuple(ref mut body, _) => {
                if let Decl::Tuple(ref other, _) = *other {
                    return body.merge(other);
                }
            }
            _ => {}
        }

        Err(ErrorKind::InvalidMerge(self.clone(), other.clone()).into())
    }
}

#[derive(Debug, PartialEq)]
pub struct UseDecl {
    pub package: Package,
    pub alias: Option<String>,
}

impl UseDecl {
    pub fn new(package: Package, alias: Option<String>) -> UseDecl {
        UseDecl {
            package: package,
            alias: alias,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub package: Package,
    pub uses: Vec<UseDecl>,
    pub decls: Vec<Decl>,
}

impl File {
    pub fn new(package: Package, uses: Vec<UseDecl>, decls: Vec<Decl>) -> File {
        File {
            package: package,
            uses: uses,
            decls: decls,
        }
    }
}
