use super::*;

#[derive(Debug, Clone, Serialize)]
pub enum RpPathSegment {
    Literal { value: Loc<String> },
    Variable { name: Loc<String>, ty: Loc<RpType> },
}

impl RpPathSegment {
    pub fn path(&self) -> String {
        match *self {
            RpPathSegment::Literal { ref value } => value.as_ref().to_owned(),
            RpPathSegment::Variable { ref name, .. } => format!("{{{}}}", name.as_ref()),
        }
    }

    pub fn id(&self) -> &str {
        match *self {
            RpPathSegment::Literal { ref value } => value.as_ref().as_ref(),
            RpPathSegment::Variable { ref name, .. } => name.as_ref().as_ref(),
        }
    }
}
