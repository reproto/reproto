#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  tuple1: Option<Tuple1>,
  #[serde(skip_serializing_if="Option::is_none")]
  tuple2: Option<Tuple2>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tuple1(
String, 
u64);

#[derive(Serialize, Deserialize, Debug)]
struct Tuple2(
String, 
Other);

#[derive(Serialize, Deserialize, Debug)]
pub struct Other {
  a: String,
}
