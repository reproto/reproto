#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip_serializing_if="Option::is_none")]
    foo: Option<Foo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Foo {
    field: String,
}
