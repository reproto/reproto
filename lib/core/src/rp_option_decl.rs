//! Option declarations

use errors::Result;
use loc::Loc;
use option_entry::OptionEntry;
use rp_number::RpNumber;
use rp_value::RpValue;

#[derive(Debug, Clone, Serialize)]
pub struct RpOptionDecl {
    pub name: String,
    pub value: Loc<RpValue>,
}

impl OptionEntry for RpOptionDecl {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_string(&self) -> Result<String> {
        match *Loc::value(&self.value) {
            RpValue::String(ref string) => Ok(string.to_string()),
            _ => Err("expected string".into()),
        }
    }

    fn as_number(&self) -> Result<RpNumber> {
        match *Loc::value(&self.value) {
            RpValue::Number(ref number) => Ok(number.clone()),
            _ => Err("expected number".into()),
        }
    }

    fn as_identifier(&self) -> Result<String> {
        match *Loc::value(&self.value) {
            RpValue::Identifier(ref identifier) => Ok(identifier.to_string()),
            _ => Err("expected identifier".into()),
        }
    }
}
