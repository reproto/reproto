#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Entry {
  #[serde(rename = "bar")]
  Bar {
    shared: String,
    bar: String,
  },
  #[serde(rename = "foo")]
  Foo {
    shared: String,
    foo: String,
  },
}
