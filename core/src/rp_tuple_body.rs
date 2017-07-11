use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpTupleBody {
    pub name: String,
    pub comment: Vec<String>,
    pub fields: Vec<Loc<RpField>>,
    pub codes: Vec<Loc<RpCode>>,
    pub match_decl: RpMatchDecl,
}

impl RpTupleBody {
    pub fn fields<'a>(&'a self) -> Box<Iterator<Item = &Loc<RpField>> + 'a> {
        Box::new(self.fields.iter())
    }
}

impl Merge for RpTupleBody {
    fn merge(&mut self, source: RpTupleBody) -> Result<()> {
        self.fields.merge(source.fields)?;
        self.codes.merge(source.codes)?;
        Ok(())
    }
}
