use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct TupleBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<RpLoc<Member<'input>>>,
}

impl<'input> IntoModel for TupleBody<'input> {
    type Output = Rc<RpTupleBody>;

    fn into_model(self) -> Result<Rc<RpTupleBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(self.members)?;

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
