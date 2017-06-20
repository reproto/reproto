use super::*;

#[derive(Debug)]
pub enum ServiceNested {
    Endpoint {
        name: AstLoc<String>,
        comment: Vec<String>,
        options: Vec<AstLoc<OptionDecl>>,
        children: Vec<ServiceNested>,
    },
    Response {
        name: AstLoc<String>,
        comment: Vec<String>,
        ty: AstLoc<RpType>,
        options: Vec<AstLoc<OptionDecl>>,
        children: Vec<ServiceNested>,
    },
}
