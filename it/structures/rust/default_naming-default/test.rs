use crate::lower_camel as lower_camel;
use crate::lower_snake as lower_snake;
use crate::upper_camel as upper_camel;
use crate::upper_snake as upper_snake;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub lower_camel: Option<lower_camel::Value>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub lower_snake: Option<lower_snake::Value>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub upper_camel: Option<upper_camel::Value>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub upper_snake: Option<upper_snake::Value>,
}
