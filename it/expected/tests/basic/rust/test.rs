#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  /// The foo field.
  #[serde(skip_serializing_if="Option::is_none")]
  foo: Option<Foo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Foo {
  /// The field.
  field: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bar {
  /// The inner field.
  field: Bar_Inner,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bar_Inner {
  /// The field.
  field: String,
}
