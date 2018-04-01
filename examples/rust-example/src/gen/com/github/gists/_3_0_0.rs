use chrono;
use chrono::offset;
use gen::com::github::common::_3_0_0 as c;
use std::collections;

#[derive(Serialize, Deserialize, Debug)]
pub struct Gist {
  /// ## Examples
  ///
  /// ```json
  /// "aa5a315d61ae9438b18d"
  /// ```
  pub id: String,

  /// ## Examples
  ///
  /// ```json
  /// "https://api.github.com/gists/aa5a315d61ae9438b18d"
  /// ```
  pub url: String,

  /// ## Examples
  ///
  /// ```json
  /// "https://api.github.com/gists/aa5a315d61ae9438b18d/forks"
  /// ```
  pub forks_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "https://api.github.com/gists/aa5a315d61ae9438b18d/commits"
  /// ```
  pub commits_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "description of gist"
  /// ```
  pub description: String,

  pub public: bool,

  pub owner: c::User,

  #[serde(skip_serializing_if="Option::is_none")]
  pub user: Option<c::User>,

  pub files: collections::HashMap<String, Gist_File>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub truncated: Option<bool>,

  /// ## Examples
  ///
  /// ```json
  /// 0
  /// ```
  pub comments: u64,

  /// ## Examples
  ///
  /// ```json
  /// "https://api.github.com/gists/aa5a315d61ae9438b18d/comments/"
  /// ```
  pub comments_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "https://gist.github.com/aa5a315d61ae9438b18d"
  /// ```
  pub html_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "https://gist.github.com/aa5a315d61ae9438b18d.git"
  /// ```
  pub git_pull_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "https://gist.github.com/aa5a315d61ae9438b18d.git"
  /// ```
  pub git_push_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "2010-04-14T02:15:15Z"
  /// ```
  pub created_at: chrono::DateTime<offset::Utc>,

  /// ## Examples
  ///
  /// ```json
  /// "2011-06-20T11:34:15Z"
  /// ```
  pub updated_at: chrono::DateTime<offset::Utc>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Gist_File {
  /// ## Examples
  ///
  /// ```json
  /// 932
  /// ```
  pub size: u64,

  /// ## Examples
  ///
  /// ```json
  /// "https://gist.githubusercontent.com/raw/365370/8c4d2d43d178df44f4c03a7f2ac0ff512853564e/ring.erl"
  /// ```
  pub raw_url: String,

  /// ## Examples
  ///
  /// ```json
  /// "text/plain"
  /// ```
  #[serde(rename = "type")]
  pub _type: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub truncated: Option<bool>,

  /// ## Examples
  ///
  /// ```json
  /// "Erlang"
  /// ```
  #[serde(skip_serializing_if="Option::is_none")]
  pub language: Option<String>,
}
