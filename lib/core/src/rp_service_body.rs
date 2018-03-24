//! Model for services.

use super::{Flavor, Loc, RpEndpoint, Translate, Translator};
use errors::Result;

#[derive(Debug, Clone, Serialize, Default)]
pub struct RpServiceBodyHttp {
    /// Default URL to use for service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Loc<String>>,
}

decl_body!(pub struct RpServiceBody<F> {
    pub http: RpServiceBodyHttp,
    pub endpoints: Vec<Loc<RpEndpoint<F>>>,
});

impl<F: 'static, T> Translate<T> for RpServiceBody<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpServiceBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<RpServiceBody<T::Target>> {
        translator.visit(&self.name)?;

        Ok(RpServiceBody {
            name: self.name,
            ident: self.ident,
            comment: self.comment,
            decls: self.decls.translate(translator)?,
            http: self.http,
            endpoints: self.endpoints.translate(translator)?,
        })
    }
}
