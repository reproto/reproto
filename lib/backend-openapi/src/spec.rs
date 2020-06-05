use core::Version;
use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use std::collections::BTreeMap;

pub struct Ref(pub String);

impl<'a> From<Ref> for Schema<'a> {
    fn from(reference: Ref) -> Self {
        Schema {
            reference: Some(reference.0),
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct SchemaBoolean {}

impl<'a> From<SchemaBoolean> for Schema<'a> {
    fn from(_: SchemaBoolean) -> Self {
        Schema {
            ty: Some("boolean"),
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct SchemaAny;

impl<'a> From<SchemaAny> for Schema<'a> {
    fn from(_: SchemaAny) -> Self {
        Schema::default()
    }
}

#[derive(Debug, Default)]
pub struct Integer {
    pub format: Option<Format>,
}

macro_rules! numeric_type {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Default)]
        pub struct $name {
            pub enum_: Vec<$ty>,
        }

        impl<'a> From<$name> for Schema<'a> {
            #[allow(unused)]
            fn from(integer: $name) -> Self {
                Schema {
                    ty: Some("integer"),
                    format: Some(Format::$name),
                    enum_: Enum::$name(integer.enum_),
                    ..Schema::default()
                }
            }
        }
    };
}

numeric_type!(U32, u32);
numeric_type!(U64, u64);
numeric_type!(I32, i32);
numeric_type!(I64, i64);
numeric_type!(Float, f32);
numeric_type!(Double, f64);

#[derive(Debug, Default)]
pub struct SchemaString<'a> {
    pub enum_: Vec<&'a str>,
    pub format: Option<Format>,
}

impl<'a> From<SchemaString<'a>> for Schema<'a> {
    fn from(string: SchemaString<'a>) -> Self {
        Schema {
            ty: Some("string"),
            enum_: Enum::String(string.enum_),
            format: string.format,
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Object<'a> {
    pub required: Vec<&'a str>,
    pub properties: LinkedHashMap<&'a str, Schema<'a>>,
    pub additional_properties: Option<Box<Schema<'a>>>,
    pub title: Option<&'a str>,
    pub description: Option<String>,
}

impl<'a> From<Object<'a>> for Schema<'a> {
    fn from(object: Object<'a>) -> Self {
        Schema {
            ty: Some("object"),
            required: Required::String(object.required),
            properties: Properties::String(object.properties),
            additional_properties: object.additional_properties,
            title: object.title,
            description: object.description,
            ..Schema::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct SchemaArray<'a> {
    pub items: Option<Box<Schema<'a>>>,
    pub format: Option<Format>,
    pub required: Vec<usize>,
    /// For tuples, map each position to a type.
    pub properties: BTreeMap<usize, Schema<'a>>,
}

impl<'a> From<SchemaArray<'a>> for Schema<'a> {
    fn from(array: SchemaArray<'a>) -> Self {
        Schema {
            ty: Some("array"),
            items: array.items,
            format: array.format,
            required: Required::Usize(array.required),
            properties: Properties::Usize(array.properties),
            ..Schema::default()
        }
    }
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub enum Properties<'a> {
    String(LinkedHashMap<&'a str, Schema<'a>>),
    Usize(BTreeMap<usize, Schema<'a>>),
    None,
}

impl<'a> Default for Properties<'a> {
    fn default() -> Self {
        Properties::None
    }
}

impl<'a> Properties<'a> {
    fn is_empty(&self) -> bool {
        use self::Properties::*;

        match *self {
            String(ref properties) => properties.is_empty(),
            Usize(ref properties) => properties.is_empty(),
            None => true,
        }
    }
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub enum Required<'a> {
    String(Vec<&'a str>),
    Usize(Vec<usize>),
    None,
}

impl<'a> Default for Required<'a> {
    fn default() -> Self {
        Required::None
    }
}

impl<'a> Required<'a> {
    fn is_empty(&self) -> bool {
        use self::Required::*;

        match *self {
            String(ref required) => required.is_empty(),
            Usize(ref required) => required.is_empty(),
            None => true,
        }
    }
}

#[serde(untagged, rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub enum Enum<'a> {
    String(Vec<&'a str>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    None,
}

impl<'a> Default for Enum<'a> {
    fn default() -> Self {
        Enum::None
    }
}

impl<'a> Enum<'a> {
    fn is_empty(&self) -> bool {
        use self::Enum::*;

        match *self {
            String(ref variants) => variants.is_empty(),
            U32(ref variants) => variants.is_empty(),
            U64(ref variants) => variants.is_empty(),
            I32(ref variants) => variants.is_empty(),
            I64(ref variants) => variants.is_empty(),
            Float(ref variants) => variants.is_empty(),
            Double(ref variants) => variants.is_empty(),
            None => true,
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
pub struct Info<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<&'a Version>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub enum ParameterIn {
    #[serde(rename = "path")]
    Path,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub struct Parameter<'a> {
    pub in_: ParameterIn,
    pub name: &'a str,
    pub schema: Schema<'a>,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Serialize)]
pub struct Discriminator<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<&'a str>,

    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    pub mapping: LinkedHashMap<&'a str, String>,
}

#[derive(Debug, Serialize)]
pub enum Format {
    #[serde(rename = "uint32")]
    U32,
    #[serde(rename = "uint64")]
    U64,
    #[serde(rename = "int32")]
    I32,
    #[serde(rename = "int64")]
    I64,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "tuple")]
    Tuple,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Serialize)]
pub struct Schema<'a> {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ty: Option<&'static str>,

    /// Description of this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// How arrays specify inner item type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema<'a>>>,

    /// Format acts as extra specification of the type when needed.
    /// Also extensible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,

    /// Available enumerations.
    #[serde(rename = "enum", skip_serializing_if = "Enum::is_empty")]
    pub enum_: Enum<'a>,

    /// `oneOf` field
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<Schema<'a>>,

    /// discriminator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator<'a>>,

    #[serde(rename = "required", skip_serializing_if = "Required::is_empty")]
    pub required: Required<'a>,

    #[serde(rename = "properties", skip_serializing_if = "Properties::is_empty")]
    pub properties: Properties<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<Schema<'a>>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub struct Content<'a> {
    pub schema: Schema<'a>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
pub struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    pub content: LinkedHashMap<&'a str, Content<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
pub struct Method<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<Payload<'a>>,
    /// Content by status code.
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    pub responses: LinkedHashMap<&'a str, Payload<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Serialize)]
pub struct SpecPath<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Method<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Method<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Serialize)]
pub struct Components<'a> {
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    pub schemas: LinkedHashMap<String, Schema<'a>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub struct Server<'a> {
    pub url: &'a str,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize)]
pub struct Spec<'a> {
    pub openapi: &'static str,
    pub info: Info<'a>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server<'a>>,
    #[serde(skip_serializing_if = "LinkedHashMap::is_empty")]
    pub paths: LinkedHashMap<String, SpecPath<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components<'a>>,
}

fn is_false(input: &bool) -> bool {
    !input
}
