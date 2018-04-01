#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
  #[serde(rename = "FooBar")]
  pub foo_bar: String,
}
