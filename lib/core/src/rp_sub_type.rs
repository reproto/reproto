//! Model for sub-types

use errors::Result;
use serde::Serialize;
use translator;
use {Flavor, Loc, RpCode, RpDecl, RpReg, Translate, Translator};

#[derive(Debug, Clone, Serialize)]
#[serde(bound = "F: Serialize, F::Field: Serialize, F::Endpoint: Serialize, F::Package: \
                 Serialize, F::Name: Serialize")]
pub struct RpSubType<F: 'static>
where
    F: Flavor,
{
    pub name: F::Name,
    pub ident: String,
    pub comment: Vec<String>,
    /// Inner declarations.
    pub decls: Vec<RpDecl<F>>,
    pub fields: Vec<Loc<F::Field>>,
    pub codes: Vec<Loc<RpCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type_name: Option<Loc<String>>,
}

impl<F: 'static> RpSubType<F>
where
    F: Flavor,
{
    pub fn name(&self) -> &str {
        self.sub_type_name
            .as_ref()
            .map(|t| t.as_str())
            .unwrap_or(&self.ident)
    }
}

impl<F: 'static, T> Translate<T> for RpSubType<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpSubType<T::Target>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<RpSubType<T::Target>> {
        translator.visit(&self.name)?;

        let name = translator.translate_local_name(RpReg::SubType, self.name)?;

        Ok(RpSubType {
            name: name,
            ident: self.ident,
            comment: self.comment,
            decls: self.decls.translate(translator)?,
            fields: translator::Fields(self.fields).translate(translator)?,
            codes: self.codes,
            sub_type_name: self.sub_type_name,
        })
    }
}
