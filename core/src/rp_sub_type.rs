use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpSubType {
    pub name: String,
    pub comment: Vec<String>,
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    pub names: Vec<Loc<String>>,
    pub match_decl: RpMatchDecl,
}

impl RpSubType {
    pub fn name(&self) -> &str {
        self.names
            .iter()
            .map(|t| t.as_ref().as_str())
            .nth(0)
            .unwrap_or(&self.name)
    }

    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Loc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

impl Merge for RpSubType {
    fn merge(&mut self, source: RpSubType) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        self.names.extend(source.names);
        Ok(())
    }
}
