use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
  #[serde(rename = "FooBar")]
  pub foo_bar: String,
}
