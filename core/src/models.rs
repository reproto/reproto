//! Data Models for the final model stage stage.

use super::loc::Loc;
use super::option_entry::OptionEntry;
use super::rp_modifier::RpModifier;
use super::rp_number::RpNumber;
use super::rp_versioned_package::RpVersionedPackage;
use errors::*;
use linked_hash_map::LinkedHashMap;
use pos::Pos;
use std::collections::{BTreeMap, HashSet, LinkedList};
use std::fmt;
use std::rc::Rc;
use std::result;
use std::slice;

/// Build a declaration body including common fields.
macro_rules! decl_body {
    (pub struct $name:ident { $($rest:tt)* }) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name {
            pub name: RpName,
            pub local_name: String,
            pub comment: Vec<String>,
            pub decls: Vec<Rc<Loc<RpDecl>>>,
            $($rest)*
        }
    };
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpDecl {
    Type(Rc<Loc<RpTypeBody>>),
    Tuple(Rc<Loc<RpTupleBody>>),
    Interface(Rc<Loc<RpInterfaceBody>>),
    Enum(Rc<Loc<RpEnumBody>>),
    Service(Rc<Loc<RpServiceBody>>),
}

impl RpDecl {
    pub fn decls(&self) -> DeclIter {
        use self::RpDecl::*;

        let iter = match *self {
            Type(ref body) => body.decls.iter(),
            Interface(ref body) => body.decls.iter(),
            Enum(ref body) => body.decls.iter(),
            Tuple(ref body) => body.decls.iter(),
            Service(ref body) => body.decls.iter(),
        };

        DeclIter { iter: iter }
    }

    pub fn local_name(&self) -> &str {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => body.local_name.as_str(),
            Interface(ref body) => body.local_name.as_str(),
            Enum(ref body) => body.local_name.as_str(),
            Tuple(ref body) => body.local_name.as_str(),
            Service(ref body) => body.local_name.as_str(),
        }
    }

    pub fn name(&self) -> &RpName {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => &body.name,
            Interface(ref body) => &body.name,
            Enum(ref body) => &body.name,
            Tuple(ref body) => &body.name,
            Service(ref body) => &body.name,
        }
    }

    pub fn comment(&self) -> &[String] {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => &body.comment,
            Interface(ref body) => &body.comment,
            Enum(ref body) => &body.comment,
            Tuple(ref body) => &body.comment,
            Service(ref body) => &body.comment,
        }
    }

    /// Convert a declaration into its registered types.
    pub fn into_registered_type(&self) -> Vec<RpRegistered> {
        use self::RpDecl::*;

        let mut out = Vec::new();

        match *self {
            Type(ref ty) => {
                out.push(RpRegistered::Type(ty.clone()));
            }
            Interface(ref interface) => {
                for sub_type in interface.sub_types.values() {
                    out.push(RpRegistered::SubType(interface.clone(), sub_type.clone()));

                    for d in &sub_type.decls {
                        out.extend(d.into_registered_type());
                    }
                }

                out.push(RpRegistered::Interface(interface.clone()));
            }
            Enum(ref en) => {
                for variant in &en.variants {
                    out.push(RpRegistered::EnumVariant(en.clone(), variant.clone()));
                }

                out.push(RpRegistered::Enum(en.clone()));
            }
            Tuple(ref tuple) => {
                out.push(RpRegistered::Tuple(tuple.clone()));
            }
            Service(ref service) => {
                out.push(RpRegistered::Service(service.clone()));
            }
        }

        for d in self.decls() {
            out.extend(d.into_registered_type());
        }

        out
    }

    /// Get stringy kind of the declaration.
    pub fn kind(&self) -> &str {
        use self::RpDecl::*;

        match *self {
            Type(_) => "type",
            Interface(_) => "interface",
            Enum(_) => "enum",
            Tuple(_) => "tuple",
            Service(_) => "service",
        }
    }
}

#[derive(Debug, Clone)]
pub enum RpRegistered {
    Type(Rc<Loc<RpTypeBody>>),
    Tuple(Rc<Loc<RpTupleBody>>),
    Interface(Rc<Loc<RpInterfaceBody>>),
    SubType(Rc<Loc<RpInterfaceBody>>, Rc<Loc<RpSubType>>),
    Enum(Rc<Loc<RpEnumBody>>),
    EnumVariant(Rc<Loc<RpEnumBody>>, Rc<Loc<RpVariant>>),
    Service(Rc<Loc<RpServiceBody>>),
}

#[derive(Debug, Clone, Serialize)]
pub struct RpCode {
    pub context: String,
    pub lines: Vec<String>,
}

pub struct DeclIter<'a> {
    iter: slice::Iter<'a, Rc<Loc<RpDecl>>>,
}

impl<'a> Iterator for DeclIter<'a> {
    type Item = &'a Rc<Loc<RpDecl>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl fmt::Display for RpDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpDecl::*;

        match *self {
            Type(ref body) => write!(f, "type {}", body.name),
            Interface(ref body) => write!(f, "interface {}", body.name),
            Enum(ref body) => write!(f, "enum {}", body.name),
            Tuple(ref body) => write!(f, "tuple {}", body.name),
            Service(ref body) => write!(f, "service {}", body.name),
        }
    }
}

decl_body!(pub struct RpEnumBody {
    /// The type of the variant.
    pub variant_type: RpEnumType,
    pub variants: Vec<Rc<Loc<RpVariant>>>,
    pub codes: Vec<Loc<RpCode>>,
});

#[derive(Debug, Clone, Serialize)]
pub struct RpVariant {
    pub name: RpName,
    pub local_name: Loc<String>,
    pub comment: Vec<String>,
    pub ordinal: RpEnumOrdinal,
}

impl RpVariant {
    pub fn ordinal(&self) -> &str {
        use self::RpEnumOrdinal::*;

        match self.ordinal {
            String(ref string) => string.as_str(),
            Generated => self.local_name.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum RpEnumType {
    String,
    Generated,
}

impl RpEnumType {
    pub fn is_assignable_from(&self, value: &RpValue) -> bool {
        use self::RpEnumType::*;

        match (self, value) {
            (&String, &RpValue::String(_)) => true,
            _ => false,
        }
    }

    pub fn as_type(&self) -> RpType {
        use self::RpEnumType::*;

        match *self {
            String => RpType::String,
            Generated => RpType::String,
        }
    }

    pub fn as_field(&self) -> RpField {
        RpField {
            modifier: RpModifier::Required,
            name: String::from("value"),
            comment: vec![],
            ty: self.as_type(),
            field_as: None,
        }
    }
}

impl fmt::Display for RpEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpEnumType::*;

        match *self {
            String => write!(f, "string"),
            Generated => write!(f, "generated"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum RpEnumOrdinal {
    /// Value is specified expliticly.
    String(String),
    /// Value is automatically derived from the name of the variant.
    Generated,
}

#[derive(Debug, Clone, Serialize)]
pub struct RpField {
    pub modifier: RpModifier,
    pub name: String,
    pub comment: Vec<String>,
    #[serde(rename = "type")]
    pub ty: RpType,
    /// Alias of field in JSON.
    pub field_as: Option<String>,
}

impl RpField {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }

    pub fn is_required(&self) -> bool {
        !self.is_optional()
    }

    pub fn ident(&self) -> &str {
        &self.name
    }

    pub fn name(&self) -> &str {
        self.field_as.as_ref().unwrap_or(&self.name)
    }

    pub fn display(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RpFile {
    pub comment: Vec<String>,
    pub options: Vec<Loc<RpOptionDecl>>,
    pub decls: Vec<Rc<Loc<RpDecl>>>,
}

/// Iterator over all declarations in a file.
pub struct ForEachDecl<'a> {
    queue: LinkedList<&'a Rc<Loc<RpDecl>>>,
}

impl<'a> Iterator for ForEachDecl<'a> {
    type Item = &'a Rc<Loc<RpDecl>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(decl) = self.queue.pop_front() {
            self.queue.extend(decl.decls());
            Some(decl)
        } else {
            None
        }
    }
}

impl RpFile {
    /// Iterate over all declarations in file.
    pub fn for_each_decl(&self) -> ForEachDecl {
        let mut queue = LinkedList::new();
        queue.extend(self.decls.iter());
        ForEachDecl { queue: queue }
    }
}

decl_body!(pub struct RpInterfaceBody {
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    pub sub_types: BTreeMap<String, Rc<Loc<RpSubType>>>,
});

impl RpInterfaceBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Loc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpName {
    /// Alias used if the name was imported from another package.
    pub prefix: Option<String>,
    /// Package that name belongs to.
    pub package: RpVersionedPackage,
    /// Absolute parts of the name, from the root of the package.
    pub parts: Vec<String>,
}

impl RpName {
    pub fn new(prefix: Option<String>, package: RpVersionedPackage, parts: Vec<String>) -> RpName {
        RpName {
            prefix: prefix,
            package: package,
            parts: parts,
        }
    }

    pub fn extend<I>(&self, it: I) -> RpName
    where
        I: IntoIterator<Item = String>,
    {
        let mut parts = self.parts.clone();
        parts.extend(it);

        RpName {
            prefix: self.prefix.clone(),
            package: self.package.clone(),
            parts: parts,
        }
    }

    pub fn push(&self, part: String) -> RpName {
        let mut parts = self.parts.clone();
        parts.push(part);

        RpName {
            prefix: self.prefix.clone(),
            package: self.package.clone(),
            parts: parts,
        }
    }

    pub fn join<S: AsRef<str>>(&self, joiner: S) -> String {
        self.parts.join(joiner.as_ref())
    }

    /// Convert to a name without a prefix component.
    pub fn without_prefix(self) -> RpName {
        RpName {
            prefix: None,
            package: self.package,
            parts: self.parts,
        }
    }

    /// Localize name.
    ///
    /// Strips version of any type which is _not_ imported.
    pub fn localize(self) -> RpName {
        if self.prefix.is_some() {
            return self;
        }

        self.without_version()
    }

    /// Convert to a name without a version component.
    pub fn without_version(self) -> RpName {
        RpName {
            prefix: self.prefix,
            package: self.package.without_version(),
            parts: self.parts,
        }
    }

    pub fn with_package(self, package: RpVersionedPackage) -> RpName {
        RpName {
            prefix: self.prefix,
            package: package,
            parts: self.parts,
        }
    }

    /// Build a new name out if the given paths.
    pub fn with_parts(self, parts: Vec<String>) -> RpName {
        RpName {
            prefix: self.prefix,
            package: self.package,
            parts: parts,
        }
    }

    pub fn is_same(&self, other: &RpName) -> bool {
        self.package == other.package && self.parts == other.parts
    }
}

impl fmt::Display for RpName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref prefix) = self.prefix {
            write!(f, "{}::{}", prefix, self.parts.join("::"))
        } else {
            write!(f, "{}", self.parts.join("::"))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RpOptionDecl {
    pub name: String,
    pub value: Loc<RpValue>,
}

impl OptionEntry for RpOptionDecl {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_string(&self) -> result::Result<String, &'static str> {
        match *self.value.value() {
            RpValue::String(ref string) => Ok(string.to_string()),
            _ => Err("expected string"),
        }
    }

    fn as_number(&self) -> result::Result<RpNumber, &'static str> {
        match *self.value.value() {
            RpValue::Number(ref number) => Ok(number.clone()),
            _ => Err("expected number"),
        }
    }

    fn as_identifier(&self) -> result::Result<String, &'static str> {
        match *self.value.value() {
            RpValue::Identifier(ref identifier) => Ok(identifier.to_string()),
            _ => Err("expected identifier"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpPathSegment {
    Literal { value: Loc<String> },
    Variable { name: Loc<String>, ty: Loc<RpType> },
}

impl RpPathSegment {
    pub fn path(&self) -> String {
        match *self {
            RpPathSegment::Literal { ref value } => value.value().to_owned(),
            RpPathSegment::Variable { ref name, .. } => format!("{{{}}}", name.value()),
        }
    }

    pub fn id(&self) -> &str {
        match *self {
            RpPathSegment::Literal { ref value } => value.value().as_str(),
            RpPathSegment::Variable { ref name, .. } => name.value().as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RpPathSpec {
    pub segments: Vec<RpPathSegment>,
}

impl RpPathSpec {
    pub fn url(&self) -> String {
        let segments: Vec<String> = self.segments.iter().map(RpPathSegment::path).collect();
        format!("/{}", segments.join("/"))
    }

    pub fn id_fragments(&self) -> Vec<&str> {
        self.segments.iter().map(RpPathSegment::id).collect()
    }
}

impl RpRegistered {
    pub fn name(&self) -> &RpName {
        use self::RpRegistered::*;

        match *self {
            Type(ref target) => &target.name,
            Tuple(ref target) => &target.name,
            Service(ref target) => &target.name,
            Interface(ref target) => &target.name,
            Enum(ref target) => &target.name,
            SubType(_, ref target) => &target.name,
            EnumVariant(_, ref target) => &target.name,
        }
    }

    /// Get the location of the registered declaration.
    pub fn pos(&self) -> &Pos {
        use self::RpRegistered::*;

        match *self {
            Type(ref target) => target.pos(),
            Tuple(ref target) => target.pos(),
            Service(ref target) => target.pos(),
            Interface(ref target) => target.pos(),
            Enum(ref target) => target.pos(),
            SubType(_, ref target) => target.pos(),
            EnumVariant(_, ref target) => target.pos(),
        }
    }

    pub fn is_assignable_from(&self, other: &RpRegistered) -> bool {
        use self::RpRegistered::*;

        match (self, other) {
            // same type
            (&Type(ref s), &Type(ref o)) => Rc::ptr_eq(s, o),
            // tuple of same type
            (&Tuple(ref s), &Tuple(ref o)) => Rc::ptr_eq(s, o),
            // interface of same type
            (&Interface(ref s), &Interface(ref o)) => Rc::ptr_eq(s, o),
            // sub type of a given interfacce
            (&Interface(ref s), &SubType(ref o, _)) => Rc::ptr_eq(s, o),
            // exact sub type match
            (&SubType(_, ref s), &SubType(_, ref o)) => Rc::ptr_eq(s, o),
            // enum of same type
            (&Enum(ref s), &Enum(ref o)) => Rc::ptr_eq(s, o),
            // variant of a given enum
            (&Enum(ref s), &EnumVariant(ref o, _)) => Rc::ptr_eq(s, o),
            // exact variant match
            (&EnumVariant(_, ref s), &EnumVariant(_, ref o)) => Rc::ptr_eq(s, o),
            // same service
            (&Service(ref s), &Service(ref o)) => Rc::ptr_eq(s, o),
            _ => false,
        }
    }

    pub fn fields<'a>(&'a self) -> Result<Box<Iterator<Item = &Loc<RpField>> + 'a>> {
        use self::RpRegistered::*;

        let fields: Box<Iterator<Item = &Loc<RpField>>> = match *self {
            Type(ref target) => Box::new(target.fields.iter()),
            Tuple(ref target) => Box::new(target.fields.iter()),
            Interface(ref target) => Box::new(target.fields.iter()),
            SubType(ref parent, ref target) => {
                Box::new(parent.fields.iter().chain(target.fields.iter()))
            }
            _ => {
                return Err(
                    format!("{}: type doesn't have fields", self.display()).into(),
                )
            }
        };

        Ok(fields)
    }

    pub fn display(&self) -> String {
        use self::RpRegistered::*;

        match *self {
            Type(ref body) => format!("type {}", body.name),
            Interface(ref body) => format!("interface {}", body.name),
            Enum(ref body) => format!("enum {}", body.name),
            Tuple(ref body) => format!("tuple {}", body.name),
            Service(ref body) => format!("service {}", body.name),
            SubType(_, ref sub_type) => format!("subtype {}", sub_type.name),
            EnumVariant(_, ref variant) => format!("variant {}", variant.name),
        }
    }

    pub fn local_name<PackageFn, InnerFn>(
        &self,
        name: &RpName,
        package_fn: PackageFn,
        inner_fn: InnerFn,
    ) -> String
    where
        PackageFn: Fn(Vec<&str>) -> String,
        InnerFn: Fn(Vec<&str>) -> String,
    {
        use self::RpRegistered::*;

        match *self {
            Type(_) | Interface(_) | Enum(_) | Tuple(_) | Service(_) => {
                let p = name.parts.iter().map(String::as_str).collect();
                package_fn(p)
            }
            SubType { .. } |
            EnumVariant { .. } => {
                let mut v: Vec<&str> = name.parts.iter().map(String::as_str).collect();
                let at = v.len().saturating_sub(2);
                let last = inner_fn(v.split_off(at));

                let mut parts = v.clone();
                parts.push(last.as_str());

                inner_fn(parts)
            }
        }
    }

    /// Get stringy kind of the registered type, if applicable.
    ///
    /// This returns the base kind as the first member of the tuple.
    /// Then the registered type as the second (if applicable).
    pub fn kind(&self) -> (&str, Option<&RpRegistered>) {
        use self::RpRegistered::*;

        let result = match *self {
            Type(_) => "type",
            Interface(_) => "interface",
            Enum(_) => "enum",
            Tuple(_) => "tuple",
            Service(_) => "service",
            SubType(_, _) => return ("interface", Some(self)),
            EnumVariant(_, _) => return ("enum", Some(self)),
        };

        // simple case
        (result, None)
    }
}

decl_body!(pub struct RpServiceBody {
    pub endpoints: LinkedHashMap<String, Loc<RpEndpoint>>,
});

#[derive(Debug, Clone, Serialize)]
pub struct RpEndpoint {
    /// Name of the endpoint. Guaranteed to be unique.
    pub id: Loc<String>,
    /// Name of the endpoint. This is the name which is being sent over the wire.
    pub name: String,
    /// Comments for documentation.
    pub comment: Vec<String>,
    /// Request type that this endpoint expects.
    pub request: Option<Loc<RpChannel>>,
    /// Response type that this endpoint responds with.
    pub response: Option<Loc<RpChannel>>,
}

impl RpEndpoint {
    pub fn id_parts<F>(&self, filter: F) -> Vec<String>
    where
        F: Fn(&str) -> String,
    {
        vec![filter(self.id.as_str())]
    }

    /// Get the name of the endpoint.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum RpChannel {
    /// Single send.
    Unary { ty: RpType },
    /// Multiple sends.
    Streaming { ty: RpType },
}

impl RpChannel {
    /// Get the type of the channel.
    pub fn ty(&self) -> &RpType {
        use self::RpChannel::*;

        match *self {
            Unary { ref ty, .. } => ty,
            Streaming { ref ty, .. } => ty,
        }
    }

    /// Check if channel is streaming.
    pub fn is_streaming(&self) -> bool {
        use self::RpChannel::*;

        match *self {
            Unary { .. } => false,
            Streaming { .. } => true,
        }
    }
}

impl fmt::Display for RpChannel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.is_streaming() {
            write!(fmt, "stream {}", self.ty())
        } else {
            write!(fmt, "{}", self.ty())
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RpSubType {
    pub name: RpName,
    pub local_name: String,
    pub comment: Vec<String>,
    /// Inner declarations.
    pub decls: Vec<Rc<Loc<RpDecl>>>,
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    pub names: Vec<Loc<String>>,
}

impl RpSubType {
    pub fn name(&self) -> &str {
        self.names
            .iter()
            .map(|t| t.value().as_str())
            .nth(0)
            .unwrap_or(&self.local_name)
    }
}

decl_body!(pub struct RpTupleBody {
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
});

impl RpTupleBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Loc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

decl_body!(pub struct RpTypeBody {
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    // Set of fields which are reserved for this type.
    pub reserved: HashSet<Loc<String>>,
});

impl RpTypeBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Loc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpType {
    Double,
    Float,
    Signed { size: usize },
    Unsigned { size: usize },
    Boolean,
    String,
    /// ISO-8601 datetime
    DateTime,
    Bytes,
    Any,
    Name { name: RpName },
    Array { inner: Box<RpType> },
    Map {
        key: Box<RpType>,
        value: Box<RpType>,
    },
}

impl fmt::Display for RpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpType::*;

        match *self {
            Double => write!(f, "double"),
            Float => write!(f, "float"),
            Signed { ref size } => write!(f, "i{}", size),
            Unsigned { ref size } => write!(f, "u{}", size),
            Boolean => write!(f, "boolean"),
            String => write!(f, "string"),
            DateTime => write!(f, "datetime"),
            Name { ref name } => write!(f, "{}", name),
            Array { ref inner } => write!(f, "[{}]", inner),
            Map { ref key, ref value } => write!(f, "{{{}: {}}}", key, value),
            Any => write!(f, "any"),
            Bytes => write!(f, "bytes"),
        }
    }
}

impl RpType {
    /// Check if a given value is assignable to current type.
    pub fn is_assignable_from(&self, value: &RpValue) -> bool {
        use self::RpType::*;

        match (self, value) {
            (&Double, &RpValue::Number(_)) => true,
            (&Float, &RpValue::Number(_)) => true,
            (&Signed { .. }, &RpValue::Number(_)) => true,
            (&Unsigned { .. }, &RpValue::Number(_)) => true,
            (&Boolean, &RpValue::Boolean(_)) => true,
            (&String, &RpValue::String(_)) => true,
            (&Array { .. }, &RpValue::Array(_)) => true,
            _ => false,
        }
    }

    /// Convert to an enum variant type.
    pub fn as_enum_type(&self) -> Option<RpEnumType> {
        use self::RpType::*;

        match *self {
            String => Some(RpEnumType::String),
            _ => None,
        }
    }

    /// Localize type.
    ///
    /// Strips version of any type which is _not_ imported.
    pub fn localize(self) -> RpType {
        self.with_name(RpName::localize)
    }

    /// Strip version component for any type.
    pub fn without_version(self) -> RpType {
        self.with_name(RpName::without_version)
    }

    /// Modify any name components with the given operation.
    fn with_name<F>(self, f: F) -> RpType
    where
        F: Clone + Fn(RpName) -> RpName,
    {
        use self::RpType::*;

        match self {
            Name { name } => Name { name: f(name) },
            Array { inner } => Array { inner: Box::new(inner.with_name(f)) },
            Map { key, value } => Map {
                key: Box::new(key.with_name(f.clone())),
                value: Box::new(value.with_name(f.clone())),
            },
            ty => ty,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RpValue {
    String(String),
    Number(RpNumber),
    Boolean(bool),
    Identifier(String),
    Array(Vec<Loc<RpValue>>),
}

impl RpValue {
    pub fn as_str(&self) -> Result<&str> {
        use self::RpValue::*;

        match *self {
            String(ref string) => Ok(string),
            _ => Err("not a string".into()),
        }
    }

    pub fn as_identifier(&self) -> Result<&str> {
        use self::RpValue::*;

        match *self {
            String(ref string) => Ok(string),
            Identifier(ref identifier) => Ok(identifier),
            _ => Err("unsupported identifier kind".into()),
        }
    }

    pub fn to_ordinal(self) -> Result<RpEnumOrdinal> {
        let ordinal = match self {
            RpValue::String(value) => RpEnumOrdinal::String(value),
            _ => return Err(ErrorKind::InvalidOrdinal.into()),
        };

        Ok(ordinal)
    }
}

impl fmt::Display for RpValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            RpValue::String(_) => "<string>",
            RpValue::Number(_) => "<number>",
            RpValue::Boolean(_) => "<boolean>",
            RpValue::Identifier(_) => "<identifier>",
            RpValue::Array(_) => "<array>",
        };

        write!(f, "{}", out)
    }
}
