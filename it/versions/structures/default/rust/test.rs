use foo::v4 as foo;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub thing: Option<foo::Thing>,
}
