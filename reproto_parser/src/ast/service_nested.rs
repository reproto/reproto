use super::*;

#[derive(Debug)]
pub enum ServiceNested {
    Endpoint {
        url: AstLoc<String>,
        comment: Vec<String>,
        options: Vec<AstLoc<OptionDecl>>,
        children: Vec<ServiceNested>,
    },
    Star {
        comment: Vec<String>,
        options: Vec<AstLoc<OptionDecl>>,
        children: Vec<ServiceNested>,
    },
    Returns {
        comment: Vec<String>,
        ty: AstLoc<RpType>,
        options: Vec<AstLoc<OptionDecl>>,
    },
}

impl ServiceNested {
    pub fn is_returns(&self) -> bool {
        match *self {
            ServiceNested::Returns { .. } => true,
            _ => false,
        }
    }
}
