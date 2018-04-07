#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
}

impl Type {
  pub fn type_method(&self) {
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Interface {
  SubType {
  },
}

impl Interface {
  pub fn interface_method(&self) {
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Enum {
  Variant,
}

impl Enum {
  pub fn value(&self) -> &'static str {
    use self::Enum::*;
    match *self {
      Variant => "Variant",
    }
  }
  pub fn enum_method(&self) {
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tuple();
