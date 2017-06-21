use std::collections::HashSet;
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct TypeBody<'a> {
    pub name: &'a str,
    pub comment: Vec<&'a str>,
    pub members: Vec<AstLoc<Member<'a>>>,
}

impl<'a> IntoModel for TypeBody<'a> {
    type Output = Rc<RpTypeBody>;

    fn into_model(self, path: &Path) -> Result<Rc<RpTypeBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(path, self.members)?;

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
