//! Model for enums

use crate::errors::Result;
use crate::{
    Diagnostics, Flavor, RpCode, RpNumber, RpNumberType, RpReg, RpStringType, RpValue, Span,
    Spanned, Translate, Translator,
};
use serde::Serialize;
use std::fmt;
use std::vec;

decl_body!(
    pub struct RpEnumBody<F> {
        /// The type of the variant.
        pub enum_type: F::EnumType,
        /// Variants in the enum.
        pub variants: RpVariants<F>,
        /// Custom code blocks in the enum.
        pub codes: Vec<Spanned<RpCode>>,
    }
);

impl<T> Translate<T> for RpEnumBody<T::Source>
where
    T: Translator,
{
    type Out = RpEnumBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpEnumBody<T::Target>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::Enum, self.name)?;
        let enum_type = translator.translate_enum_type(diag, self.enum_type)?;
        let decls = self.decls.translate(diag, translator)?;
        let variants = self.variants.translate(diag, translator)?;

        Ok(RpEnumBody {
            name,
            ident: self.ident,
            comment: self.comment,
            decls,
            decl_idents: self.decl_idents,
            enum_type,
            variants,
            codes: self.codes,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpVariantValue<'a> {
    String(&'a str),
    Number(&'a RpNumber),
}

impl<'a> From<&'a RpNumber> for RpVariantValue<'a> {
    fn from(value: &'a RpNumber) -> Self {
        RpVariantValue::Number(value)
    }
}

impl<'a> From<&'a String> for RpVariantValue<'a> {
    fn from(value: &'a String) -> Self {
        RpVariantValue::String(value.as_str())
    }
}

impl<'a> fmt::Display for RpVariantValue<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::RpVariantValue::*;

        match *self {
            String(string) => write!(fmt, "{:?}", string),
            Number(value) => value.fmt(fmt),
        }
    }
}

/// A cheap, type-erasured variant that can be used for value comparisons.
///
/// This is typically created using `RpVariants::iter()`.
#[derive(Debug)]
pub struct RpVariantRef<'a, F>
where
    F: Flavor,
{
    pub span: Span,
    pub name: &'a F::Name,
    pub ident: &'a Spanned<String>,
    pub comment: &'a Vec<String>,
    pub value: RpVariantValue<'a>,
}

impl<'a, F> RpVariantRef<'a, F>
where
    F: Flavor,
{
    /// Get the identifier for this variant.
    pub fn ident(&self) -> &'a str {
        self.ident.as_str()
    }
}

impl<'a, F> Clone for RpVariantRef<'a, F>
where
    F: Flavor,
{
    fn clone(&self) -> Self {
        Self {
            span: self.span,
            name: self.name,
            ident: self.ident,
            comment: self.comment,
            value: self.value,
        }
    }
}

impl<'a, F> Copy for RpVariantRef<'a, F> where F: Flavor {}

impl<'a, F> fmt::Display for RpVariantRef<'a, F>
where
    F: Flavor,
    F::Name: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} as {}", self.name, self.value)
    }
}

/// Variant in an enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(bound = "F::Package: Serialize, F::Name: Serialize, V: Serialize")]
pub struct RpVariant<F, V>
where
    F: Flavor,
{
    pub name: F::Name,
    pub ident: Spanned<String>,
    pub comment: Vec<String>,
    pub value: V,
}

impl<'a, F, V> RpVariant<F, V>
where
    F: Flavor,
{
    /// Get the identifier of the variant.
    pub fn ident(&self) -> &str {
        self.ident.as_str()
    }
}

impl<'a, F, V: 'a> RpVariant<F, V>
where
    F: Flavor,
    RpVariantValue<'a>: From<&'a V>,
{
    /// Convert into a variant value.
    pub fn value(&'a self) -> RpVariantValue<'a> {
        RpVariantValue::from(&self.value)
    }
}

impl<T, V> Translate<T> for RpVariant<T::Source, V>
where
    T: Translator,
{
    type Out = RpVariant<T::Target, V>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpVariant<T::Target, V>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::EnumVariant, self.name)?;

        Ok(RpVariant {
            name,
            ident: self.ident,
            comment: self.comment,
            value: self.value,
        })
    }
}

/// Model for enum types
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpEnumType {
    String(RpStringType),
    Number(RpNumberType),
}

impl RpEnumType {
    pub fn is_assignable_from<F>(&self, value: &RpValue<F>) -> bool
    where
        F: Flavor,
    {
        use self::RpEnumType::*;

        match (self, value) {
            (&String(..), &RpValue::String(_)) => true,
            (&Number(..), &RpValue::Number(_)) => true,
            _ => false,
        }
    }
}

impl fmt::Display for RpEnumType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::RpEnumType::*;

        match *self {
            String(..) => "string".fmt(fmt),
            Number(ref number) => number.fmt(fmt),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(bound = "F: Serialize, F::Package: Serialize, F::Name: Serialize")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpVariants<F>
where
    F: Flavor,
{
    String {
        variants: Vec<Spanned<RpVariant<F, String>>>,
    },
    Number {
        variants: Vec<Spanned<RpVariant<F, RpNumber>>>,
    },
}

impl<F> RpVariants<F>
where
    F: Flavor,
{
    /// Check if the collection of variants are empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String { variants } => variants.is_empty(),
            Self::Number { variants } => variants.is_empty(),
        }
    }
}

pub struct RpVariantsIter<'a, F>
where
    F: Flavor,
{
    iter: vec::IntoIter<RpVariantRef<'a, F>>,
}

impl<'a, F> Iterator for RpVariantsIter<'a, F>
where
    F: Flavor,
{
    type Item = RpVariantRef<'a, F>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<F> RpVariants<F>
where
    F: Flavor,
{
    /// Iterate over all variants in a type-erasured manner.
    ///
    /// Each variant being iterator over has a value which is reflected by the `RpVariantValue` enum.
    pub fn iter(&self) -> RpVariantsIter<F> {
        use self::RpVariants::*;

        macro_rules! variants {
            ($slf:ident, $($ty:ident),*) => {
                match *$slf {
                $(
                $ty { ref variants } => {
                    let mut __o = Vec::new();

                    for v in variants {
                        let (value, span) = Spanned::borrow_pair(v);

                        __o.push(RpVariantRef {
                            span: span,
                            name: &value.name,
                            ident: &value.ident,
                            comment: &value.comment,
                            value: RpVariantValue::from(&value.value),
                        })
                    }

                    __o
                },
                )*
                }
            };
        }

        let variants: Vec<_> = variants!(self, String, Number);

        RpVariantsIter {
            iter: variants.into_iter(),
        }
    }
}

impl<T> Translate<T> for RpVariants<T::Source>
where
    T: Translator,
{
    type Out = RpVariants<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpVariants<T::Target>> {
        use self::RpVariants::*;

        let out = match self {
            String { variants } => String {
                variants: variants.translate(diag, translator)?,
            },
            Number { variants } => Number {
                variants: variants.translate(diag, translator)?,
            },
        };

        Ok(out)
    }
}

impl<'a, F> IntoIterator for &'a RpVariants<F>
where
    F: Flavor,
{
    type Item = RpVariantRef<'a, F>;
    type IntoIter = RpVariantsIter<'a, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
