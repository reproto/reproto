extern crate chrono;
#[macro_use]
extern crate genco;
extern crate inflector;
extern crate linked_hash_map;
extern crate reproto_core as core;
extern crate serde_json as json;

use chrono::{DateTime, Utc};
use core::{Loc, Object, Pos, RpDecl, RpField, RpInterfaceBody, RpModifier, RpName, RpPackage,
           RpSubType, RpSubTypeStrategy, RpTupleBody, RpType, RpTypeBody, RpVersionedPackage,
           DEFAULT_TAG};
use core::errors::Result;
use genco::{Custom, Formatter, IoFmt, Quoted, Tokens, WriteTokens};
use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::{self, Write};
use std::hash;
use std::io;
use std::mem;
use std::ops;
use std::rc::Rc;

/// Very simplified string escaping.
struct StringEscape<'a>(&'a str);

impl<'a> fmt::Display for StringEscape<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\"{}\"", self.0)
    }
}

impl<'a> StringEscape<'a> {
    fn new(content: &'a str) -> StringEscape<'a> {
        StringEscape(content)
    }
}

type TypesCache = HashMap<Fp, RpName>;

/// An opaque data structure, well all instances are equal but can contain different data.
#[derive(Debug, Clone)]
pub struct Opaque<T> {
    content: T,
}

impl<T> Opaque<T> {
    fn new(content: T) -> Self {
        Self { content: content }
    }
}

impl<T> cmp::PartialEq for Opaque<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> cmp::Eq for Opaque<T> {}

impl<T> hash::Hash for Opaque<T> {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> ops::Deref for Opaque<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T> ops::DerefMut for Opaque<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

/// The fingerprint of a field.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldFp {
    optional: bool,
    field: Fp,
}

impl FieldFp {
    /// Simplify the given fingerprint into something useful for looking up similar structures.
    fn hash(&self) -> FieldHashFp {
        FieldHashFp {
            field: self.field.hash(),
        }
    }

    fn refine(&mut self, other: &FieldFp) -> Result<()> {
        // relax optionality
        if !self.optional && other.optional {
            self.optional = true;
        }

        self.field.refine(&other.field)
    }
}

/// The fingerprint of a field.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldHashFp {
    field: HashFp,
}

/// Describes an interface sub-type.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SubTypeFp {
    /// Name of the sub-type.
    name: String,
    /// The structure of the sub-type, without the type-key.
    structure: LinkedHashMap<String, FieldFp>,
}

impl SubTypeFp {
    /// Simplify the given fingerprint into something useful for looking up similar structures.
    fn hash(&self) -> SubTypeHashFp {
        SubTypeHashFp {
            name: self.name.clone(),
            structure: self.structure
                .iter()
                .map(|(k, v)| (k.clone(), v.hash()))
                .collect(),
        }
    }
}

/// Describes an interface sub-type suitable for looking up similar structures.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SubTypeHashFp {
    /// Name of the sub-type.
    name: String,
    /// The structure of the sub-type, without the type-key.
    structure: LinkedHashMap<String, FieldHashFp>,
}

/// Fingerprint calculated from a JSON object.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Fp {
    U64(Opaque<Vec<u64>>),
    I64(Opaque<Vec<i64>>),
    Float,
    Double,
    Boolean,
    String(Opaque<Vec<String>>),
    DateTime(Opaque<Vec<String>>),
    Any,
    Object(LinkedHashMap<String, FieldFp>),
    Interface(String, Vec<SubTypeFp>),
    Array(Box<Fp>),
    Tuple(Vec<FieldFp>),
}

impl Fp {
    /// Recursively simplify this fingerprint into one that is only used for hashing.
    fn hash(&self) -> HashFp {
        match *self {
            Fp::U64(_)
            | Fp::I64(_)
            | Fp::Float
            | Fp::Double
            | Fp::Boolean
            | Fp::String(_)
            | Fp::DateTime(_)
            | Fp::Any => HashFp::Scalar,
            Fp::Object(ref fields) => {
                HashFp::Object(fields.iter().map(|(k, v)| (k.clone(), v.hash())).collect())
            }
            Fp::Interface(ref key, ref sub_types) => HashFp::Interface(
                key.to_string(),
                sub_types.iter().map(SubTypeFp::hash).collect(),
            ),
            Fp::Array(ref inner) => HashFp::Array(Box::new(inner.hash())),
            Fp::Tuple(ref inner) => HashFp::Tuple(inner.iter().map(FieldFp::hash).collect()),
        }
    }

    /// Check if this is an object.
    fn is_object(&self) -> bool {
        use self::Fp::*;

        match *self {
            Object(_) => true,
            _ => false,
        }
    }

    /// Check if this is a string.
    fn is_string(&self) -> bool {
        use self::Fp::*;

        match *self {
            String(_) => true,
            _ => false,
        }
    }

    /// Check if this is an object.
    fn as_object(&self) -> Option<&LinkedHashMap<String, FieldFp>> {
        use self::Fp::*;

        match *self {
            Object(ref object) => Some(object),
            _ => None,
        }
    }

    /// Refine this fingerprint with another.
    fn refine(&mut self, other: &Fp) -> Result<()> {
        // test for replacements
        let replace = match (&*self, other) {
            (&Fp::U64(ref examples), &Fp::I64(ref other)) => {
                let mut examples: Vec<_> = examples.iter().map(|v| *v as i64).collect();
                examples.extend(other.iter().cloned());
                Some(Fp::I64(Opaque::new(examples)))
            }
            (&Fp::Float, &Fp::Double) => Some(Fp::Double),
            (&Fp::U64(_), &Fp::Float) | (&Fp::I64(_), &Fp::Float) => Some(Fp::Float),
            (&Fp::U64(_), &Fp::Double) | (&Fp::I64(_), &Fp::Double) => Some(Fp::Double),
            // current value unknown, replace with other.
            (&Fp::Any, other) if *other != Fp::Any => Some(other.clone()),
            _ => None,
        };

        if let Some(replace) = replace {
            mem::replace(self, replace);
            return Ok(());
        }

        match (self, other) {
            (&mut Fp::U64(ref mut examples), &Fp::U64(ref other)) => {
                examples.extend(other.iter().cloned());
            }
            (&mut Fp::I64(ref mut examples), &Fp::I64(ref other)) => {
                examples.extend(other.iter().cloned());
            }
            // sign change
            (&mut Fp::Float, &Fp::Float) => {}
            (&mut Fp::Double, &Fp::Double) => {}
            (&mut Fp::Boolean, &Fp::Boolean) => {}
            (&mut Fp::String(ref mut examples), &Fp::String(ref other)) => {
                examples.extend(other.iter().cloned());
            }
            (&mut Fp::DateTime(ref mut examples), &Fp::DateTime(ref other)) => {
                examples.extend(other.iter().cloned());
            }
            (&mut Fp::Object(ref mut entries), &Fp::Object(ref other)) => {
                for ((kl, vl), (kr, vr)) in entries.iter_mut().zip(other.iter()) {
                    if kl != kr {
                        return Err(format!("Expected object key {}, but got {}", kl, kr).into());
                    }

                    vl.refine(vr)?;
                }
            }
            (&mut Fp::Interface(_, ref mut _entries), &Fp::Interface(_, ref _other)) => {}
            (&mut Fp::Array(ref mut inner), &Fp::Array(ref other)) => {
                return inner.refine(other);
            }
            (&mut Fp::Tuple(ref mut inner), &Fp::Tuple(ref other)) => {
                for (inner, other) in inner.iter_mut().zip(other.iter()) {
                    inner.refine(other)?;
                }
            }
            // Nothing to refine :(
            (_, &Fp::Any) => {}
            (current, other) => {
                return Err(format!("{:?} cannot be refined by {:?}", current, other).into());
            }
        }

        Ok(())
    }

    /// Calculate fingerprint from JSON value.
    fn from_json(value: &json::Value) -> Result<Fp> {
        let f = match *value {
            json::Value::Number(ref number) if number.is_u64() => {
                let number = number
                    .as_u64()
                    .ok_or_else(|| format!("Expected u64, got: {}", number))?;

                Fp::U64(Opaque::new(vec![number]))
            }
            json::Value::Number(ref number) if number.is_i64() => {
                let number = number
                    .as_i64()
                    .ok_or_else(|| format!("Expected i64, got: {}", number))?;

                Fp::I64(Opaque::new(vec![number]))
            }
            json::Value::Number(ref number) => {
                // Find best representation, float or double.

                let number = number
                    .as_f64()
                    .ok_or_else(|| format!("Expected f64, got: {}", number))?;

                let diff = (((number as f32) as f64) - number).abs();

                if diff != 0f64 {
                    Fp::Double
                } else {
                    Fp::Float
                }
            }
            json::Value::Bool(_) => Fp::Boolean,
            json::Value::String(ref string) => {
                let date_time = string.parse::<DateTime<Utc>>();

                if date_time.is_ok() {
                    Fp::DateTime(Opaque::new(vec![string.to_string()]))
                } else {
                    Fp::String(Opaque::new(vec![string.to_string()]))
                }
            }
            json::Value::Null => Fp::Any,
            json::Value::Array(ref array) => process_array(array)?,
            json::Value::Object(ref map) => {
                let mut entries = LinkedHashMap::new();

                for (key, value) in map {
                    let value = Fp::from_json(value)?;

                    let field = FieldFp {
                        optional: value == Fp::Any,
                        field: value,
                    };

                    entries.insert(key.to_string(), field);
                }

                Fp::Object(entries)
            }
        };

        return Ok(f);

        /// Process the given array.
        fn process_array(array: &[json::Value]) -> Result<Fp> {
            let mut children = Vec::new();
            let mut values = Vec::new();
            let mut unique: Vec<Fp> = Vec::new();

            for item in array {
                let f = Fp::from_json(item)?;

                let contained =
                    if let Some(current) = unique.iter_mut().find(|e| e.hash() == f.hash()) {
                        current.refine(&f)?;
                        true
                    } else {
                        false
                    };

                if !contained {
                    unique.push(f.clone());
                }

                children.push(f);
                values.push(item);
            }

            let mut it = unique.into_iter();

            if let Some(first) = it.next() {
                if it.next().is_some() {
                    if let Some(InterfaceTestResult { tag, sub_types }) =
                        test_interface(&children, &values)?
                    {
                        return Ok(Fp::Interface(tag, sub_types));
                    }

                    // All tuple fields are required.
                    let children = children
                        .into_iter()
                        .map(|f| FieldFp {
                            optional: false,
                            field: f,
                        })
                        .collect();

                    return Ok(Fp::Tuple(children));
                }

                return Ok(Fp::Array(Box::new(first)));
            }

            Ok(Fp::Array(Box::new(Fp::Any)))
        }

        /// Test if the given array is an interface.
        ///
        /// Interface detection happens by eliminating all common fields across a set of JSON
        /// fingerprints, and identifying if the common field can be used as an identifier.
        fn test_interface(
            children: &[Fp],
            values: &[&json::Value],
        ) -> Result<Option<InterfaceTestResult>> {
            // check if interface
            if !children.iter().all(Fp::is_object) {
                return Ok(None);
            }

            let children = children.iter().flat_map(Fp::as_object).collect::<Vec<_>>();
            let values = values
                .iter()
                .flat_map(|v| json_as_object(*v))
                .collect::<Vec<_>>();

            let mut common_keys = HashSet::new();
            let mut it = children.iter();

            if let Some(first) = it.next() {
                common_keys.extend(
                    first
                        .iter()
                        .filter(|&(_, v)| v.field.is_string())
                        .map(|(k, _)| k.clone()),
                );

                while let Some(other) = it.next() {
                    // Only strings can be used as type keys.
                    common_keys
                        .retain(|k| other.get(k).map(|v| v.field.is_string()).unwrap_or(false));
                }
            }

            let tag = match pick_tag(&common_keys)? {
                Some(tag) => tag,
                None => return Ok(None),
            };

            let mut out = Vec::new();

            for (child, value) in children.iter().zip(values.iter()) {
                let structure: LinkedHashMap<String, FieldFp> = child
                    .iter()
                    .filter(|&(key, _)| key != tag)
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect();

                let name = value
                    .get(tag)
                    .ok_or_else(|| format!("Missing common key: {}", tag))
                    .map(json::Value::as_str)
                    .and_then(|s| {
                        s.ok_or_else(|| format!("Expected string as common key: {}", tag))
                    })?
                    .to_string();

                out.push(SubTypeFp {
                    name: name,
                    structure: structure,
                });
            }

            return Ok(Some(InterfaceTestResult {
                tag: tag.to_string(),
                sub_types: out,
            }));

            /// Pick the tag used to determine sub-type.
            ///
            /// Some keys get priority, since they are default values for many frameworks.
            fn pick_tag<'a>(common_keys: &'a HashSet<String>) -> Result<Option<&'a str>> {
                if common_keys.contains("type") {
                    return Ok(Some("type"));
                }

                if common_keys.contains("@class") {
                    return Ok(Some("@class"));
                }

                Ok(common_keys
                    .iter()
                    .collect::<BTreeSet<_>>()
                    .iter()
                    .map(|s| s.as_str())
                    .next())
            }
        }

        struct InterfaceTestResult {
            tag: String,
            sub_types: Vec<SubTypeFp>,
        }

        fn json_as_object(input: &json::Value) -> Option<&json::Map<String, json::Value>> {
            match *input {
                json::Value::Object(ref object) => Some(object),
                _ => None,
            }
        }
    }
}

/// Simplified fingerprint used for hashing common lookups.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HashFp {
    Scalar,
    Object(LinkedHashMap<String, FieldHashFp>),
    Interface(String, Vec<SubTypeHashFp>),
    Array(Box<HashFp>),
    Tuple(Vec<FieldHashFp>),
}

#[derive(Clone)]
pub enum Reproto {
}

impl Custom for Reproto {
    type Extra = ();

    fn quote_string(out: &mut Formatter, input: &str) -> fmt::Result {
        out.write_char('"')?;

        for c in input.chars() {
            match c {
                '\t' => out.write_str("\\t")?,
                '\u{0007}' => out.write_str("\\b")?,
                '\n' => out.write_str("\\n")?,
                '\r' => out.write_str("\\r")?,
                '\u{0014}' => out.write_str("\\f")?,
                '\'' => out.write_str("\\'")?,
                '"' => out.write_str("\\\"")?,
                '\\' => out.write_str("\\\\")?,
                c => out.write_char(c)?,
            }
        }

        out.write_char('"')?;

        Ok(())
    }
}

/// The root name given to any derived item.
pub fn root_name() -> (String, RpName) {
    let package = RpPackage::empty();
    let package = RpVersionedPackage::new(package, None);
    let local_name = String::from("Generated");
    let name = RpName::new(None, package, vec![String::from("Generated")]);

    (local_name, name)
}

struct FieldInit<'a> {
    pos: &'a Pos,
    path: &'a [String],
    types: &'a mut TypesCache,
}

impl<'a> FieldInit<'a> {
    fn new(pos: &'a Pos, path: &'a [String], types: &'a mut TypesCache) -> FieldInit<'a> {
        FieldInit {
            pos: pos,
            path: path,
            types: types,
        }
    }

    fn init(self, original_name: String, fp: &FieldFp, decls: &mut Vec<RpDecl>) -> Result<RpField> {
        let mut comment = Vec::new();

        let name = to_snake_case(&original_name);

        let ty = match fp.field {
            Fp::Boolean => RpType::Boolean,
            Fp::Float => RpType::Float,
            Fp::Double => RpType::Double,
            Fp::I64(ref examples) => {
                format_comment(&mut comment, examples);
                RpType::Signed { size: 64 }
            }
            Fp::U64(ref examples) => {
                format_comment(&mut comment, examples);
                RpType::Unsigned { size: 64 }
            }
            Fp::String(ref examples) => {
                let examples = examples
                    .iter()
                    .map(|s| StringEscape::new(s.as_str()))
                    .collect::<Vec<_>>();

                format_comment(&mut comment, &examples);
                RpType::String
            }
            Fp::DateTime(ref examples) => {
                let examples = examples
                    .iter()
                    .map(|s| StringEscape::new(s.as_str()))
                    .collect::<Vec<_>>();

                format_comment(&mut comment, &examples);
                RpType::DateTime
            }
            Fp::Any => RpType::Any,
            Fp::Array(ref inner) => {
                let field = FieldFp {
                    optional: false,
                    field: (**inner).clone(),
                };

                let f = FieldInit::new(&self.pos, &self.path, self.types).init(
                    name.clone(),
                    &field,
                    decls,
                )?;

                RpType::Array {
                    inner: Box::new(f.ty),
                }
            }
            ref fp => {
                let package = RpPackage::empty();
                let package = RpVersionedPackage::new(package, None);

                let mut path = self.path.iter().cloned().collect::<Vec<_>>();
                path.push(to_pascal_case(&name));

                let name = if let Some(name) = self.types.get(fp).cloned() {
                    name
                } else {
                    let name = RpName::new(None, package, path.clone());
                    self.types.insert(fp.clone(), name.clone());

                    decls.push(DeclDeriver {
                        pos: &self.pos,
                        path: &path,
                        types: self.types,
                    }.derive(fp)?);

                    name
                };

                RpType::Name { name: name }
            }
        };

        let field_as = if name != original_name {
            Some(original_name)
        } else {
            None
        };

        // field referencing inner declaration
        return Ok(RpField {
            modifier: if fp.optional {
                RpModifier::Optional
            } else {
                RpModifier::Required
            },
            name: name.clone(),
            comment: comment,
            ty: ty,
            field_as: field_as,
        });

        /// Format comments and attach examples.
        fn format_comment<D: fmt::Display>(out: &mut Vec<String>, examples: &[D]) {
            out.push(format!("## Examples"));
            out.push("".to_string());

            out.push(format!("```json"));

            let mut seen = HashSet::new();

            for example in examples.iter() {
                let string = example.to_string();

                if !seen.contains(&string) {
                    seen.insert(string.clone());
                    out.push(string);
                }
            }

            out.push(format!("```"));
        }
    }
}

struct DeclDeriver<'a> {
    pos: &'a Pos,
    path: &'a [String],
    types: &'a mut TypesCache,
}

impl<'a> DeclDeriver<'a> {
    /// Derive a declaration from the given JSON.
    fn derive(self, fp: &Fp) -> Result<RpDecl> {
        let decl = match *fp {
            Fp::Tuple(ref array) => {
                let mut path = self.path.iter().cloned().collect::<Vec<_>>();

                let mut refiner = TupleRefiner {
                    pos: &self.pos,
                    path: &path,
                    types: self.types,
                };

                let tuple = refiner.derive(array)?;
                RpDecl::Tuple(Rc::new(Loc::new(tuple, self.pos.clone())))
            }
            Fp::Object(ref object) => {
                let mut path = self.path.iter().cloned().collect::<Vec<_>>();

                let mut refiner = TypeRefiner {
                    pos: &self.pos,
                    path: &path,
                    types: self.types,
                };

                let type_ = refiner.derive(object)?;
                RpDecl::Type(Rc::new(Loc::new(type_, self.pos.clone())))
            }
            Fp::Interface(ref type_field, ref sub_types) => {
                let mut path = self.path.iter().cloned().collect::<Vec<_>>();

                let type_ = InterfaceRefiner {
                    pos: &self.pos,
                    path: &path,
                    types: self.types,
                }.derive(type_field, sub_types)?;

                RpDecl::Interface(Rc::new(Loc::new(type_, self.pos.clone())))
            }
            // For arrays, only generate the inner type.
            Fp::Array(ref inner) => self.derive(inner)?,
            ref value => return Err(format!("Unexpected JSON value: {:?}", value).into()),
        };

        Ok(decl)
    }
}

struct TypeRefiner<'a> {
    pos: &'a Pos,
    path: &'a [String],
    types: &'a mut TypesCache,
}

impl<'a> TypeRefiner<'a> {
    /// Derive an struct body from the given input array.
    fn derive(&mut self, object: &LinkedHashMap<String, FieldFp>) -> Result<RpTypeBody> {
        let path = self.path.iter().cloned().collect::<Vec<_>>();

        let local_name = if let Some(local_name) = path.iter().last().cloned() {
            local_name
        } else {
            return Err(format!("No last component in name").into());
        };

        let package = RpPackage::empty();
        let package = RpVersionedPackage::new(package, None);
        let name = RpName::new(None, package, path);

        let mut body = RpTypeBody {
            local_name: local_name,
            name: name,
            comment: Vec::new(),
            decls: Vec::new(),
            fields: Vec::new(),
            codes: Vec::new(),
            reserved: HashSet::new(),
        };

        self.init(&mut body, object)?;
        Ok(body)
    }

    fn init(
        &mut self,
        base: &mut RpTypeBody,
        object: &LinkedHashMap<String, FieldFp>,
    ) -> Result<()> {
        for (name, added) in object.iter() {
            let field = FieldInit::new(&self.pos, &self.path, self.types).init(
                name.to_string(),
                added,
                &mut base.decls,
            )?;

            base.fields.push(Loc::new(field, self.pos.clone()));
        }

        Ok(())
    }
}

struct SubTypeRefiner<'a> {
    pos: &'a Pos,
    path: &'a [String],
    types: &'a mut TypesCache,
}

impl<'a> SubTypeRefiner<'a> {
    /// Derive an struct body from the given input array.
    fn derive(&mut self, sub_type: &SubTypeFp) -> Result<RpSubType> {
        let path = self.path.iter().cloned().collect::<Vec<_>>();

        let local_name = if let Some(local_name) = path.iter().last().cloned() {
            local_name
        } else {
            return Err(format!("No last component in name").into());
        };

        let package = RpPackage::empty();
        let package = RpVersionedPackage::new(package, None);
        let name = RpName::new(None, package, path.clone());

        let mut body = RpSubType {
            name: name,
            local_name: local_name.clone(),
            comment: vec![],
            decls: vec![],
            fields: vec![],
            codes: vec![],
            sub_type_name: None,
        };

        self.init(&mut body, sub_type)?;
        Ok(body)
    }

    fn init(&mut self, base: &mut RpSubType, sub_type: &SubTypeFp) -> Result<()> {
        if sub_type.name != base.local_name {
            base.sub_type_name = Some(Loc::new(sub_type.name.to_string(), self.pos.clone()));
        }

        for (field_name, field_value) in &sub_type.structure {
            let field = FieldInit::new(&self.pos, &self.path, self.types).init(
                field_name.to_string(),
                field_value,
                &mut base.decls,
            )?;

            base.fields.push(Loc::new(field, self.pos.clone()));
        }

        Ok(())
    }
}

struct InterfaceRefiner<'a> {
    pos: &'a Pos,
    path: &'a [String],
    types: &'a mut TypesCache,
}

impl<'a> InterfaceRefiner<'a> {
    /// Derive an struct body from the given input array.
    fn derive(&mut self, tag: &str, sub_types: &[SubTypeFp]) -> Result<RpInterfaceBody> {
        let path = self.path.iter().cloned().collect::<Vec<_>>();

        let local_name = if let Some(local_name) = path.iter().last().cloned() {
            local_name
        } else {
            return Err(format!("No last component in name").into());
        };

        let package = RpPackage::empty();
        let package = RpVersionedPackage::new(package, None);
        let name = RpName::new(None, package, path);

        let sub_type_strategy = if tag != DEFAULT_TAG {
            RpSubTypeStrategy::Tagged {
                tag: tag.to_string(),
            }
        } else {
            RpSubTypeStrategy::default()
        };

        let mut body = RpInterfaceBody {
            local_name: local_name,
            name: name,
            comment: Vec::new(),
            decls: Vec::new(),
            fields: Vec::new(),
            codes: Vec::new(),
            sub_types: BTreeMap::new(),
            sub_type_strategy: sub_type_strategy,
        };

        self.init(&mut body, sub_types)?;
        Ok(body)
    }

    fn init(&mut self, base: &mut RpInterfaceBody, sub_types: &[SubTypeFp]) -> Result<()> {
        for st in sub_types {
            let local_name = to_pascal_case(&st.name);

            let mut path = self.path.iter().cloned().collect::<Vec<_>>();
            path.push(local_name.clone());

            let sub_type = SubTypeRefiner {
                pos: self.pos,
                path: &path,
                types: self.types,
            }.derive(st)?;

            base.sub_types
                .insert(local_name, Rc::new(Loc::new(sub_type, self.pos.clone())));
        }

        Ok(())
    }
}

struct TupleRefiner<'a> {
    pos: &'a Pos,
    path: &'a [String],
    types: &'a mut TypesCache,
}

impl<'a> TupleRefiner<'a> {
    /// Derive an tuple body from the given input array.
    fn derive(&mut self, array: &[FieldFp]) -> Result<RpTupleBody> {
        let path = self.path.iter().cloned().collect::<Vec<_>>();

        let local_name = if let Some(local_name) = path.iter().last().cloned() {
            local_name
        } else {
            return Err(format!("No last component in name").into());
        };

        let package = RpPackage::empty();
        let package = RpVersionedPackage::new(package, None);
        let name = RpName::new(None, package, path);

        let mut body = RpTupleBody {
            local_name: local_name,
            name: name,
            comment: Vec::new(),
            decls: Vec::new(),
            fields: Vec::new(),
            codes: Vec::new(),
        };

        self.init(&mut body, array)?;
        Ok(body)
    }

    fn init(&mut self, base: &mut RpTupleBody, array: &[FieldFp]) -> Result<()> {
        for (index, added) in array.iter().enumerate() {
            let field = FieldInit::new(&self.pos, &self.path, self.types).init(
                format!("field_{}", index),
                added,
                &mut base.decls,
            )?;

            base.fields.push(Loc::new(field, self.pos.clone()));
        }

        Ok(())
    }
}

/// Format the given declaration.
fn format_decl<'el>(decl: &'el RpDecl) -> Result<Tokens<'el, Reproto>> {
    let result = match *decl {
        RpDecl::Type(ref type_) => format_type(type_),
        RpDecl::Interface(ref interface) => format_interface(interface),
        RpDecl::Tuple(ref tuple) => format_tuple(tuple),
        ref decl => return Err(format!("Unsupported declaration: {:?}", decl).into()),
    };

    return result;

    fn format_type<'el>(body: &'el RpTypeBody) -> Result<Tokens<'el, Reproto>> {
        let mut tuple = Tokens::new();

        tuple.push(toks!["type ", body.local_name.as_str(), " {"]);

        tuple.nested({
            let mut t = Tokens::new();

            for f in &body.fields {
                t.push(format_field(f)?);
            }

            for d in &body.decls {
                t.push(format_decl(d)?);
            }

            t.join_line_spacing()
        });

        tuple.push("}");

        Ok(tuple)
    }

    fn format_interface<'el>(body: &'el RpInterfaceBody) -> Result<Tokens<'el, Reproto>> {
        let mut interface = Tokens::new();

        match body.sub_type_strategy {
            RpSubTypeStrategy::Tagged { ref tag, .. } => {
                if tag != DEFAULT_TAG {
                    interface.push(toks![
                        "#[type_info(strategy = ",
                        "tagged".quoted(),
                        ", tag = ",
                        tag.as_str().quoted(),
                        ")]"
                    ]);
                }
            }
        }

        interface.push(toks!["interface ", body.local_name.as_str(), " {"]);

        interface.nested({
            let mut t = Tokens::new();

            for sub_type in body.sub_types.values() {
                t.push({
                    let mut t = Tokens::new();

                    if let Some(ref alias) = sub_type.sub_type_name {
                        t.push(toks![
                            sub_type.local_name.as_str(),
                            " as ",
                            alias.as_str().quoted(),
                            " {"
                        ]);
                    } else {
                        t.push(toks![sub_type.local_name.as_str(), " {"]);
                    }

                    t.nested({
                        let mut t = Tokens::new();

                        for f in &sub_type.fields {
                            t.push(format_field(f)?);
                        }

                        for d in &sub_type.decls {
                            t.push(format_decl(d)?);
                        }

                        t.join_line_spacing()
                    });

                    t.push("}");

                    t
                });
            }

            for d in &body.decls {
                t.push(format_decl(d)?);
            }

            t.join_line_spacing()
        });

        interface.push("}");

        Ok(interface)
    }

    fn format_tuple<'el>(body: &'el RpTupleBody) -> Result<Tokens<'el, Reproto>> {
        let mut tuple = Tokens::new();

        tuple.push(toks!["tuple ", body.local_name.as_str(), " {"]);

        tuple.nested({
            let mut t = Tokens::new();

            for f in &body.fields {
                t.push(format_field(f)?);
            }

            for d in &body.decls {
                t.push(format_decl(d)?);
            }

            t.join_line_spacing()
        });

        tuple.push("}");

        Ok(tuple)
    }

    fn format_field<'el>(field: &'el RpField) -> Result<Tokens<'el, Reproto>> {
        let mut t = Tokens::new();

        for line in &field.comment {
            if line.is_empty() {
                t.push("///");
            } else {
                t.push(toks!["/// ", line.as_str()]);
            }
        }

        if field.is_optional() {
            t.push(toks![field.name.as_str(), "?: ", field.ty.to_string()]);
        } else {
            t.push(toks![field.name.as_str(), ": ", field.ty.to_string()]);
        }

        if let Some(ref field_as) = field.field_as {
            t.extend(toks![" as ", field_as.as_str().quoted()]);
        }

        t.append(";");

        Ok(t)
    }
}

pub fn derive<O: 'static>(object: O) -> Result<()>
where
    O: Object,
{
    let mut der = json::Deserializer::from_reader(object.read()?).into_iter::<json::Value>();

    let value: Result<json::Value> = der.next()
        .ok_or_else(|| format!("Expected at least one JSON value").into())
        .and_then(|v| v.map_err(|e| format!("Bad JSON: {}", e).into()));

    let value = value?;
    let fp = Fp::from_json(&value)?;

    let pos: Pos = (Rc::new(Box::new(object) as Box<Object>), 0, 0).into();

    let mut types = HashMap::new();

    let decl = DeclDeriver {
        pos: &pos,
        path: &vec!["Generated".to_string()],
        types: &mut types,
    }.derive(&fp)?;

    let stdout = io::stdout();
    let toks = format_decl(&decl)?;

    IoFmt(&mut stdout.lock()).write_file(toks, &mut ())?;

    Ok(())
}
