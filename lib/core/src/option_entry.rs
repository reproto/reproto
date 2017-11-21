use super::RpNumber;

pub trait OptionEntry {
    /// Get the name of the option.
    fn name(&self) -> &str;

    /// Get the value as a string.
    fn as_string(&self) -> Result<String, &'static str>;

    /// Get the value as an 32-bit unsigned integer.
    fn as_number(&self) -> Result<RpNumber, &'static str>;

    /// Get the value as an identifier.
    fn as_identifier(&self) -> Result<String, &'static str>;
}
