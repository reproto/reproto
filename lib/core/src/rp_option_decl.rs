//! Option declarations

use loc::Loc;
use option_entry::OptionEntry;
use rp_number::RpNumber;
use rp_value::RpValue;
use std::result;

#[derive(Debug, Clone, Serialize)]
pub struct RpOptionDecl {
    pub name: String,
    pub value: Loc<RpValue>,
}

impl OptionEntry for RpOptionDecl {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_string(&self) -> result::Result<String, &'static str> {
        match *self.value.value() {
            RpValue::String(ref string) => Ok(string.to_string()),
            _ => Err("expected string"),
        }
    }

    fn as_number(&self) -> result::Result<RpNumber, &'static str> {
        match *self.value.value() {
            RpValue::Number(ref number) => Ok(number.clone()),
            _ => Err("expected number"),
        }
    }

    fn as_identifier(&self) -> result::Result<String, &'static str> {
        match *self.value.value() {
            RpValue::Identifier(ref identifier) => Ok(identifier.to_string()),
            _ => Err("expected identifier"),
        }
    }
}
