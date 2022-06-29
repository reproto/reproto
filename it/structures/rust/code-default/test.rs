use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Type {}

impl Type {
  pub fn type_method(&self) {
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Interface {
  SubType(Interface_SubType),
}

impl Interface {
  pub fn interface_method(&self) {
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Interface_SubType {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Enum {
  Variant,
}

impl Enum {
  pub fn value(&self) -> &'static str {
    match self {
      Self::Variant => "Variant",
    }
  }

  pub fn enum_method(&self) {
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tuple();
