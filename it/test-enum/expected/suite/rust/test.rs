#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  explicit: EnumExplicit,
  implicit: EnumImplicit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EnumExplicit {
  #[serde(rename = "foo")]
  A,
  #[serde(rename = "bar")]
  B,
}

impl EnumExplicit {
  pub fn value(&self) -> &'static str {
    use self::EnumExplicit::*;
    match *self {
      A => "foo",
      B => "bar",
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EnumImplicit {
  A,
  B,
}

impl EnumImplicit {
  pub fn value(&self) -> &'static str {
    use self::EnumImplicit::*;
    match *self {
      A => "A",
      B => "B",
    }
  }
}
