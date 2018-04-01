use gen::com::github::gists::_3_0_0 as gists;
use gen::reproto;
use reqwest;
use std::fmt::Write;

pub struct Github_Reqwest{
  client: reqwest::Client,
  url: reqwest::Url,
}

impl Github_Reqwest {
  pub fn new(client: reqwest::Client, url: Option<reqwest::Url>) -> reproto::Result<Self> {
    let url = match url {
      Some(url) => url,
      None => reqwest::Url::parse("https://api.github.com/")?,
    };

    Ok(Self {
      client,
      url,
    })
  }

  /// Get the gists for the given user.
  pub fn get_user_gists(&self, username: String) -> reproto::Result<Vec<gists::Gist>> {
    let mut path_ = String::new();
    path_.push_str("/");
    path_.push_str("users");
    path_.push_str("/");
    write!(path_, "{}", reproto::PathEncode(username))?;
    path_.push_str("/");
    path_.push_str("gists");

    let url_ = self.url.join(&path_)?;

    let mut req_ = self.client.request(reqwest::Method::Get, url_);

    let mut res_ = req_.send()?;

    let body_ = res_.json()?;

    Ok(body_)
  }
}
