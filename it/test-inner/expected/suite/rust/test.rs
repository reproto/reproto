#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  a: Option<A>,
  #[serde(skip_serializing_if="Option::is_none")]
  b: Option<A_B>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A {
  b: A_B,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A_B {
  field: String,
}
