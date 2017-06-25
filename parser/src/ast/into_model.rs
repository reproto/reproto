//! Implementations for converting asts into models.

pub use std::path::{Path, PathBuf};
use super::*;
use super::errors::*;

/// Adds a method for all types that supports conversion into core types.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self) -> Result<Self::Output>;
}

/// Generic implementation for vectors.
impl<T> IntoModel for RpLoc<T>
    where T: IntoModel
{
    type Output = RpLoc<T::Output>;

    fn into_model(self) -> Result<Self::Output> {
        let (value, pos) = self.both();
        Ok(RpLoc::new(value.into_model()?, pos))
    }
}

/// Generic implementation for vectors.
impl<T> IntoModel for Vec<T>
    where T: IntoModel
{
    type Output = Vec<T::Output>;

    fn into_model(self) -> Result<Self::Output> {
        let mut out = Vec::new();

        for v in self {
            out.push(v.into_model()?);
        }

        Ok(out)
    }
}

impl<T> IntoModel for Option<T>
    where T: IntoModel
{
    type Output = Option<T::Output>;

    fn into_model(self) -> Result<Self::Output> {
        if let Some(value) = self {
            return Ok(Some(value.into_model()?));
        }

        Ok(None)
    }
}

impl<'a> IntoModel for &'a str {
    type Output = String;

    fn into_model(self) -> Result<Self::Output> {
        Ok(self.to_owned())
    }
}

impl IntoModel for String {
    type Output = String;

    fn into_model(self) -> Result<Self::Output> {
        Ok(self)
    }
}

impl IntoModel for RpPackage {
    type Output = RpPackage;

    fn into_model(self) -> Result<Self::Output> {
        Ok(self)
    }
}

impl IntoModel for RpType {
    type Output = RpType;

    fn into_model(self) -> Result<Self::Output> {
        Ok(self)
    }
}

impl IntoModel for RpName {
    type Output = RpName;

    fn into_model(self) -> errors::Result<Self::Output> {
        Ok(self)
    }
}

impl<'input> IntoModel for (&'input Path, usize, usize) {
    type Output = (PathBuf, usize, usize);

    fn into_model(self) -> Result<Self::Output> {
        Ok((self.0.to_owned(), self.1, self.2))
    }
}
