#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
  #[serde(rename = "fooBar")]
  foo_bar: String,
}

pub trait Service {
  fn foo_bar();
}
