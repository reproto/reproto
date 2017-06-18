use foo_4_0_0 as foo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  thing: Option<foo::Thing>,
}
