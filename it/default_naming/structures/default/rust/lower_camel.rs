#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
  #[serde(rename = "fooBar")]
  pub foo_bar: String,
}
