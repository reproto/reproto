use super::*;

#[derive(Debug)]
pub enum ServiceNested<'input> {
    Endpoint {
        url: RpLoc<String>,
        comment: Vec<&'input str>,
        options: Vec<RpLoc<OptionDecl<'input>>>,
        children: Vec<ServiceNested<'input>>,
    },
    Star {
        comment: Vec<&'input str>,
        options: Vec<RpLoc<OptionDecl<'input>>>,
        children: Vec<ServiceNested<'input>>,
    },
    Returns {
        comment: Vec<&'input str>,
        ty: RpLoc<RpType>,
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
