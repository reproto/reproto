use core;
use core::errors::Result;
use sir::Sir;

///
/// Decoder to use.
pub trait Format {
    fn decode(&self, object: &core::Object) -> Result<Sir>;
}

/// Object accessor
pub trait Object {
    type Value;

    /// Get the value of the given key.
    fn get(&self, key: &str) -> Option<&Self::Value>;
}

pub trait Value {
    /// Attempt to convert the current value into an Object.
    fn as_object(&self) -> Option<&Object<Value = Self>>;

    /// Attempt to convert the current value into a String.
    fn as_str(&self) -> Option<&str>;
}
