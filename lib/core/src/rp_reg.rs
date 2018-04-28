//! Model for registered types.

use std::fmt;
use {Flavor, RpName};

/// Marker for the existence of a registered type of the given kind.
#[derive(Debug, Clone)]
pub enum RpReg {
    Type,
    Tuple,
    Interface,
    SubType,
    Enum,
    EnumVariant,
    Service,
}

impl RpReg {
    pub fn ident<PackageFn, InnerFn, F: 'static>(
        &self,
        name: &RpName<F>,
        package_fn: PackageFn,
        inner_fn: InnerFn,
    ) -> String
    where
        PackageFn: Fn(Vec<&str>) -> String,
        InnerFn: Fn(Vec<&str>) -> String,
        F: Flavor,
    {
        use self::RpReg::*;

        match *self {
            Type | Interface | Enum | Tuple | Service => {
                let p = name.path.iter().map(String::as_str).collect();
                package_fn(p)
            }
            SubType | EnumVariant => {
                let mut v: Vec<&str> = name.path.iter().map(String::as_str).collect();
                let at = v.len().saturating_sub(2);
                let last = inner_fn(v.split_off(at));

                let mut path = v.clone();
                path.push(last.as_str());

                inner_fn(path)
            }
        }
    }

    /// Check if registered type is an enum.
    pub fn is_enum(&self) -> bool {
        use self::RpReg::*;

        match *self {
            Enum => true,
            _ => false,
        }
    }
}

impl fmt::Display for RpReg {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::RpReg::*;

        match *self {
            Type => write!(fmt, "type"),
            Interface => write!(fmt, "interface"),
            Enum => write!(fmt, "enum"),
            Tuple => write!(fmt, "tuple"),
            Service => write!(fmt, "service"),
            SubType => write!(fmt, "subtype"),
            EnumVariant => write!(fmt, "variant"),
        }
    }
}
