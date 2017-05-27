use std::collections::btree_map;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use super::errors::*;
use token;

pub type Pos = (PathBuf, usize, usize);
pub type Token<T> = token::Token<T, Pos>;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Double,
    Float,
    Signed(Option<usize>),
    Unsigned(Option<usize>),
    Boolean,
    String,
    Bytes,
    Any,
    UsedType(String, String),
    Custom(String),
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Field {
    pub modifier: Modifier,
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(modifier: Modifier, name: String, ty: Type) -> Field {
        Field {
            modifier: modifier,
            name: name,
            ty: ty,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self.modifier {
            Modifier::Optional => true,
            _ => false,
        }
    }

    pub fn display(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub context: String,
    pub lines: Vec<String>,
}

impl Code {
    pub fn new(context: String, lines: Vec<String>) -> Code {
        Code {
            context: context,
            lines: lines,
        }
    }
}

pub trait BodyLike {
    fn mut_fields(&mut self) -> &mut Vec<Token<Field>>;
    fn fields(&self) -> &Vec<Token<Field>>;

    fn mut_codes(&mut self) -> &mut Vec<Token<Code>>;
    fn codes(&self) -> &Vec<Token<Code>>;

    /// Insert the given field, or return the already existing field if it already exists.
    fn push_if_absent(&mut self, field: &Token<Field>) -> Option<Pos> {
        for f in self.fields() {
            if f.name == field.name {
                return Some(f.pos.clone());
            }
        }

        self.mut_fields().push(field.clone());
        None
    }

    fn merge_body<B>(&mut self, other: &B) -> Result<()>
        where B: BodyLike
    {
        for field in other.fields() {
            if let Some(pos) = self.push_if_absent(field) {
                return Err(Error::field_conflict(field.name.clone(),
                                                 field.pos.clone(),
                                                 pos.clone()));
            }
        }

        for code in other.codes() {
            self.mut_codes().push(code.clone());
        }

        Ok(())
    }
}

impl BodyLike for InterfaceBody {
    fn mut_fields(&mut self) -> &mut Vec<Token<Field>> {
        &mut self.fields
    }

    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn mut_codes(&mut self) -> &mut Vec<Token<Code>> {
        &mut self.codes
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for TypeBody {
    fn mut_fields(&mut self) -> &mut Vec<Token<Field>> {
        &mut self.fields
    }

    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn mut_codes(&mut self) -> &mut Vec<Token<Code>> {
        &mut self.codes
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for EnumBody {
    fn mut_fields(&mut self) -> &mut Vec<Token<Field>> {
        &mut self.fields
    }

    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn mut_codes(&mut self) -> &mut Vec<Token<Code>> {
        &mut self.codes
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for TupleBody {
    fn mut_fields(&mut self) -> &mut Vec<Token<Field>> {
        &mut self.fields
    }

    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn mut_codes(&mut self) -> &mut Vec<Token<Code>> {
        &mut self.codes
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for SubType {
    fn mut_fields(&mut self) -> &mut Vec<Token<Field>> {
        &mut self.fields
    }

    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn mut_codes(&mut self) -> &mut Vec<Token<Code>> {
        &mut self.codes
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

#[derive(Debug, Clone)]
pub struct SubType {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub names: Vec<Token<String>>,
}

impl SubType {
    pub fn new(name: String,
               fields: Vec<Token<Field>>,
               codes: Vec<Token<Code>>,
               names: Vec<Token<String>>)
               -> SubType {
        SubType {
            name: name,
            fields: fields,
            codes: codes,
            names: names,
        }
    }

    pub fn merge(&mut self, other: &SubType) -> Result<()> {
        self.merge_body(other)
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceBody {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub sub_types: BTreeMap<String, Token<SubType>>,
}

impl InterfaceBody {
    pub fn new(name: String,
               fields: Vec<Token<Field>>,
               codes: Vec<Token<Code>>,
               sub_types: BTreeMap<String, Token<SubType>>)
               -> InterfaceBody {
        InterfaceBody {
            name: name,
            fields: fields,
            codes: codes,
            sub_types: sub_types,
        }
    }

    pub fn merge(&mut self, other: &InterfaceBody) -> Result<()> {
        self.merge_body(other)?;

        for (key, sub_type) in &other.sub_types {
            match self.sub_types.entry(key.clone()) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(sub_type.clone());
                }
                btree_map::Entry::Occupied(entry) => {
                    let entry = &mut entry.into_mut().inner;
                    entry.merge(&sub_type.inner)?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TypeBody {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
}

impl TypeBody {
    pub fn new(name: String, fields: Vec<Token<Field>>, codes: Vec<Token<Code>>) -> TypeBody {
        TypeBody {
            name: name,
            fields: fields,
            codes: codes,
        }
    }

    pub fn merge(&mut self, other: &TypeBody) -> Result<()> {
        self.merge_body(other)
    }
}

#[derive(Debug, Clone)]
pub struct TupleBody {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
}

impl TupleBody {
    pub fn new(name: String, fields: Vec<Token<Field>>, codes: Vec<Token<Code>>) -> TupleBody {
        TupleBody {
            name: name,
            fields: fields,
            codes: codes,
        }
    }

    pub fn merge(&mut self, other: &TupleBody) -> Result<()> {
        self.merge_body(other)
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub arguments: Vec<Token<Value>>,
}

#[derive(Debug, Clone)]
pub struct EnumBody {
    pub name: String,
    pub values: Vec<Token<EnumValue>>,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub serialized_as: Option<Token<String>>,
}

impl EnumBody {
    pub fn new(name: String,
               values: Vec<Token<EnumValue>>,
               fields: Vec<Token<Field>>,
               codes: Vec<Token<Code>>,
               serialized_as: Option<Token<String>>)
               -> EnumBody {
        EnumBody {
            name: name,
            values: values,
            fields: fields,
            codes: codes,
            serialized_as: serialized_as,
        }
    }

    pub fn merge(&mut self, other: &EnumBody) -> Result<()> {
        self.merge_body(other)
    }
}

#[derive(Clone)]
pub enum Decl {
    Type(TypeBody),
    Interface(InterfaceBody),
    Enum(EnumBody),
    Tuple(TupleBody),
}

impl Decl {
    pub fn name(&self) -> &str {
        match *self {
            Decl::Type(ref body) => &body.name,
            Decl::Interface(ref body) => &body.name,
            Decl::Enum(ref body) => &body.name,
            Decl::Tuple(ref body) => &body.name,
        }
    }

    pub fn display(&self) -> String {
        match *self {
            Decl::Type(ref body) => format!("type {}", body.name),
            Decl::Interface(ref body) => format!("interface {}", body.name),
            Decl::Enum(ref body) => format!("enum {}", body.name),
            Decl::Tuple(ref body) => format!("tuple {}", body.name),
        }
    }
}
