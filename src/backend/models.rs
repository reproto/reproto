pub use loc::WithPrefix;
use std::collections::{BTreeMap, HashSet, HashMap};
use std::path::PathBuf;
use std::rc::Rc;
use super::errors::*;
use loc;

pub type Pos = (PathBuf, usize, usize);
pub type RpLoc<T> = loc::Loc<T, Pos>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId {
    pub package: RpPackage,
    pub name: RpName,
}

impl TypeId {
    pub fn new(package: RpPackage, name: RpName) -> TypeId {
        TypeId {
            package: package,
            name: name,
        }
    }

    pub fn with_name(&self, name: RpName) -> TypeId {
        TypeId {
            package: self.package.clone(),
            name: name,
        }
    }

    pub fn extend(&self, part: String) -> TypeId {
        TypeId {
            package: self.package.clone(),
            name: self.name.extend(part),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: RpLoc<String>,
    pub value: RpLoc<RpValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub ty: RpName,
    pub arguments: RpLoc<Vec<RpLoc<FieldInit>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RpValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Type(RpType),
    Instance(RpLoc<Instance>),
    Constant(RpLoc<RpName>),
    Array(Vec<RpLoc<RpValue>>),
}

impl ::std::fmt::Display for RpValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let out = match *self {
            RpValue::String(_) => "<string>",
            RpValue::Number(_) => "<number>",
            RpValue::Boolean(_) => "<boolean>",
            RpValue::Identifier(_) => "<identifier>",
            RpValue::Type(_) => "<type>",
            RpValue::Instance(_) => "<instance>",
            RpValue::Constant(_) => "<constant>",
            RpValue::Array(_) => "<array>",
        };

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<RpLoc<RpValue>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RpName {
    pub prefix: Option<String>,
    pub parts: Vec<String>,
}

impl RpName {
    pub fn with_parts(parts: Vec<String>) -> RpName {
        RpName {
            prefix: None,
            parts: parts,
        }
    }

    pub fn extend(&self, part: String) -> RpName {
        let mut parts = self.parts.clone();
        parts.push(part);

        RpName {
            prefix: self.prefix.clone(),
            parts: parts,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RpType {
    Double,
    Float,
    Signed(Option<usize>),
    Unsigned(Option<usize>),
    Boolean,
    String,
    Bytes,
    Any,
    Name(RpName),
    Array(Box<RpType>),
    Map(Box<RpType>, Box<RpType>),
}

impl ::std::fmt::Display for RpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RpType::Double => write!(f, "double"),
            RpType::Float => write!(f, "float"),
            RpType::Signed(ref size) => {
                if let Some(size) = *size {
                    write!(f, "signed/{}", size)
                } else {
                    write!(f, "signed")
                }
            }
            RpType::Unsigned(ref size) => {
                if let Some(size) = *size {
                    write!(f, "unsigned/{}", size)
                } else {
                    write!(f, "unsigned")
                }
            }
            RpType::Boolean => write!(f, "boolean"),
            RpType::String => write!(f, "string"),
            RpType::Name(ref name) => {
                if let Some(ref used) = name.prefix {
                    write!(f, "{}::{}", used, name.parts.join("."))
                } else {
                    write!(f, "{}", name.parts.join("."))
                }
            }
            RpType::Array(ref inner) => write!(f, "[{}]", inner),
            RpType::Map(ref key, ref value) => write!(f, "{{{}: {}}}", key, value),
            RpType::Any => write!(f, "any"),
            RpType::Bytes => write!(f, "bytes"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RpPackage {
    pub parts: Vec<String>,
}

impl RpPackage {
    pub fn new(parts: Vec<String>) -> RpPackage {
        RpPackage { parts: parts }
    }

    pub fn join(&self, other: &RpPackage) -> RpPackage {
        let mut parts = self.parts.clone();
        parts.extend(other.parts.clone());
        RpPackage::new(parts)
    }

    pub fn into_type_id(&self, name: &RpName) -> TypeId {
        TypeId::new(self.clone(), name.clone())
    }
}

impl ::std::fmt::Display for RpPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RpModifier {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RpModifiers {
    modifiers: HashSet<RpModifier>,
}

impl RpModifiers {
    pub fn new(modifiers: HashSet<RpModifier>) -> RpModifiers {
        RpModifiers { modifiers: modifiers }
    }

    pub fn test(&self, modifier: &RpModifier) -> bool {
        self.modifiers.contains(modifier)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub modifier: RpModifier,
    pub name: String,
    pub ty: RpType,
    pub field_as: Option<RpLoc<String>>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
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
    fn fields(&self) -> &Vec<RpLoc<Field>>;
    fn codes(&self) -> &Vec<RpLoc<Code>>;
}

impl BodyLike for InterfaceBody {
    fn fields(&self) -> &Vec<RpLoc<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<RpLoc<Code>> {
        &self.codes
    }
}

impl BodyLike for TypeBody {
    fn fields(&self) -> &Vec<RpLoc<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<RpLoc<Code>> {
        &self.codes
    }
}

impl BodyLike for EnumBody {
    fn fields(&self) -> &Vec<RpLoc<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<RpLoc<Code>> {
        &self.codes
    }
}

impl BodyLike for TupleBody {
    fn fields(&self) -> &Vec<RpLoc<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<RpLoc<Code>> {
        &self.codes
    }
}

impl BodyLike for SubType {
    fn fields(&self) -> &Vec<RpLoc<Field>> {
        &self.fields
    }

    fn codes(&self) -> &Vec<RpLoc<Code>> {
        &self.codes
    }
}

#[derive(Debug, Clone)]
pub struct SubType {
    pub name: String,
    pub fields: Vec<RpLoc<Field>>,
    pub codes: Vec<RpLoc<Code>>,
    pub names: Vec<RpLoc<String>>,
}

impl SubType {
    pub fn name(&self) -> String {
        self.names
            .iter()
            .map(|t| t.inner.to_owned())
            .nth(0)
            .unwrap_or_else(|| self.name.clone())
    }

    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &RpLoc<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceBody {
    pub name: String,
    pub fields: Vec<RpLoc<Field>>,
    pub codes: Vec<RpLoc<Code>>,
    pub match_decl: MatchDecl,
    pub sub_types: BTreeMap<String, RpLoc<Rc<SubType>>>,
}

impl InterfaceBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &RpLoc<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct TypeBody {
    pub name: String,
    pub fields: Vec<RpLoc<Field>>,
    pub codes: Vec<RpLoc<Code>>,
    pub match_decl: MatchDecl,
    // Set of fields which are reserved for this type.
    pub reserved: HashSet<RpLoc<String>>,
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

    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &RpLoc<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct TupleBody {
    pub name: String,
    pub fields: Vec<RpLoc<Field>>,
    pub codes: Vec<RpLoc<Code>>,
    pub match_decl: MatchDecl,
}

impl TupleBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &RpLoc<Field>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: RpLoc<String>,
    pub arguments: Vec<RpLoc<RpValue>>,
    pub ordinal: u32,
}

#[derive(Debug, Clone)]
pub struct EnumBody {
    pub name: String,
    pub values: Vec<RpLoc<Rc<EnumValue>>>,
    pub fields: Vec<RpLoc<Field>>,
    pub codes: Vec<RpLoc<Code>>,
    pub match_decl: MatchDecl,
    pub serialized_as: Option<RpLoc<String>>,
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
    pub fn fields<'a>(&'a self) -> Result<Box<Iterator<Item = &RpLoc<Field>> + 'a>> {
        let it: Box<Iterator<Item = &RpLoc<Field>>> = match *self {
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

    pub fn find_field(&self, name: &str) -> Result<Option<&RpLoc<Field>>> {
        for field in self.fields()? {
            if field.name == name {
                return Ok(Some(field));
            }
        }

        Ok(None)
    }

    pub fn is_assignable_from(&self, other: &Registered) -> bool {
        match (self, other) {
            // exact type
            (&Registered::Type(ref target), &Registered::Type(ref source)) => {
                Rc::ptr_eq(target, source)
            }
            // exact tuple
            (&Registered::Tuple(ref target), &Registered::Tuple(ref source)) => {
                Rc::ptr_eq(target, source)
            }
            // exact interface, with unknown sub-type.
            (&Registered::Interface(ref target), &Registered::Interface(ref source)) => {
                Rc::ptr_eq(target, source)
            }
            // exact enum, with unknown value
            (&Registered::Enum(ref target), &Registered::Enum(ref source)) => {
                Rc::ptr_eq(target, source)
            }
            // sub-type to parent
            (&Registered::Interface(ref target),
             &Registered::SubType { parent: ref source, sub_type: _ }) => {
                Rc::ptr_eq(target, source)
            }
            // enum constant to parent type
            (&Registered::Enum(ref target),
             &Registered::EnumConstant { parent: ref source, value: _ }) => {
                Rc::ptr_eq(target, source)
            }
            // exact matching sub-type
            (&Registered::SubType { parent: ref target_parent, sub_type: ref target },
             &Registered::SubType { parent: ref source_parent, sub_type: ref source }) => {
                Rc::ptr_eq(target_parent, source_parent) && Rc::ptr_eq(target, source)
            }
            // exact matching constant
            (&Registered::EnumConstant { parent: ref target_parent, value: ref target },
             &Registered::EnumConstant { parent: ref source_parent, value: ref source }) => {
                Rc::ptr_eq(target_parent, source_parent) && Rc::ptr_eq(target, source)
            }
            _ => false,
        }
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
    RpValue(RpLoc<RpValue>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(RpLoc<MatchVariable>),
}

#[derive(Debug, Clone)]
pub struct MatchMember {
    pub condition: RpLoc<MatchCondition>,
    pub value: RpLoc<RpValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchVariable {
    pub name: String,
    pub ty: RpType,
}

#[derive(Debug, Clone)]
pub struct MatchDecl {
    pub by_value: Vec<(RpLoc<RpValue>, RpLoc<RpValue>)>,
    pub by_type: Vec<(MatchKind, (RpLoc<MatchVariable>, RpLoc<RpValue>))>,
}

impl MatchDecl {
    pub fn new() -> MatchDecl {
        MatchDecl {
            by_value: Vec::new(),
            by_type: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.by_value.is_empty() && self.by_type.is_empty()
    }

    pub fn identify_match_kind(&self, variable: &MatchVariable) -> MatchKind {
        match variable.ty {
            RpType::Double |
            RpType::Float |
            RpType::Signed(_) |
            RpType::Unsigned(_) => MatchKind::Number,
            RpType::Boolean => MatchKind::Boolean,
            RpType::String | RpType::Bytes => MatchKind::String,
            RpType::Any => MatchKind::Any,
            RpType::Name(_) |
            RpType::Map(_, _) => MatchKind::Object,
            RpType::Array(_) => MatchKind::Array,
        }
    }

    pub fn push(&mut self, member: RpLoc<MatchMember>) -> Result<()> {
        match member.condition.inner {
            MatchCondition::Type(ref variable) => {
                let match_kind = self.identify_match_kind(variable);

                {
                    // conflicting when type matches
                    let result =
                        self.by_type.iter().find(|e| e.0 == match_kind || e.0 == MatchKind::Any);

                    if let Some(&(_, ref existing_value)) = result {
                        let err = ErrorKind::MatchConflict(member.condition.pos.clone(),
                                                           existing_value.0.pos.clone());
                        return Err(err.into());
                    }
                }

                self.by_type.push((match_kind, (variable.clone(), member.value.clone())));
            }
            MatchCondition::RpValue(ref value) => {
                {
                    // conflicting when value matches
                    let result = self.by_value.iter().find(|e| e.0.inner == value.inner);

                    if let Some(&(_, ref existing_value)) = result {
                        let err = ErrorKind::MatchConflict(member.condition.pos.clone(),
                                                           existing_value.pos.clone());
                        return Err(err.into());
                    }
                }

                self.by_value.push((value.clone(), member.value.clone()));
            }
        }

        Ok(())
    }
}

pub struct Variables<'a> {
    variables: HashMap<String, &'a RpType>,
}

impl<'a> Variables<'a> {
    pub fn new() -> Variables<'a> {
        Variables { variables: HashMap::new() }
    }

    pub fn get(&self, key: &String) -> Option<&'a RpType> {
        self.variables.get(key).map(|t| *t)
    }

    pub fn insert(&mut self, key: String, value: &'a RpType) {
        self.variables.insert(key, value);
    }
}
