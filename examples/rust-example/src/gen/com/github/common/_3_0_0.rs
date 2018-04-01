/// A github user.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  /// Login handle for the user.
  pub login: String,

  /// Identifier of the user.
  pub id: u64,

  /// URL to avatar of the user.
  pub avatar_url: String,

  /// Gravatar ID of the user.
  pub gravatar_id: String,

  /// URL to the user page.
  pub url: String,

  /// HTML URL to the user page.
  pub html_url: String,

  /// API URL to get followers.
  pub followers_url: String,

  /// API URL to get following.
  pub following_url: String,

  /// API URL to get gists for user.
  pub gists_url: String,

  /// API URL to get starred by user.
  pub starred_url: String,

  /// URL to get subscriptions for user.
  pub subscriptions_url: String,

  /// URL to get organizations for user.
  pub organizations_url: String,

  /// URL to get repositories for user.
  pub repos_url: String,

  /// URL to get events for user.
  pub events_url: String,

  /// URL to get received events by user.
  pub received_events_url: String,

  /// Type of the user.
  #[serde(rename = "type")]
  pub _type: User_Type,

  /// Is this user a site admin?
  pub site_admin: bool,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum User_Type {
  #[serde(rename = "User")]
  USER,
}

impl User_Type {
  pub fn value(&self) -> &'static str {
    use self::User_Type::*;
    match *self {
      USER => "User",
    }
  }
}
