mod decl;
mod package_decl;
mod utils;
mod enum_body;
mod enum_variant;
mod field;
mod field_init;
mod file;
mod instance;
mod interface_body;
mod match_condition;
mod match_decl;
mod match_member;
mod match_variable;
mod member;
mod option_decl;
mod service_body;
mod sub_type;
mod tuple_body;
mod type_body;
mod use_decl;
mod value;
mod into_model;

pub use reproto_core::*;
pub use self::decl::*;
pub use self::enum_body::*;
pub use self::enum_variant::*;
pub use self::field::*;
pub use self::field_init::*;
pub use self::file::*;
pub use self::instance::*;
pub use self::interface_body::*;
pub use self::into_model::*;
pub use self::match_condition::*;
pub use self::match_decl::*;
pub use self::match_member::*;
pub use self::match_variable::*;
pub use self::member::*;
pub use self::option_decl::*;
pub use self::package_decl::*;
pub use self::service_body::*;
pub use self::sub_type::*;
pub use self::tuple_body::*;
pub use self::type_body::*;
pub use self::use_decl::*;
pub use self::value::*;
pub(crate) use super::errors;

/// Position relative in file where the declaration is present.
pub type Pos = (usize, usize);
pub type AstLoc<T> = Loc<T, Pos>;

use std::path::Path;

impl<T> IntoModel for Loc<T, (usize, usize)>
    where T: IntoModel
{
    type Output = RpLoc<T::Output>;

    fn into_model(self, path: &Path) -> errors::Result<Self::Output> {
        let (value, pos) = self.both();
        let value = value.into_model(path)?;
        let pos = (path.to_owned(), pos.0, pos.1);
        Ok(RpLoc::new(value, pos))
    }
}

impl IntoModel for RpName {
    type Output = RpName;

    fn into_model(self, _pos: &Path) -> errors::Result<Self::Output> {
        Ok(self)
    }
}
