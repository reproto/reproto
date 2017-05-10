use std::fmt::{Debug, Formatter, Error};

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Required,
    Optional,
    Repeated,
    None,
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
    pub modifier: Modifier,
    pub name: String,
    pub fieldType: Type,
    pub id: u32,
}

impl Field {
    pub fn new(modifier: Modifier, name: String, fieldType: Type, id: u32) -> Field {
        Field {
            modifier: modifier,
            name: name,
            fieldType: fieldType,
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
    pub members: Vec<MessageMember>,
}

impl MessageDecl {
    pub fn new(name: String, members: Vec<MessageMember>) -> MessageDecl {
        MessageDecl {
            name: name,
            members: members,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InterfaceMember {
    Field(Field),
    OneOf(OneOf),
}

/// interface <name> { <members>* }
#[derive(Debug, PartialEq)]
pub struct InterfaceDecl {
    pub name: String,
    pub members: Vec<InterfaceMember>,
}

impl InterfaceDecl {
    pub fn new(name: String, members: Vec<InterfaceMember>) -> InterfaceDecl {
        InterfaceDecl {
            name: name,
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
    MessageDecl(MessageDecl),
    InterfaceDecl(InterfaceDecl),
    TypeDecl(TypeDecl),
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
