use backend::models as m;
use token;

/// Position relative in file where the declaration is present.
pub type Pos = (usize, usize);
pub type AstToken<T> = token::Token<T, Pos>;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldInit {
    pub name: AstToken<String>,
    pub value: AstToken<Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub ty: m::Custom,
    pub arguments: AstToken<Vec<AstToken<FieldInit>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Type(m::RpType),
    Instance(AstToken<Instance>),
    Constant(AstToken<m::Custom>),
    Array(Vec<AstToken<Value>>),
}

#[derive(Debug)]
pub struct OptionDecl {
    pub name: String,
    pub values: Vec<AstToken<Value>>,
}

#[derive(Debug)]
pub struct Field {
    pub modifier: m::RpModifier,
    pub name: String,
    pub ty: m::RpType,
    pub field_as: Option<AstToken<Value>>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            m::RpModifier::Optional => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Member {
    Field(Field),
    Code(String, Vec<String>),
    Option(AstToken<OptionDecl>),
    Match(MatchDecl),
}

#[derive(Debug)]
pub struct MatchVariable {
    pub name: String,
    pub ty: m::RpType,
}

#[derive(Debug)]
pub enum MatchCondition {
    /// Match a specific value.
    Value(AstToken<Value>),
    /// Match a type, and add a binding for the given name that can be resolved in the action.
    Type(AstToken<MatchVariable>),
}

#[derive(Debug)]
pub struct MatchMember {
    pub condition: AstToken<MatchCondition>,
    pub value: AstToken<Value>,
}

#[derive(Debug)]
pub struct MatchDecl {
    pub members: Vec<AstToken<MatchMember>>,
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
    pub members: Vec<AstToken<Member>>,
}

#[derive(Debug)]
pub struct InterfaceBody {
    pub name: String,
    pub members: Vec<AstToken<Member>>,
    pub sub_types: Vec<AstToken<SubType>>,
}

#[derive(Debug)]
pub struct TypeBody {
    pub name: String,
    pub members: Vec<AstToken<Member>>,
}

/// Sub-types in interface declarations.
#[derive(Debug)]
pub struct SubType {
    pub name: String,
    pub members: Vec<AstToken<Member>>,
}

#[derive(Debug)]
pub struct EnumBody {
    pub name: String,
    pub values: Vec<AstToken<EnumValue>>,
    pub members: Vec<AstToken<Member>>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub name: AstToken<String>,
    pub arguments: Vec<AstToken<Value>>,
    pub ordinal: Option<AstToken<Value>>,
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
    pub package: AstToken<m::Package>,
    pub alias: Option<String>,
}

#[derive(Debug)]
pub struct File {
    pub package: AstToken<m::Package>,
    pub uses: Vec<AstToken<UseDecl>>,
    pub decls: Vec<AstToken<Decl>>,
}
