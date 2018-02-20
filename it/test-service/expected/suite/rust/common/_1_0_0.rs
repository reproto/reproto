#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  name: String,
}

/// # Error me
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
  message: String,
  status_code: i32,
}
