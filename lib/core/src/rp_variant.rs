//! Variant in an enum

use errors::Result;
use serde::Serialize;
use {Flavor, Loc, RpEnumOrdinal, RpReg, Translate, Translator};

#[derive(Debug, Clone, Serialize)]
#[serde(bound = "F::Package: Serialize, F::Name: Serialize")]
pub struct RpVariant<F: 'static>
where
    F: Flavor,
{
    pub name: F::Name,
    pub ident: Loc<String>,
    pub comment: Vec<String>,
    pub ordinal: RpEnumOrdinal,
}

impl<F: 'static> RpVariant<F>
where
    F: Flavor,
{
    /// Get the identifier of the variant.
    pub fn ident(&self) -> &str {
        self.ident.as_str()
    }

    /// Get the ordinal value of the variant.
    pub fn ordinal(&self) -> &str {
        use self::RpEnumOrdinal::*;

        match self.ordinal {
            String(ref string) => string.as_str(),
            Generated => self.ident(),
        }
    }
}

impl<F: 'static, T> Translate<T> for RpVariant<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpVariant<T::Target>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<RpVariant<T::Target>> {
        translator.visit(&self.name)?;

        let name = translator.translate_local_name(RpReg::EnumVariant, self.name)?;

        Ok(RpVariant {
            name: name,
            ident: self.ident,
            comment: self.comment,
            ordinal: self.ordinal,
        })
    }
}
