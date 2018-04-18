use serde;
use serde::de;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  #[serde(skip_serializing_if="Option::is_none")]
  pub explicit: Option<EnumExplicit>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub implicit: Option<EnumImplicit>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub enum_u32: Option<EnumU32>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub enum_u64: Option<EnumU64>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub enum_i32: Option<EnumI32>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub enum_i64: Option<EnumI64>,
}

/// Explicitly assigned strings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// Implicit naming depending on the variant
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// Variants with long names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumU32 {
  Min,
  Max,
}

impl EnumU32 {
  pub fn value(&self) -> u32 {
    use self::EnumU32::*;
    match *self {
      Min => 0,
      Max => 2147483647,
    }
  }
}

impl serde::Serialize for EnumU32 {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
  use self::EnumU32::*;
    let o = match *self {
      Min => 0u32,
      Max => 2147483647u32,
    };
    s.serialize_u32(o)
  }
}

impl<'de> serde::Deserialize<'de> for EnumU32 {
  fn deserialize<D>(d: D) -> Result<EnumU32, D::Error>
    where D: serde::Deserializer<'de>
  {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
      type Value = EnumU32;

      fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("EnumU32, one of: 0, 2147483647")
      }

      fn visit_u32<E>(self, value: u32) -> Result<EnumU32, E>
        where E: de::Error
      {
        match value {
          0u32 => Ok(EnumU32::Min),
          2147483647u32 => Ok(EnumU32::Max),
          value => Err(E::custom(format!("EnumU32: unknown value: {}", value))),
        }
      }

      fn visit_u64<E>(self, value: u64) -> Result<EnumU32, E>
        where E: de::Error
      {
        self.visit_u32(value as u32)
      }
    }

    d.deserialize_u32(Visitor)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumU64 {
  Min,
  Max,
}

impl EnumU64 {
  pub fn value(&self) -> u64 {
    use self::EnumU64::*;
    match *self {
      Min => 0,
      Max => 9007199254740991,
    }
  }
}

impl serde::Serialize for EnumU64 {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
  use self::EnumU64::*;
    let o = match *self {
      Min => 0u64,
      Max => 9007199254740991u64,
    };
    s.serialize_u64(o)
  }
}

impl<'de> serde::Deserialize<'de> for EnumU64 {
  fn deserialize<D>(d: D) -> Result<EnumU64, D::Error>
    where D: serde::Deserializer<'de>
  {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
      type Value = EnumU64;

      fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("EnumU64, one of: 0, 9007199254740991")
      }

      fn visit_u64<E>(self, value: u64) -> Result<EnumU64, E>
        where E: de::Error
      {
        match value {
          0u64 => Ok(EnumU64::Min),
          9007199254740991u64 => Ok(EnumU64::Max),
          value => Err(E::custom(format!("EnumU64: unknown value: {}", value))),
        }
      }
    }

    d.deserialize_u64(Visitor)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumI32 {
  Min,
  NegativeOne,
  Zero,
  Max,
}

impl EnumI32 {
  pub fn value(&self) -> i32 {
    use self::EnumI32::*;
    match *self {
      Min => -2147483648,
      NegativeOne => -1,
      Zero => 0,
      Max => 2147483647,
    }
  }
}

impl serde::Serialize for EnumI32 {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
  use self::EnumI32::*;
    let o = match *self {
      Min => -2147483648i32,
      NegativeOne => -1i32,
      Zero => 0i32,
      Max => 2147483647i32,
    };
    s.serialize_i32(o)
  }
}

impl<'de> serde::Deserialize<'de> for EnumI32 {
  fn deserialize<D>(d: D) -> Result<EnumI32, D::Error>
    where D: serde::Deserializer<'de>
  {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
      type Value = EnumI32;

      fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("EnumI32, one of: -2147483648, -1, 0, 2147483647")
      }

      fn visit_i32<E>(self, value: i32) -> Result<EnumI32, E>
        where E: de::Error
      {
        match value {
          -2147483648i32 => Ok(EnumI32::Min),
          -1i32 => Ok(EnumI32::NegativeOne),
          0i32 => Ok(EnumI32::Zero),
          2147483647i32 => Ok(EnumI32::Max),
          value => Err(E::custom(format!("EnumI32: unknown value: {}", value))),
        }
      }

      fn visit_i64<E>(self, value: i64) -> Result<EnumI32, E>
        where E: de::Error
      {
        self.visit_i32(value as i32)
      }

      fn visit_u64<E>(self, value: u64) -> Result<EnumI32, E>
        where E: de::Error
      {
        self.visit_i32(value as i32)
      }
    }

    d.deserialize_i32(Visitor)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumI64 {
  Min,
  NegativeOne,
  Zero,
  Max,
}

impl EnumI64 {
  pub fn value(&self) -> i64 {
    use self::EnumI64::*;
    match *self {
      Min => -9007199254740991,
      NegativeOne => -1,
      Zero => 0,
      Max => 9007199254740991,
    }
  }
}

impl serde::Serialize for EnumI64 {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
  use self::EnumI64::*;
    let o = match *self {
      Min => -9007199254740991i64,
      NegativeOne => -1i64,
      Zero => 0i64,
      Max => 9007199254740991i64,
    };
    s.serialize_i64(o)
  }
}

impl<'de> serde::Deserialize<'de> for EnumI64 {
  fn deserialize<D>(d: D) -> Result<EnumI64, D::Error>
    where D: serde::Deserializer<'de>
  {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
      type Value = EnumI64;

      fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("EnumI64, one of: -9007199254740991, -1, 0, 9007199254740991")
      }

      fn visit_i64<E>(self, value: i64) -> Result<EnumI64, E>
        where E: de::Error
      {
        match value {
          -9007199254740991i64 => Ok(EnumI64::Min),
          -1i64 => Ok(EnumI64::NegativeOne),
          0i64 => Ok(EnumI64::Zero),
          9007199254740991i64 => Ok(EnumI64::Max),
          value => Err(E::custom(format!("EnumI64: unknown value: {}", value))),
        }
      }

      fn visit_u64<E>(self, value: u64) -> Result<EnumI64, E>
        where E: de::Error
      {
        self.visit_i64(value as i64)
      }
    }

    d.deserialize_i64(Visitor)
  }
}
