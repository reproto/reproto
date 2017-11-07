#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Entry {
  #[serde(rename = "bar")]
  Bar {
  },
  #[serde(rename = "foo")]
  Foo {
  },
}
