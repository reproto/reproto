#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum Entry {
  #[serde(rename = "foo")]
  A {
    shared: String,
  },
  #[serde(rename = "b")]
  B {
    shared: String,
  },
  Bar {
    shared: String,
  },
  Baz {
    shared: String,
  },
}
