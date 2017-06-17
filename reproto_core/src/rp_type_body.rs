use std::collections::HashSet;
use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpTypeBody {
    pub name: String,
    pub comment: Vec<String>,
    pub fields: Vec<RpLoc<RpField>>,
    pub codes: Vec<RpLoc<RpCode>>,
    pub match_decl: RpMatchDecl,
    // Set of fields which are reserved for this type.
    pub reserved: HashSet<RpLoc<String>>,
}

impl RpTypeBody {
    pub fn verify(&self) -> Result<()> {
        for reserved in &self.reserved {
            if let Some(field) = self.fields.iter().find(|f| f.name() == reserved.inner) {
                return Err(ErrorKind::ReservedField(field.pos.clone(), reserved.pos.clone())
                    .into());
            }
        }

        Ok(())
    }

    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &RpLoc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

impl Merge for RpTypeBody {
    fn merge(&mut self, source: RpTypeBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        Ok(())
    }
}
