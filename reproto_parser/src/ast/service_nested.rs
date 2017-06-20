use super::*;

#[derive(Debug)]
pub enum ServiceNested {
    Endpoint {
        comment: Vec<String>,
        options: Vec<AstLoc<OptionDecl>>,
        children: Vec<ServiceNested>,
    },
    Response {
        comment: Vec<String>,
        ty: Option<AstLoc<RpType>>,
        options: Vec<AstLoc<OptionDecl>>,
    },
}
