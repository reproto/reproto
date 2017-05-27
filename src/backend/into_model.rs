use std::path::Path;
use super::convert_to_model::ConvertToModel;
use super::errors::*;

/// Adds the into_model() method for all types that supports ConvertToModel.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, path: &Path) -> Result<Self::Output>;
}

impl<T, O> IntoModel for T
    where T: ConvertToModel<Output = O>
{
    type Output = O;

    fn into_model(self, path: &Path) -> Result<O> {
        ConvertToModel::convert_to_model(self, path)
    }
}
