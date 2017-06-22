use super::*;

pub enum RpPathFragment<'input> {
    Literal { value: RpLoc<String> },
    Variable {
        name: RpLoc<&'input str>,
        ty: RpLoc<RpType>,
    },
}
