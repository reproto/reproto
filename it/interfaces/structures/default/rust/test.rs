#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub tagged: Option<Tagged>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub untagged: Option<Untagged>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Tagged {
  #[serde(rename = "foo")]
  A(Tagged_A),
  #[serde(rename = "b")]
  B(Tagged_B),
  Bar(Tagged_Bar),
  Baz(Tagged_Baz),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_A {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_B {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_Bar {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Tagged_Baz {
  pub shared: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Untagged {
  A(Untagged_A),
  B(Untagged_B),
  C(Untagged_C),
}

/// Special case: fields shared with other sub-types.
/// NOTE: due to rust support through untagged, the types are matched in-order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Untagged_A {
  pub shared: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub shared_ignore: Option<String>,

  pub a: String,

  pub b: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub ignore: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Untagged_B {
  pub shared: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub shared_ignore: Option<String>,

  pub a: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub ignore: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Untagged_C {
  pub shared: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub shared_ignore: Option<String>,

  pub b: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub ignore: Option<String>,
}
