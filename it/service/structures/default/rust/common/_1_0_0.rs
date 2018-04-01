#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  pub name: String,
}

/// # Error me
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
  pub message: String,

  pub status_code: u32,
}
