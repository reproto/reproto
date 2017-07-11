use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
pub struct RpMatchDecl {
    pub by_value: Vec<(Loc<RpValue>, RpByValueMatch)>,
    pub by_type: Vec<(RpMatchKind, RpByTypeMatch)>,
}

impl RpMatchDecl {
    pub fn new() -> RpMatchDecl {
        RpMatchDecl {
            by_value: Vec::new(),
            by_type: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.by_value.is_empty() && self.by_type.is_empty()
    }

    pub fn identify_match_kind(&self, variable: &RpMatchVariable) -> RpMatchKind {
        match variable.ty {
            RpType::Double |
            RpType::Float |
            RpType::Signed { size: _ } |
            RpType::Unsigned { size: _ } => RpMatchKind::Number,
            RpType::Boolean => RpMatchKind::Boolean,
            RpType::String | RpType::Bytes => RpMatchKind::String,
            RpType::Any => RpMatchKind::Any,
            RpType::Name { name: _ } |
            RpType::Map { key: _, value: _ } => RpMatchKind::Object,
            RpType::Array { inner: _ } => RpMatchKind::Array,
        }
    }

    pub fn push(&mut self, member: Loc<RpMatchMember>) -> Result<()> {
        match *member.condition {
            RpMatchCondition::Type(ref variable) => {
                let match_kind = self.identify_match_kind(variable);

                {
                    // conflicting when type matches
                    let result =
                        self.by_type.iter().find(|e| e.0 == match_kind || e.0 == RpMatchKind::Any);

                    if let Some(&(_, ref existing_value)) = result {
                        let err = ErrorKind::MatchConflict(member.condition.pos().into(),
                                                           existing_value.instance.pos().into());
                        return Err(err.into());
                    }
                }

                self.by_type.push((match_kind,
                                   RpByTypeMatch {
                                       variable: variable.clone(),
                                       instance: member.value.clone(),
                                   }));
            }
            RpMatchCondition::Value(ref value) => {
                {
                    // conflicting when value matches
                    let result = self.by_value.iter().find(|e| e.0.as_ref() == value.as_ref());

                    if let Some(&(_, ref existing_value)) = result {
                        let err = ErrorKind::MatchConflict(member.condition.pos().into(),
                                                           existing_value.instance.pos().into());
                        return Err(err.into());
                    }
                }

                self.by_value
                    .push((value.clone(), RpByValueMatch { instance: member.value.clone() }));
            }
        }

        Ok(())
    }
}
