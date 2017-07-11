use super::*;
use super::errors::*;

#[derive(Debug)]
pub enum PathSegment<'input> {
    Literal { value: Loc<String> },
    Variable {
        name: Loc<&'input str>,
        ty: Loc<RpType>,
    },
}

impl<'input> IntoModel for PathSegment<'input> {
    type Output = RpPathSegment;

    fn into_model(self) -> Result<RpPathSegment> {
        let out = match self {
            PathSegment::Literal { value } => RpPathSegment::Literal { value: value.into_model()? },
            PathSegment::Variable { name, ty } => {
                RpPathSegment::Variable {
                    name: name.into_model()?,
                    ty: ty.into_model()?,
                }
            }
        };

        Ok(out)
    }
}
