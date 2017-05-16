use std::collections::HashSet;

use errors::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OptionValue {
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OptionPair {
    pub name: String,
    pub values: Vec<OptionValue>,
}

impl OptionPair {
    pub fn new(name: String, values: Vec<OptionValue>) -> OptionPair {
        OptionPair {
            name: name,
            values: values,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Options {
    options: Vec<OptionPair>,
}

impl Options {
    pub fn new(options: Vec<OptionPair>) -> Options {
        Options { options: options }
    }

    pub fn lookup(&self, name: &str) -> Vec<&OptionValue> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .collect()
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
    Custom(String),
    Array(Box<Type>),
    Tuple(Vec<Type>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Package {
    pub parts: Vec<String>,
}

impl Package {
    pub fn new(parts: Vec<String>) -> Package {
        Package { parts: parts }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub modifier: Modifier,
    pub name: String,
    pub type_: Type,
    pub id: u32,
}

impl Field {
    pub fn new(modifier: Modifier, name: String, type_: Type, id: u32) -> Field {
        Field {
            modifier: modifier,
            name: name,
            type_: type_,
            id: id,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OneOf {
    pub name: String,
    pub fields: Vec<Field>,
}

impl OneOf {
    pub fn new(name: String, fields: Vec<Field>) -> OneOf {
        OneOf {
            name: name,
            fields: fields,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageMember {
    Field(Field),
    OneOf(OneOf),
}

/// message <name> { <members>* }
#[derive(Debug, PartialEq, Clone)]
pub struct MessageDecl {
    pub name: String,
    pub options: Options,
    pub members: Vec<MessageMember>,
}

impl MessageDecl {
    pub fn new(name: String, options: Options, members: Vec<MessageMember>) -> MessageDecl {
        MessageDecl {
            name: name,
            options: options,
            members: members,
        }
    }

    pub fn merge(&mut self, other: &Decl) -> Result<()> {
        if let Decl::Message(ref other) = *other {
            self.options.merge(&other.options);
            self.members.extend(other.members.clone());
            return Ok(());
        }

        return Err("unexpected declaration".into());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubTypeMember {
    Field(Field),
    OneOf(OneOf),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubType {
    pub name: String,
    pub options: Options,
    pub members: Vec<SubTypeMember>,
}

impl SubType {
    pub fn new(name: String, options: Options, members: Vec<SubTypeMember>) -> SubType {
        SubType {
            name: name,
            options: options,
            members: members,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InterfaceMember {
    Field(Field),
    OneOf(OneOf),
    SubType(SubType),
}

/// interface <name> { <members>* }
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceDecl {
    pub name: String,
    pub options: Options,
    pub members: Vec<InterfaceMember>,
}

impl InterfaceDecl {
    pub fn new(name: String, options: Options, members: Vec<InterfaceMember>) -> InterfaceDecl {
        InterfaceDecl {
            name: name,
            options: options,
            members: members,
        }
    }

    pub fn merge(&mut self, other: &Decl) -> Result<()> {
        if let Decl::Interface(ref other) = *other {
            self.options.merge(&other.options);
            self.members.extend(other.members.clone());
            return Ok(());
        }

        return Err("unexpected declaration".into());
    }
}

/// type <name> = <value>;
///
/// Example, simple type alias:
/// type Foo = Bar;
#[derive(Debug, PartialEq, Clone)]
pub struct TypeDecl {
    pub name: String,
    pub value: Type,
}

impl TypeDecl {
    pub fn new(name: String, value: Type) -> TypeDecl {
        TypeDecl {
            name: name,
            value: value,
        }
    }

    pub fn merge(&mut self, _: &Decl) -> Result<()> {
        return Err("cannot merge type declarations".into());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    Message(MessageDecl),
    Interface(InterfaceDecl),
    Type(TypeDecl),
}

impl Decl {
    pub fn name(&self) -> String {
        match *self {
            Decl::Message(ref message) => message.name.clone(),
            Decl::Interface(ref interface) => interface.name.clone(),
            Decl::Type(ref type_) => type_.name.clone(),
        }
    }

    pub fn merge(&mut self, other: &Decl) -> Result<()> {
        match *self {
            Decl::Message(ref mut message) => message.merge(other),
            Decl::Interface(ref mut interface) => interface.merge(other),
            Decl::Type(ref mut type_) => type_.merge(other),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub package: Package,
    pub decls: Vec<Decl>,
}

impl File {
    pub fn new(package: Package, decls: Vec<Decl>) -> File {
        File {
            package: package,
            decls: decls,
        }
    }
}
