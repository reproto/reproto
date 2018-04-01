#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
  #[serde(rename = "FOO_BAR")]
  pub foo_bar: String,
}
