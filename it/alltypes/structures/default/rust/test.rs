use chrono;
use chrono::offset;
use serde_json as json;
use std::collections;

#[derive(Serialize, Deserialize, Debug)]
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
