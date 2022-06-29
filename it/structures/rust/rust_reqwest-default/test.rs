use crate::reproto;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {}

#[allow(non_camel_case_types)]
pub struct MyService_Reqwest {
  client: reqwest::Client,
  url: reqwest::Url,
}

impl MyService_Reqwest {
  pub fn new(client: reqwest::Client, url: Option<reqwest::Url>) -> reproto::Result<Self> {
    let url = match url {
      Some(url) => url,
      None => reqwest::Url::parse("http://example.com")?,
    };

    Ok(Self { client, url })
  }

  /// UNKNOWN
  pub async fn unknown(&self, id: u32) -> reproto::Result<()> {
    use std::fmt::Write as _;

    let mut path_ = String::new();

    path_.push_str("/");
    path_.push_str("unknown");
    path_.push_str("/");
    write!(path_, "{}", reproto::PathEncode(id))?;

    let url_ = self.url.join(&path_)?;

    let req_ = self.client
      .request(reqwest::Method::GET, url_);

    req_.send().await?;
    Ok(())
  }

  /// UNKNOWN
  pub async fn unknown_return(&self, id: u32) -> reproto::Result<Entry> {
    use std::fmt::Write as _;

    let mut path_ = String::new();

    path_.push_str("/");
    path_.push_str("unknown-return");
    path_.push_str("/");
    write!(path_, "{}", reproto::PathEncode(id))?;

    let url_ = self.url.join(&path_)?;

    let req_ = self.client
      .request(reqwest::Method::GET, url_);

    let res_ = req_.send().await?;
    let body_ = res_.json().await?;
    Ok(body_)
  }

  /// UNKNOWN
  pub async fn unknown_argument(&self, request: Entry, id: u32) -> reproto::Result<()> {
    use std::fmt::Write as _;

    let mut path_ = String::new();

    path_.push_str("/");
    path_.push_str("unknown-argument");
    path_.push_str("/");
    write!(path_, "{}", reproto::PathEncode(id))?;

    let url_ = self.url.join(&path_)?;

    let req_ = self.client
      .request(reqwest::Method::GET, url_)
      .json(&request);

    req_.send().await?;
    Ok(())
  }

  /// UNARY
  pub async fn unary(&self, request: Entry, id: u32) -> reproto::Result<Entry> {
    use std::fmt::Write as _;

    let mut path_ = String::new();

    path_.push_str("/");
    path_.push_str("unary");
    path_.push_str("/");
    write!(path_, "{}", reproto::PathEncode(id))?;

    let url_ = self.url.join(&path_)?;

    let req_ = self.client
      .request(reqwest::Method::GET, url_)
      .json(&request);

    let res_ = req_.send().await?;
    let body_ = res_.json().await?;
    Ok(body_)
  }
}
