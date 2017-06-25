use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpEnumBody {
    pub name: String,
    pub comment: Vec<String>,
    pub variants: Vec<RpLoc<Rc<RpEnumVariant>>>,
    pub fields: Vec<RpLoc<RpField>>,
    pub codes: Vec<RpLoc<RpCode>>,
    pub match_decl: RpMatchDecl,
    pub serialized_as: Option<RpLoc<String>>,
    pub serialized_as_name: bool,
}

impl Merge for RpEnumBody {
    fn merge(&mut self, source: RpEnumBody) -> Result<()> {
        self.codes.merge(source.codes)?;
        Ok(())
    }
}
