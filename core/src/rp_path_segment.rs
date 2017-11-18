//! Path specifications

use super::{Loc, RpType};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpPathSegment {
    Literal { value: Loc<String> },
    Variable { name: Loc<String>, ty: Loc<RpType> },
}

impl RpPathSegment {
    pub fn path(&self) -> String {
        use self::RpPathSegment::*;

        match *self {
            Literal { ref value } => value.value().to_owned(),
            Variable { ref name, .. } => format!("{{{}}}", name.value()),
        }
    }

    pub fn id(&self) -> &str {
        use self::RpPathSegment::*;

        match *self {
            Literal { ref value } => value.value().as_str(),
            Variable { ref name, .. } => name.value().as_str(),
        }
    }
}
