//! Model for services.

use errors::Result;
use {Diagnostics, Flavor, Loc, RpReg, Translate, Translator};

#[derive(Debug, Clone, Serialize, Default)]
pub struct RpServiceBodyHttp {
    /// Default URL to use for service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Loc<String>>,
}

decl_body!(pub struct RpServiceBody<F> {
    pub http: RpServiceBodyHttp,
    pub endpoints: Vec<Loc<F::Endpoint>>,
});

impl<F: 'static, T> Translate<T> for RpServiceBody<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = RpServiceBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpServiceBody<T::Target>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::Service, self.name)?;

        let endpoints = self
            .endpoints
            .into_iter()
            .map(|e| Loc::and_then(e, |e| translator.translate_endpoint(diag, e)))
            .collect::<Result<Vec<_>>>()?;

        let decls = self.decls.translate(diag, translator)?;

        Ok(RpServiceBody {
            name,
            ident: self.ident,
            comment: self.comment,
            decls,
            decl_idents: self.decl_idents,
            http: self.http,
            endpoints,
        })
    }
}
