#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub tagged: Option<Tagged>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub untagged: Option<Untagged>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Tagged {
  #[serde(rename = "foo")]
  A {
    shared: String,
  },
  #[serde(rename = "b")]
  B {
    shared: String,
  },
  Bar {
    shared: String,
  },
  Baz {
    shared: String,
  },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Untagged {
  /// Special case: fields shared with other sub-types.
  /// NOTE: due to rust support through untagged, the types are matched in-order.
  A {
    shared: String,

    #[serde(skip_serializing_if="Option::is_none")]
    shared_ignore: Option<String>,

    a: String,

    b: String,

    #[serde(skip_serializing_if="Option::is_none")]
    ignore: Option<String>,
  },
  B {
    shared: String,

    #[serde(skip_serializing_if="Option::is_none")]
    shared_ignore: Option<String>,

    a: String,

    #[serde(skip_serializing_if="Option::is_none")]
    ignore: Option<String>,
  },
  C {
    shared: String,

    #[serde(skip_serializing_if="Option::is_none")]
    shared_ignore: Option<String>,

    b: String,

    #[serde(skip_serializing_if="Option::is_none")]
    ignore: Option<String>,
  },
}
