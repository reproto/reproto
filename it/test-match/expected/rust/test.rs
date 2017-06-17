#[derive(Serialize, Deserialize)]
struct Entry {
  data: Option<Data>,
  point: Option<Point>,
  interface_field: Option<Interface>,
  type_field: Option<Type>,
}

#[derive(Serialize, Deserialize)]
struct Data {
  name: String,
}

#[derive(Serialize, Deserialize)]
struct Point(u64, double);

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Interface {
  #[serde(rename = "one")]
  One {
    name: String,
    other: Option<u32>,
    data: Data,
  },

  #[serde(rename = "two")]
  Two {
    name: String,
    other: Option<u32>,
    data: Data,
  },
}

#[derive(Serialize, Deserialize)]
struct Type {
  data: String,
  other: Option<u32>,
}
