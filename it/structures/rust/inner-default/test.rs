#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub a: Option<A>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub b: Option<A_B>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A {
  pub b: A_B,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A_B {
  pub field: String,
}
