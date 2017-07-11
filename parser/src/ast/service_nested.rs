use super::*;

#[derive(Debug)]
pub enum ServiceNested<'input> {
    Endpoint {
        method: Option<Loc<&'input str>>,
        path: Option<Loc<PathSpec<'input>>>,
        comment: Vec<&'input str>,
        options: Vec<Loc<OptionDecl<'input>>>,
        children: Vec<ServiceNested<'input>>,
    },
    Returns {
        comment: Vec<&'input str>,
        ty: Option<Loc<RpType>>,
        options: Vec<Loc<OptionDecl<'input>>>,
    },
    Accepts {
        comment: Vec<&'input str>,
        ty: Loc<RpType>,
        options: Vec<Loc<OptionDecl<'input>>>,
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
