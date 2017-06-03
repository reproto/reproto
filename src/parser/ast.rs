pub use backend::models::{RpName, RpPackage, RpType, RpModifier};
use loc;

/// Position relative in file where the declaration is present.
pub type Pos = (usize, usize);
pub type AstLoc<T> = loc::Loc<T, Pos>;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: AstLoc<String>,
    pub value: AstLoc<RpValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub ty: RpName,
    pub arguments: AstLoc<Vec<AstLoc<FieldInit>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RpValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Type(RpType),
    Instance(AstLoc<Instance>),
    Constant(AstLoc<RpName>),
    Array(Vec<AstLoc<RpValue>>),
}

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<AstLoc<RpValue>>,
}

#[derive(Debug)]
pub struct Field {
    pub modifier: RpModifier,
    pub name: String,
    pub ty: RpType,
    pub field_as: Option<AstLoc<RpValue>>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Member {
    Field(Field),
    Code(String, Vec<String>),
    Option(AstLoc<OptionDecl>),
    Match(MatchDecl),
}

#[derive(Debug)]
pub struct MatchVariable {
    pub name: String,
    pub ty: RpType,
}

#[derive(Debug)]
pub enum MatchCondition {
    /// Match a specific value.
    RpValue(AstLoc<RpValue>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(AstLoc<MatchVariable>),
}

#[derive(Debug)]
pub struct MatchMember {
    pub condition: AstLoc<MatchCondition>,
    pub value: AstLoc<RpValue>,
}

#[derive(Debug)]
pub struct MatchDecl {
    pub members: Vec<AstLoc<MatchMember>>,
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

#[derive(Debug)]
pub struct TupleBody {
    pub name: String,
    pub members: Vec<AstLoc<Member>>,
}

#[derive(Debug)]
pub struct InterfaceBody {
    pub name: String,
    pub members: Vec<AstLoc<Member>>,
    pub sub_types: Vec<AstLoc<SubType>>,
}

#[derive(Debug)]
pub struct TypeBody {
    pub name: String,
    pub members: Vec<AstLoc<Member>>,
}

/// Sub-types in interface declarations.
#[derive(Debug)]
pub struct SubType {
    pub name: String,
    pub members: Vec<AstLoc<Member>>,
}

#[derive(Debug)]
pub struct EnumBody {
    pub name: String,
    pub values: Vec<AstLoc<EnumValue>>,
    pub members: Vec<AstLoc<Member>>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub name: AstLoc<String>,
    pub arguments: Vec<AstLoc<RpValue>>,
    pub ordinal: Option<AstLoc<RpValue>>,
}

#[derive(Debug)]
pub enum Decl {
    Type(TypeBody),
    Tuple(TupleBody),
    Interface(InterfaceBody),
    Enum(EnumBody),
}

impl Decl {
    pub fn name(&self) -> String {
        match *self {
            Decl::Interface(ref interface) => interface.name.clone(),
            Decl::Type(ref ty) => ty.name.clone(),
            Decl::Tuple(ref ty) => ty.name.clone(),
            Decl::Enum(ref ty) => ty.name.clone(),
        }
    }

    pub fn display(&self) -> String {
        match *self {
            Decl::Interface(ref body) => format!("interface {}", body.name),
            Decl::Type(ref body) => format!("type {}", body.name),
            Decl::Tuple(ref body) => format!("tuple {}", body.name),
            Decl::Enum(ref body) => format!("enum {}", body.name),
        }
    }
}

#[derive(Debug)]
pub struct UseDecl {
    pub package: AstLoc<RpPackage>,
    pub alias: Option<String>,
}

#[derive(Debug)]
pub struct File {
    pub package: AstLoc<RpPackage>,
    pub uses: Vec<AstLoc<UseDecl>>,
    pub decls: Vec<AstLoc<Decl>>,
}
