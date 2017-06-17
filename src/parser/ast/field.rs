use super::*;

#[derive(Debug)]
pub struct Field {
    pub modifier: RpModifier,
    pub name: String,
    pub comment: Vec<String>,
    pub ty: RpType,
    pub field_as: Option<AstLoc<Value>>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        match self.modifier {
            RpModifier::Optional => true,
            _ => false,
        }
    }
}
