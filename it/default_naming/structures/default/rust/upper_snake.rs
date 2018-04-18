#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
  #[serde(rename = "FOO_BAR")]
  pub foo_bar: String,
}
