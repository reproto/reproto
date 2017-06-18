use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct TupleBody {
    pub name: String,
    pub comment: Vec<String>,
    pub members: Vec<AstLoc<Member>>,
}

impl IntoModel for TupleBody {
    type Output = Rc<RpTupleBody>;

    fn into_model(self, path: &Path) -> Result<Rc<RpTupleBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(path, self.members)?;

        let _options = Options::new(options);

        let tuple_body = RpTupleBody {
            name: self.name,
            comment: self.comment,
            fields: fields,
            codes: codes,
            match_decl: match_decl,
        };

        Ok(Rc::new(tuple_body))
    }
}
