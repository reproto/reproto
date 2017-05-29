use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use super::errors::*;
use token;

pub type Pos = (PathBuf, usize, usize);
pub type Token<T> = token::Token<T, Pos>;

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<Token<Value>>,
}

impl OptionDecl {
    pub fn new(name: String, values: Vec<Token<Value>>) -> OptionDecl {
        OptionDecl {
            name: name,
            values: values,
        }
    }
}

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
    Number(f64),
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
    fn fields(&self) -> &Vec<Token<Field>>;
    fn codes(&self) -> &Vec<Token<Code>>;
}

impl BodyLike for InterfaceBody {
    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for TypeBody {
    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for EnumBody {
    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for TupleBody {
    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<Token<Code>> {
        &self.codes
    }
}

impl BodyLike for SubType {
    fn fields(&self) -> &Vec<Token<Field>> {
        &self.fields
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

#[derive(Debug)]
pub struct Options {
    options: Vec<Token<OptionDecl>>,
}

impl Options {
    pub fn new(options: Vec<Token<OptionDecl>>) -> Options {
        Options { options: options }
    }

    pub fn lookup<'a>(&'a self, name: &'a str) -> Box<Iterator<Item = &Token<Value>> + 'a> {
        let it = self.options
            .iter();

        Box::new(it.filter(move |o| o.name.as_str() == name)
            .flat_map(|o| o.values.iter()))
    }

    pub fn find_all_strings(&self, name: &str) -> Result<Vec<Token<String>>> {
        let mut out: Vec<Token<String>> = Vec::new();

        for s in self.lookup(name) {
            match **s {
                Value::String(ref string) => {
                    out.push(s.map_inner(|_| string.clone()));
                }
                _ => {
                    return Err(Error::pos(format!("{}: expected string", name), s.pos.clone()));
                }
            }
        }

        Ok(out)
    }

    pub fn find_one_identifier(&self, name: &str) -> Result<Option<Token<String>>> {
        let mut out: Option<Token<String>> = None;

        for s in self.lookup(name) {
            if let Some(_) = out {
                return Err(Error::pos(format!("{}: only one value may be present", name),
                                      s.pos.clone()));
            }

            match **s {
                Value::Identifier(ref string) => {
                    out = Some(s.map_inner(|_| string.clone()));
                }
                _ => {
                    return Err(Error::pos(format!("{}: expected identifier", name), s.pos.clone()));
                }
            }
        }

        Ok(out)
    }
}
