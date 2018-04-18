#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  /// The foo field.
  #[serde(skip_serializing_if="Option::is_none")]
  pub foo: Option<Foo>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Foo {
  /// The field.
  pub field: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bar {
  /// The inner field.
  pub field: Bar_Inner,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bar_Inner {
  /// The field.
  pub field: String,
}
