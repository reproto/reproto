//! Type of a model.

use serde::Serialize;
use std::fmt;
use {CoreFlavor, Flavor, RpEnumType, RpName};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(bound = "F::Package: Serialize")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpType<F: 'static>
where
    F: Flavor,
{
    Double,
    Float,
    Signed {
        size: usize,
    },
    Unsigned {
        size: usize,
    },
    Boolean,
    String,
    /// ISO-8601 datetime
    DateTime,
    Bytes,
    Any,
    Name {
        name: RpName<F>,
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
            String => Some(RpEnumType::String),
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
            Name { name } => Name { name: f(name) },
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
            Signed { ref size } => write!(f, "i{}", size),
            Unsigned { ref size } => write!(f, "u{}", size),
            Boolean => write!(f, "boolean"),
            String => write!(f, "string"),
            DateTime => write!(f, "datetime"),
            Name { ref name } => write!(f, "{}", name),
            Array { ref inner } => write!(f, "[{}]", inner),
            Map { ref key, ref value } => write!(f, "{{{}: {}}}", key, value),
            Any => write!(f, "any"),
            Bytes => write!(f, "bytes"),
        }
    }
}
