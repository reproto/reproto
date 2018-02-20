#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  tuple1: Option<Tuple1>,
  #[serde(skip_serializing_if="Option::is_none")]
  tuple2: Option<Tuple2>,
}

/// Tuple containing primitive.
#[derive(Serialize, Deserialize, Debug)]
struct Tuple1(
String, 
u64);

/// Tuple containing object.
#[derive(Serialize, Deserialize, Debug)]
struct Tuple2(
String, 
Other);

/// Complex object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Other {
  a: String,
}
