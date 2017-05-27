use std::path::Path;
use super::convert_to_model::ConvertToModel;
use super::errors::*;

pub trait IntoModel {
    type Output;

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
