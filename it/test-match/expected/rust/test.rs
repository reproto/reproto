#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  data: Option<Data>,
  #[serde(skip_serializing_if="Option::is_none")]
  point: Option<Point>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "interface")]
  interface_field: Option<Interface>,
  #[serde(skip_serializing_if="Option::is_none")]
  #[serde(rename = "type")]
  type_field: Option<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point(u64, f64);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Interface {
  #[serde(rename = "one")]
  One {
    name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    other: Option<u32>,
    data: Data,
  },

  #[serde(rename = "two")]
  Two {
    name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    other: Option<u32>,
    data: Data,
  },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
  data: String,
  #[serde(skip_serializing_if="Option::is_none")]
  other: Option<u32>,
}
