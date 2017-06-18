//! Implementations for converting asts into models.

pub use std::path::Path;
use super::errors::*;

/// Adds a method for all types that supports conversion into core types.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, pos: &Path) -> Result<Self::Output>;
}

/// Generic implementation for vectors.
impl<T> IntoModel for Vec<T>
    where T: IntoModel
{
    type Output = Vec<T::Output>;

    fn into_model(self, pos: &Path) -> Result<Self::Output> {
        let mut out = Vec::new();

        for v in self {
            out.push(v.into_model(pos)?);
        }

        Ok(out)
    }
}

impl<T> IntoModel for Option<T>
    where T: IntoModel
{
    type Output = Option<T::Output>;

    fn into_model(self, pos: &Path) -> Result<Self::Output> {
        if let Some(value) = self {
            return Ok(Some(value.into_model(pos)?));
        }

        Ok(None)
    }
}

impl IntoModel for String {
    type Output = String;

    fn into_model(self, _pos: &Path) -> Result<String> {
        Ok(self)
    }
}
