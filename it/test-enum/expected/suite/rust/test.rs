#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  explicit: Option<EnumExplicit>,
  #[serde(skip_serializing_if="Option::is_none")]
  implicit: Option<EnumImplicit>,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum EnumLongNames {
  FooBar,
  Baz,
}

impl EnumLongNames {
  pub fn value(&self) -> &'static str {
    use self::EnumLongNames::*;
    match *self {
      FooBar => "FooBar",
      Baz => "Baz",
    }
  }
}
