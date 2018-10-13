//! Type of a model.

use errors::Result;
use regex::Regex;
use serde::Serialize;
use std::fmt;
use {BigInt, CoreFlavor, Flavor, Loc, RpEnumType, RpName, RpNumber};

/// Describes number validation.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct RpNumberValidate {
    pub min: Option<RpNumber>,
    pub max: Option<RpNumber>,
}

impl RpNumberValidate {
    /// Check if the validation rules are empty.
    pub fn is_empty(&self) -> bool {
        self.min.is_none() && self.max.is_none()
    }
}

/// Describes string validation.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct RpStringValidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Regex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

impl RpStringValidate {
    /// Check if the validation rules are empty.
    pub fn is_empty(&self) -> bool {
        self.pattern.is_none() && self.min_length.is_none() && self.max_length.is_none()
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum RpNumberKind {
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "u64")]
    U64,
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "i64")]
    I64,
}

impl fmt::Display for RpNumberKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::RpNumberKind::*;

        match *self {
            U32 => "u32".fmt(fmt),
            U64 => "u64".fmt(fmt),
            I32 => "i32".fmt(fmt),
            I64 => "i64".fmt(fmt),
        }
    }
}

/// A number type.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RpNumberType {
    pub kind: RpNumberKind,
    #[serde(skip_serializing_if = "RpNumberValidate::is_empty")]
    pub validate: RpNumberValidate,
}

impl RpNumberType {
    /// Validate that the given number doesn't violate expected numeric bounds.
    pub fn validate_number(&self, number: &RpNumber) -> Result<()> {
        // max contiguous whole number that can be represented with a double: 2^53 - 1
        const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991i64;
        const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991i64;

        // TODO: calculate numeric bounds instead of switching over a couple of well-known ones.
        let (mn, mx): (BigInt, BigInt) = match self.kind {
            RpNumberKind::U32 => (0u32.into(), i32::max_value().into()),
            RpNumberKind::U64 => (0u64.into(), MAX_SAFE_INTEGER.into()),
            RpNumberKind::I32 => (i32::min_value().into(), i32::max_value().into()),
            RpNumberKind::I64 => (MIN_SAFE_INTEGER.into(), MAX_SAFE_INTEGER.into()),
        };

        let n = number.to_bigint().ok_or_else(|| "not a whole number")?;

        // withing bounds
        if mn <= *n && *n <= mx {
            return Ok(());
        }

        Err(format!("number is not within {} to {} (inclusive)", mn, mx).into())
    }
}

impl fmt::Display for RpNumberType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(fmt)
    }
}

/// Describes a string type.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct RpStringType {
    #[serde(skip_serializing_if = "RpStringValidate::is_empty")]
    pub validate: RpStringValidate,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(bound = "F::Package: Serialize")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpType<F: 'static>
where
    F: Flavor,
{
    Double,
    Float,
    Number(RpNumberType),
    Boolean,
    String(RpStringType),
    /// ISO-8601 datetime
    DateTime,
    Bytes,
    Any,
    Argument {
        argument: Loc<String>,
    },
    Name {
        name: Loc<RpName<F>>,
    },
    Array {
        inner: Box<RpType<F>>,
    },
    Map {
        key: Box<RpType<F>>,
        value: Box<RpType<F>>,
    },
}

impl<F: 'static> RpType<F>
where
    F: Flavor,
{
    /// Convert to an enum variant type.
    pub fn as_enum_type(&self) -> Option<RpEnumType> {
        use self::RpType::*;

        match *self {
            String(ref string) => Some(RpEnumType::String(string.clone())),
            Number(ref number) => Some(RpEnumType::Number(number.clone())),
            _ => None,
        }
    }

    /// Modify any name components with the given operation.
    fn with_name<M>(self, f: M) -> Self
    where
        M: Clone + Fn(RpName<F>) -> RpName<F>,
    {
        use self::RpType::*;

        match self {
            Name { name } => Name {
                name: Loc::map(name, f),
            },
            Array { inner } => Array {
                inner: Box::new(inner.with_name(f)),
            },
            Map { key, value } => Map {
                key: Box::new(key.with_name(f.clone())),
                value: Box::new(value.with_name(f.clone())),
            },
            ty => ty,
        }
    }
}

impl RpType<CoreFlavor> {
    /// Localize type.
    ///
    /// Strips version of any type which is _not_ imported.
    pub fn localize(self) -> Self {
        self.with_name(RpName::localize)
    }

    /// Strip version component for any type.
    pub fn without_version(self) -> Self {
        self.with_name(RpName::without_version)
    }
}

impl<F: 'static> fmt::Display for RpType<F>
where
    F: Flavor,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpType::*;

        match *self {
            Double => write!(f, "double"),
            Float => write!(f, "float"),
            Number(ref number) => write!(f, "{}", number),
            Boolean => write!(f, "boolean"),
            String(..) => write!(f, "string"),
            DateTime => write!(f, "datetime"),
            Argument { ref argument } => write!(f, "{}", argument),
            Name { ref name } => write!(f, "{}", name),
            Array { ref inner } => write!(f, "[{}]", inner),
            Map { ref key, ref value } => write!(f, "{{{}: {}}}", key, value),
            Any => write!(f, "any"),
            Bytes => write!(f, "bytes"),
        }
    }
}
