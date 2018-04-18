#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  pub name: String,
}

/// # Error me
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorMessage {
  pub message: String,

  pub status_code: u32,
}
