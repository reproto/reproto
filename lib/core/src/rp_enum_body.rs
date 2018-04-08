//! Model for enums

use errors::Result;
use serde::Serialize;
use std::fmt;
use {Flavor, Loc, RpCode, RpReg, RpValue, Translate, Translator};

decl_body!(pub struct RpEnumBody<F> {
    /// The type of the variant.
    pub enum_type: RpEnumType,
    /// Variants in the enum.
    pub variants: Vec<Loc<RpVariant<F>>>,
    /// Custom code blocks in the enum.
    pub codes: Vec<Loc<RpCode>>,
});

impl<F: 'static, T> Translate<T> for RpEnumBody<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpEnumBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<RpEnumBody<T::Target>> {
        translator.visit(&self.name)?;

        let name = translator.translate_local_name(RpReg::Enum, self.name)?;

        Ok(RpEnumBody {
            name: name,
            ident: self.ident,
            comment: self.comment,
            decls: self.decls.translate(translator)?,
            enum_type: self.enum_type,
            variants: self.variants.translate(translator)?,
            codes: self.codes,
        })
    }
}

/// Variant in an enum
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

/// Data Models for the final model stage stage.
#[derive(Debug, Clone, Serialize)]
pub enum RpEnumOrdinal {
    /// Value is specified expliticly.
    String(String),
    /// Value is automatically derived from the name of the variant.
    Generated,
}

/// Model for enum types
#[derive(Debug, Clone, Serialize)]
pub enum RpEnumType {
    String,
}

impl RpEnumType {
    pub fn is_assignable_from(&self, value: &RpValue) -> bool {
        use self::RpEnumType::*;

        match (self, value) {
            (&String, &RpValue::String(_)) => true,
            _ => false,
        }
    }
}

impl fmt::Display for RpEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RpEnumType::*;

        match *self {
            String => write!(f, "string"),
        }
    }
}
