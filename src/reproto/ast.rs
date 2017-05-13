use std::fmt::{Debug, Formatter, Error};
use std::collections::{HashSet, LinkedList};

#[derive(Debug, PartialEq)]
pub enum OptionValue {
    String(String),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Modifier {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    Custom(String),
}

#[derive(Debug, PartialEq)]
pub struct Package {
    parts: Vec<String>,
}

impl Package {
    pub fn new(parts: Vec<String>) -> Package {
        Package { parts: parts }
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub modifiers: Modifiers,
    pub name: String,
    pub type_: Type,
    pub id: u32,
}

impl Field {
    pub fn new(modifiers: Modifiers, name: String, type_: Type, id: u32) -> Field {
        Field {
            modifiers: modifiers,
            name: name,
            type_: type_,
            id: id,
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum MessageMember {
    Field(Field),
    OneOf(OneOf),
}

/// message <name> { <members>* }
#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub enum SubTypeMember {
    Field(Field),
    OneOf(OneOf),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum InterfaceMember {
    Field(Field),
    OneOf(OneOf),
    SubType(SubType),
}

/// interface <name> { <members>* }
#[derive(Debug, PartialEq)]
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
}

/// type <name> = <value>;
///
/// Example, simple type alias:
/// type Foo = Bar;
#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Message(MessageDecl),
    Interface(InterfaceDecl),
    Type(TypeDecl),
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
