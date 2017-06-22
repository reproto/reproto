use std::collections::HashSet;
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct TypeBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<AstLoc<'input, Member<'input>>>,
}

impl<'input> IntoModel for TypeBody<'input> {
    type Output = Rc<RpTypeBody>;

    fn into_model(self) -> Result<Rc<RpTypeBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(self.members)?;

        let options = Options::new(options);

        let reserved: HashSet<RpLoc<String>> =
            options.find_all_identifiers("reserved")?.into_iter().collect();

        let type_body = RpTypeBody {
            name: self.name.to_owned(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            fields: fields,
            codes: codes,
            match_decl: match_decl,
            reserved: reserved,
        };

        Ok(Rc::new(type_body))
    }
}
