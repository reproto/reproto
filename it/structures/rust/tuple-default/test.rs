use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub tuple1: Option<Tuple1>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub tuple2: Option<Tuple2>,
}

/// Tuple containing primitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tuple1(pub String, pub u64);

/// Tuple containing object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tuple2(pub String, pub Other);

/// Complex object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Other {
  pub a: String,
}
