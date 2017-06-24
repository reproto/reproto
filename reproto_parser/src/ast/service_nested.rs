use super::*;

#[derive(Debug)]
pub enum ServiceNested<'input> {
    Endpoint {
        method: Option<RpLoc<&'input str>>,
        path: Option<RpLoc<PathSpec<'input>>>,
        comment: Vec<&'input str>,
        options: Vec<RpLoc<OptionDecl<'input>>>,
        children: Vec<ServiceNested<'input>>,
    },
    Returns {
        comment: Vec<&'input str>,
        ty: Option<RpLoc<RpType>>,
        options: Vec<RpLoc<OptionDecl<'input>>>,
    },
    Accepts {
        comment: Vec<&'input str>,
        ty: RpLoc<RpType>,
        options: Vec<RpLoc<OptionDecl<'input>>>,
    },
}

impl<'input> ServiceNested<'input> {
    pub fn is_terminus(&self) -> bool {
        match *self {
            ServiceNested::Returns { .. } => true,
            ServiceNested::Accepts { .. } => true,
            _ => false,
        }
    }
}
