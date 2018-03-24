//! Model for tuples.

use {Flavor, Loc, RpCode, RpField, RpSubType};
use std::rc::Rc;
use std::slice;

/// Default key to use for tagged sub type strategy.
pub const DEFAULT_TAG: &str = "type";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpSubTypeStrategy {
    /// An object, with a single tag key indicating which sub-type to use.
    Tagged { tag: String },
}

impl Default for RpSubTypeStrategy {
    fn default() -> Self {
        RpSubTypeStrategy::Tagged {
            tag: DEFAULT_TAG.to_string(),
        }
    }
}

decl_body!(pub struct RpInterfaceBody<F> {
    pub fields: Vec<Loc<RpField<F>>>,
    pub codes: Vec<Loc<RpCode>>,
    pub sub_types: Vec<Rc<Loc<RpSubType<F>>>>,
    pub sub_type_strategy: RpSubTypeStrategy,
});

/// Iterator over fields.
pub struct Fields<'a, F: 'static>
where
    F: Flavor,
{
    iter: slice::Iter<'a, Loc<RpField<F>>>,
}

impl<'a, F: 'static> Iterator for Fields<'a, F>
where
    F: Flavor,
{
    type Item = &'a Loc<RpField<F>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<F: 'static> RpInterfaceBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> Fields<F> {
        Fields {
            iter: self.fields.iter(),
        }
    }
}
