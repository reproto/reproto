#[derive(Serialize, Deserialize)]
struct Entry {
  foo: Option<Foo>,
}

#[derive(Serialize, Deserialize)]
struct Foo {
  field: String,
}
