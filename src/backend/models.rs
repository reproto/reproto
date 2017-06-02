pub use token::WithPrefix;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use super::errors::*;
use token;

pub type Pos = (PathBuf, usize, usize);
pub type Token<T> = token::Token<T, Pos>;

pub type RootTypeId = (Package, String);
pub type NestedTypeId = (Package, Vec<String>);

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: Token<String>,
    pub value: Token<Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub ty: Custom,
    pub arguments: Token<Vec<Token<FieldInit>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Type(Type),
    Instance(Token<Instance>),
    Constant(Token<Custom>),
    Array(Vec<Token<Value>>),
}

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<Token<Value>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Custom {
    pub prefix: Option<String>,
    pub parts: Vec<String>,
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
    Custom(Custom),
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
}

impl ::std::fmt::Display for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Type::Double => write!(f, "double"),
            Type::Float => write!(f, "float"),
            Type::Signed(ref size) => {
                if let Some(size) = *size {
                    write!(f, "signed/{}", size)
                } else {
                    write!(f, "signed")
                }
            }
            Type::Unsigned(ref size) => {
                if let Some(size) = *size {
                    write!(f, "unsigned/{}", size)
                } else {
                    write!(f, "unsigned")
                }
            }
            Type::Boolean => write!(f, "boolean"),
            Type::String => write!(f, "string"),
            Type::Custom(ref custom) => {
                if let Some(ref used) = custom.prefix {
                    write!(f, "{}::{}", used, custom.parts.join("."))
                } else {
                    write!(f, "{}", custom.parts.join("."))
                }
            }
            Type::Array(ref inner) => write!(f, "[{}]", inner),
            Type::Map(ref key, ref value) => write!(f, "{{{}: {}}}", key, value),
            Type::Any => write!(f, "any"),
            Type::Bytes => write!(f, "bytes"),
        }
    }
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
    pub field_as: Option<Token<String>>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            Modifier::Optional => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &str {
        if let Some(ref field) = self.field_as {
            &field.inner
        } else {
            &self.name
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
    pub fn name(&self) -> String {
        self.names
            .iter()
            .map(|t| t.inner.to_owned())
            .nth(0)
            .unwrap_or_else(|| self.name.clone())
    }

    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Token<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceBody {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub match_decl: MatchDecl,
    pub sub_types: BTreeMap<String, Token<Rc<SubType>>>,
}

impl InterfaceBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Token<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct TypeBody {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub match_decl: MatchDecl,
    // Set of fields which are reserved for this type.
    pub reserved: HashSet<Token<String>>,
}

impl TypeBody {
    pub fn verify(&self) -> Result<()> {
        for reserved in &self.reserved {
            if let Some(field) = self.fields.iter().find(|f| f.name == reserved.inner) {
                return Err(Error::reserved_field(field.pos.clone(), reserved.pos.clone()));
            }
        }

        Ok(())
    }

    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Token<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct TupleBody {
    pub name: String,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub match_decl: MatchDecl,
}

impl TupleBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Token<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: Token<String>,
    pub arguments: Vec<Token<Value>>,
    pub ordinal: u32,
}

#[derive(Debug, Clone)]
pub struct EnumBody {
    pub name: String,
    pub values: Vec<Token<Rc<EnumValue>>>,
    pub fields: Vec<Token<Field>>,
    pub codes: Vec<Token<Code>>,
    pub match_decl: MatchDecl,
    pub serialized_as: Option<Token<String>>,
    pub serialized_as_name: bool,
}

#[derive(Debug, Clone)]
pub enum Registered {
    Type(Rc<TypeBody>),
    Interface(Rc<InterfaceBody>),
    Enum(Rc<EnumBody>),
    Tuple(Rc<TupleBody>),
    SubType {
        parent: Rc<InterfaceBody>,
        sub_type: Rc<SubType>,
    },
    EnumConstant {
        parent: Rc<EnumBody>,
        value: Rc<EnumValue>,
    },
}

impl Registered {
    pub fn fields<'a>(&'a self) -> Result<Box<Iterator<Item = &Token<Field>> + 'a>> {
        let it: Box<Iterator<Item = &Token<Field>>> = match *self {
            Registered::Type(ref body) => Box::new(body.fields.iter()),
            Registered::Tuple(ref body) => Box::new(body.fields.iter()),
            Registered::SubType { ref parent, ref sub_type } => {
                Box::new(parent.fields.iter().chain(sub_type.fields.iter()))
            }
            _ => {
                return Err("has no fields".into());
            }
        };

        Ok(it)
    }

    pub fn find_field(&self, name: &str) -> Result<Option<&Token<Field>>> {
        for field in self.fields()? {
            if field.name == name {
                return Ok(Some(field));
            }
        }

        Ok(None)
    }

    pub fn is_assignable_from(&self, other: &Registered) -> bool {
        match *self {
            Registered::Type(ref body) => {
                // Check if type equals.
                if let Registered::Type(ref other) = *other {
                    return Rc::ptr_eq(body, other);
                }
            }
            Registered::Tuple(ref body) => {
                // Check if tuple equals.
                if let Registered::Tuple(ref other) = *other {
                    return Rc::ptr_eq(body, other);
                }
            }
            Registered::Interface(ref interface) => {
                // Check if implementation is interface.
                if let Registered::SubType { ref parent, sub_type: _ } = *other {
                    return Rc::ptr_eq(interface, parent);
                }
            }
            Registered::Enum(ref en) => {
                // Check if constant is contained in enum
                if let Registered::EnumConstant { ref parent, value: _ } = *other {
                    return Rc::ptr_eq(en, parent);
                }
            }
            _ => {}
        }

        false
    }

    pub fn display(&self) -> String {
        match *self {
            Registered::Type(ref body) => format!("type {}", body.name.to_owned()),
            Registered::Interface(ref body) => format!("interface {}", body.name.to_owned()),
            Registered::Enum(ref body) => format!("enum {}", body.name.to_owned()),
            Registered::Tuple(ref body) => format!("tuple {}", body.name.to_owned()),
            Registered::SubType { ref parent, ref sub_type } => {
                format!("type {}.{}", parent.name, sub_type.name)
            }
            Registered::EnumConstant { ref parent, ref value } => {
                format!("{}.{}", parent.name, *value.name)
            }
        }
    }
}

#[derive(Clone)]
pub enum Decl {
    Type(Rc<TypeBody>),
    Interface(Rc<InterfaceBody>),
    Enum(Rc<EnumBody>),
    Tuple(Rc<TupleBody>),
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

/// Simplified types that _can_ be uniquely matched over.
#[derive(Debug, PartialEq, Clone)]
pub enum MatchKind {
    Any,
    Object,
    Array,
    String,
    Boolean,
    Number,
}

#[derive(Debug, Clone)]
pub enum MatchCondition {
    /// Match a specific value.
    Value(Token<Value>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(MatchVariable),
}

#[derive(Debug, Clone)]
pub struct MatchMember {
    pub condition: Token<MatchCondition>,
    pub value: Token<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchVariable {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct MatchDecl {
    pub by_value: Vec<(Token<Value>, Token<MatchMember>)>,
    pub by_type: Vec<(MatchKind, Token<MatchMember>)>,
}

impl MatchDecl {
    pub fn new() -> MatchDecl {
        MatchDecl {
            by_value: Vec::new(),
            by_type: Vec::new(),
        }
    }

    pub fn identify_match_kind(&self, variable: &MatchVariable) -> MatchKind {
        match variable.ty {
            Type::Double |
            Type::Float |
            Type::Signed(_) |
            Type::Unsigned(_) => MatchKind::Number,
            Type::Boolean => MatchKind::Boolean,
            Type::String | Type::Bytes => MatchKind::String,
            Type::Any => MatchKind::Any,
            Type::Custom(_) | Type::Map(_, _) => MatchKind::Object,
            Type::Array(_) => MatchKind::Array,
        }
    }

    pub fn push(&mut self, member: Token<MatchMember>) -> Result<()> {
        match member.condition.inner {
            MatchCondition::Type(ref variable) => {
                let match_kind = self.identify_match_kind(variable);

                {
                    // conflicting when type matches
                    let result =
                        self.by_type.iter().find(|e| e.0 == match_kind || e.0 == MatchKind::Any);

                    if let Some(&(_, ref existing_value)) = result {
                        let err = ErrorKind::MatchConflict(member.condition.pos.clone(),
                                                           existing_value.condition.pos.clone());
                        return Err(err.into());
                    }
                }

                self.by_type.push((match_kind, member.clone()));
            }
            MatchCondition::Value(ref value) => {
                {
                    // conflicting when value matches
                    let result = self.by_value.iter().find(|e| e.0.inner == value.inner);

                    if let Some(&(_, ref existing_value)) = result {
                        let err = ErrorKind::MatchConflict(member.condition.pos.clone(),
                                                           existing_value.condition.pos.clone());
                        return Err(err.into());
                    }
                }

                self.by_value.push((value.clone(), member.clone()));
            }
        }

        Ok(())
    }
}
