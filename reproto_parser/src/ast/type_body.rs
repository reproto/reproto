use std::collections::HashSet;
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct TypeBody {
    pub name: String,
    pub comment: Vec<String>,
    pub members: Vec<AstLoc<Member>>,
}

impl IntoModel for TypeBody {
    type Output = Rc<RpTypeBody>;

    fn into_model(self, path: &Path) -> Result<Rc<RpTypeBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(path, self.members)?;

        let options = Options::new(options);

        let reserved: HashSet<RpLoc<String>> =
            options.find_all_identifiers("reserved")?.into_iter().collect();

        let type_body = RpTypeBody {
            name: self.name,
            comment: self.comment,
            fields: fields,
            codes: codes,
            match_decl: match_decl,
            reserved: reserved,
        };

        Ok(Rc::new(type_body))
    }
}
