use chrono;
use chrono::offset;
use serde;
use serde::de;
use serde_json as json;
use std::collections;
use std::fmt;

/// A bizarre entry with many different optional fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub boolean_type: Option<bool>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub string_type: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub datetime_type: Option<chrono::DateTime<offset::Utc>>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub unsigned_32: Option<u32>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub unsigned_64: Option<u64>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub signed_32: Option<i32>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub signed_64: Option<i64>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub float_type: Option<f32>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub double_type: Option<f64>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub bytes_type: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub any_type: Option<json::Value>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub array_type: Option<Vec<Entry>>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub array_of_array_type: Option<Vec<Vec<Entry>>>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub map_type: Option<collections::HashMap<String, Entry>>,
}

/// The state of a thing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
  /// The open state.
  #[serde(rename = "open")]
  Open,
  /// The closed state.
  #[serde(rename = "closed")]
  Closed,
}

impl State {
  pub fn value(&self) -> &'static str {
    use self::State::*;
    match *self {
      Open => "open",
      Closed => "closed",
    }
  }
}

/// A numeric thing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
  /// The error was caused by the user.
  User,
  /// The error was caused by the server.
  Server,
}

impl ErrorCode {
  pub fn value(&self) -> u32 {
    use self::ErrorCode::*;
    match *self {
      User => 400,
      Server => 500,
    }
  }
}

impl serde::Serialize for ErrorCode {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
  use self::ErrorCode::*;
    let o = match *self {
      User => 400u32,
      Server => 500u32,
    };
    s.serialize_u32(o)
  }
}

impl<'de> serde::Deserialize<'de> for ErrorCode {
  fn deserialize<D>(d: D) -> Result<ErrorCode, D::Error>
    where D: serde::Deserializer<'de>
  {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
      type Value = ErrorCode;

      fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("ErrorCode, one of: 400, 500")
      }

      fn visit_u32<E>(self, value: u32) -> Result<ErrorCode, E>
        where E: de::Error
      {
        match value {
          400u32 => Ok(ErrorCode::User),
          500u32 => Ok(ErrorCode::Server),
          value => Err(E::custom(format!("ErrorCode: unknown value: {}", value))),
        }
      }

      fn visit_u64<E>(self, value: u64) -> Result<ErrorCode, E>
        where E: de::Error
      {
        self.visit_u32(value as u32)
      }
    }

    d.deserialize_u32(Visitor)
  }
}

/// A single point in time with a value associated with it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point(pub u64, pub f64);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Tagged {
  #[serde(rename = "foo")]
  A(Tagged_A),
  #[serde(rename = "b")]
  B(Tagged_B),
  Bar(Tagged_Bar),
  Baz(Tagged_Baz),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_A {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_B {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_Bar {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_Baz {
  pub shared: String,
}

/// An untagged interface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Untagged {
  A(Untagged_A),
  B(Untagged_B),
  C(Untagged_C),
}

/// Special case: fields shared with other sub-types.
/// NOTE: due to rust support through untagged, the types are matched in-order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Untagged_A {
  pub shared: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub shared_ignore: Option<String>,

  pub a: String,

  pub b: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub ignore: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Untagged_B {
  pub shared: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub shared_ignore: Option<String>,

  pub a: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub ignore: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Untagged_C {
  pub shared: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub shared_ignore: Option<String>,

  pub b: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub ignore: Option<String>,
}
