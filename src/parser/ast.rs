use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use errors::*;

/// Position relative in file where the declaration is present.
pub type Pos = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum OptionValue {
    String(String),
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

    pub fn lookup_string_nth(&self, name: &str, n: usize) -> Option<&String> {
        self.options
            .iter()
            .filter(|o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter())
            .flat_map(|v| match *v {
                OptionValue::String(ref s) => Some(s).into_iter(),
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
    Tuple(Vec<Type>),
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
    Field(Field, Pos),
    OneOf(OneOf, Pos),
}

/// message <name> { <members>* }
#[derive(Debug, PartialEq, Clone)]
pub struct MessageDecl {
    pub name: String,
    pub options: Options,
    pub members: Vec<MessageMember>,
    pub pos: Pos,
}

impl MessageDecl {
    pub fn new(name: String,
               options: Options,
               members: Vec<MessageMember>,
               pos: Pos)
               -> MessageDecl {
        MessageDecl {
            name: name,
            options: options,
            members: members,
            pos: pos,
        }
    }

    pub fn merge(&mut self, other: &Decl) -> Result<()> {
        if let Decl::Message(ref other) = *other {
            self.options.merge(&other.options);
            self.members.extend(other.members.clone());
            return Ok(());
        }

        return Err(format!("Expected Decl::Message, but got {:?}", other).into());
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
    pub pos: Pos,
}

impl SubType {
    pub fn new(name: String, options: Options, members: Vec<SubTypeMember>, pos: Pos) -> SubType {
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
pub enum InterfaceMember {
    Field(Field, Pos),
    OneOf(OneOf, Pos),
}

/// interface <name> { <members>* }
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceDecl {
    pub name: String,
    pub options: Options,
    pub members: Vec<InterfaceMember>,
    pub sub_types: BTreeMap<String, SubType>,
    pub pos: Pos,
}

impl InterfaceDecl {
    pub fn new(name: String,
               options: Options,
               members: Vec<InterfaceMember>,
               sub_types: BTreeMap<String, SubType>,
               pos: Pos)
               -> InterfaceDecl {
        InterfaceDecl {
            name: name,
            options: options,
            members: members,
            sub_types: sub_types,
            pos: pos,
        }
    }

    pub fn merge(&mut self, other: &Decl) -> Result<()> {
        if let Decl::Interface(ref other) = *other {
            self.options.merge(&other.options);
            self.members.extend(other.members.clone());

            for (key, sub_type) in &other.sub_types {
                match self.sub_types.entry(key.to_owned()) {
                    Entry::Vacant(entry) => {
                        entry.insert(sub_type.clone());
                    }
                    Entry::Occupied(entry) => {
                        entry.into_mut().merge(sub_type)?;
                    }
                }
            }

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
    pub pos: Pos,
}

impl TypeDecl {
    pub fn new(name: String, value: Type, pos: Pos) -> TypeDecl {
        TypeDecl {
            name: name,
            value: value,
            pos: pos,
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

    pub fn pos(&self) -> Pos {
        match *self {
            Decl::Message(ref message) => message.pos.clone(),
            Decl::Interface(ref interface) => interface.pos.clone(),
            Decl::Type(ref type_) => type_.pos.clone(),
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
