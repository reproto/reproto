use super::*;

#[derive(Debug)]
pub enum ServiceNested<'input> {
    Endpoint {
        url: AstLoc<'input, String>,
        comment: Vec<&'input str>,
        options: Vec<AstLoc<'input, OptionDecl<'input>>>,
        children: Vec<ServiceNested<'input>>,
    },
    Star {
        comment: Vec<&'input str>,
        options: Vec<AstLoc<'input, OptionDecl<'input>>>,
        children: Vec<ServiceNested<'input>>,
    },
    Returns {
        comment: Vec<&'input str>,
        ty: AstLoc<'input, RpType>,
        options: Vec<AstLoc<'input, OptionDecl<'input>>>,
    },
    Accepts {
        comment: Vec<&'input str>,
        ty: AstLoc<'input, RpType>,
        options: Vec<AstLoc<'input, OptionDecl<'input>>>,
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
