use std::collections::BTreeMap;
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpInterfaceBody {
    pub name: String,
    pub comment: Vec<String>,
    pub fields: Vec<RpLoc<RpField>>,
    pub codes: Vec<RpLoc<RpCode>>,
    pub match_decl: RpMatchDecl,
    pub sub_types: BTreeMap<String, RpLoc<Rc<RpSubType>>>,
}

impl RpInterfaceBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &RpLoc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

impl Merge for RpInterfaceBody {
    fn merge(&mut self, source: RpInterfaceBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        self.sub_types.merge(source.sub_types)?;
        Ok(())
    }
}
