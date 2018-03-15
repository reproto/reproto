use chrono;
use chrono::offset;
use serde_json as json;
use std::collections;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  boolean_type: Option<bool>,
  #[serde(skip_serializing_if="Option::is_none")]
  string_type: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  datetime_type: Option<chrono::DateTime<offset::Utc>>,
  #[serde(skip_serializing_if="Option::is_none")]
  unsigned_32: Option<u32>,
  #[serde(skip_serializing_if="Option::is_none")]
  unsigned_64: Option<u64>,
  #[serde(skip_serializing_if="Option::is_none")]
  signed_32: Option<i32>,
  #[serde(skip_serializing_if="Option::is_none")]
  signed_64: Option<i64>,
  #[serde(skip_serializing_if="Option::is_none")]
  float_type: Option<f32>,
  #[serde(skip_serializing_if="Option::is_none")]
  double_type: Option<f64>,
  #[serde(skip_serializing_if="Option::is_none")]
  bytes_type: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  any_type: Option<json::Value>,
  #[serde(skip_serializing_if="Option::is_none")]
  array_type: Option<Vec<Entry>>,
  #[serde(skip_serializing_if="Option::is_none")]
  map_type: Option<collections::HashMap<String, Entry>>,
}
