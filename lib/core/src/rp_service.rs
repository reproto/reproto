//! Model for services.

use std::result;
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
    fn translate(
        self,
        diag: &mut Diagnostics,
        translator: &T,
    ) -> result::Result<RpServiceBody<T::Target>, ()> {
        let (name, span) = Loc::take_pair(self.name);
        try_diag!(diag, span, translator.visit(diag, &name));
        let name = Loc::new(
            translator.translate_local_name(diag, RpReg::Service, name)?,
            span,
        );

        let mut endpoints = Vec::new();

        for endpoint in self.endpoints {
            let (endpoint, span) = Loc::take_pair(endpoint);
            let endpoint = translator.translate_endpoint(diag, endpoint)?;
            endpoints.push(Loc::new(endpoint, span));
        }

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
