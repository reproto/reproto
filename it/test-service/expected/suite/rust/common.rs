#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
  message: String,
  status_code: i32,
}
