use std::collections::{BTreeMap, btree_map};
use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct InterfaceBody<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<RpLoc<Member<'input>>>,
    pub sub_types: Vec<RpLoc<SubType<'input>>>,
}

impl<'input> IntoModel for InterfaceBody<'input> {
    type Output = Rc<RpInterfaceBody>;

    fn into_model(self) -> Result<Rc<RpInterfaceBody>> {
        let (fields, codes, options, match_decl) = utils::members_into_model(self.members)?;

        let mut sub_types: BTreeMap<String, RpLoc<Rc<RpSubType>>> = BTreeMap::new();

        for sub_type in self.sub_types.into_model()? {
            // key has to be owned by entry
            let key = sub_type.name.clone();

            match sub_types.entry(key) {
                btree_map::Entry::Occupied(entry) => {
                    entry.into_mut().merge(sub_type)?;
                }
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(sub_type);
                }
            }
        }

        let _options = Options::new(options);

        let interface_body = RpInterfaceBody {
            name: self.name.to_owned(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            fields: fields,
            codes: codes,
            match_decl: match_decl,
            sub_types: sub_types,
        };

        Ok(Rc::new(interface_body))
    }
}
