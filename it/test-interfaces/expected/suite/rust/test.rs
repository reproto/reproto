#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum Entry {
  #[serde(rename = "foo")]
  A {
  },
  #[serde(rename = "b")]
  B {
  },
  Bar {
  },
  Baz {
  },
}
