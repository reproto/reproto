#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub tuple1: Option<Tuple1>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub tuple2: Option<Tuple2>,
}

/// Tuple containing primitive.
#[derive(Serialize, Deserialize, Debug)]
pub struct Tuple1(pub String, pub u64);

/// Tuple containing object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Tuple2(pub String, pub Other);

/// Complex object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Other {
  pub a: String,
}
