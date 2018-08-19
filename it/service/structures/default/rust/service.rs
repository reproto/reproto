use chrono;
use chrono::offset;
use serde_json as json;

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

/// A single point in time with a value associated with it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point(pub u64, pub f64);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Tagged {
  #[serde(rename = "foo")]
  A {
    shared: String,
  },
  #[serde(rename = "b")]
  B {
    shared: String,
  },
  Bar {
    shared: String,
  },
  Baz {
    shared: String,
  },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Untagged {
  /// Special case: fields shared with other sub-types.
  /// NOTE: due to rust support through untagged, the types are matched in-order.
  A {
    shared: String,

    #[serde(skip_serializing_if="Option::is_none")]
    shared_ignore: Option<String>,

    a: String,

    b: String,

    #[serde(skip_serializing_if="Option::is_none")]
    ignore: Option<String>,
  },
  B {
    shared: String,

    #[serde(skip_serializing_if="Option::is_none")]
    shared_ignore: Option<String>,

    a: String,

    #[serde(skip_serializing_if="Option::is_none")]
    ignore: Option<String>,
  },
  C {
    shared: String,

    #[serde(skip_serializing_if="Option::is_none")]
    shared_ignore: Option<String>,

    b: String,

    #[serde(skip_serializing_if="Option::is_none")]
    ignore: Option<String>,
  },
}
