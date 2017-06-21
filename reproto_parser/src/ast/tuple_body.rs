use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct TupleBody<'a> {
    pub name: &'a str,
    pub comment: Vec<&'a str>,
    pub members: Vec<AstLoc<Member<'a>>>,
}

impl<'a> IntoModel for TupleBody<'a> {
    type Output = Rc<RpTupleBody>;

    fn into_model(self, path: &Path) -> Result<Rc<RpTupleBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(path, self.members)?;

        let _options = Options::new(options);

        let tuple_body = RpTupleBody {
            name: self.name.to_owned(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            fields: fields,
            codes: codes,
            match_decl: match_decl,
        };

        Ok(Rc::new(tuple_body))
    }
}
