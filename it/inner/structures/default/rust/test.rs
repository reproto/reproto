#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub a: Option<A>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub b: Option<A_B>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A {
  pub b: A_B,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct A_B {
  pub field: String,
}
