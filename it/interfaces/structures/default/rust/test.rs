#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub tagged: Option<Tagged>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub required_fields: Option<RequiredFields>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequiredFields {
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
