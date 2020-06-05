#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
  #[serde(rename = "fooBar")]
  pub foo_bar: String,
}
