#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  foo: Option<Foo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Foo {
  field: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bar {
  field: Bar_Inner,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bar_Inner {
  field: String,
}
