//! Model for tuples.

use crate::errors::Result;
use crate::translator;
use crate::{
    Diagnostics, Flavor, FlavorField, RpCode, RpDecl, RpReg, Spanned, Translate, Translator,
};
use linked_hash_map::LinkedHashMap;
use serde::Serialize;

/// Default key to use for tagged sub type strategy.
pub const DEFAULT_TAG: &str = "type";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpSubTypeStrategy {
    /// An object, with a single tag key indicating which sub-type to use.
    Tagged { tag: String },
    /// An sub-type is distinguished by its set of unique fields.
    /// This requires a sub-type to actually _have_ a unique set of fields, which is validates
    /// during translation.
    Untagged,
}

impl Default for RpSubTypeStrategy {
    fn default() -> Self {
        RpSubTypeStrategy::Tagged {
            tag: DEFAULT_TAG.to_string(),
        }
    }
}

decl_body!(
    pub struct RpInterfaceBody<F> {
        pub fields: Vec<Spanned<F::Field>>,
        pub codes: Vec<Spanned<RpCode>>,
        pub sub_types: Vec<Spanned<RpSubType<F>>>,
        pub sub_type_strategy: RpSubTypeStrategy,
    }
);

impl<F> RpInterfaceBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> impl Iterator<Item = &Spanned<F::Field>> {
        self.fields.iter()
    }
}

impl<T> Translate<T> for RpInterfaceBody<T::Source>
where
    T: Translator,
{
    type Out = RpInterfaceBody<T::Target>;

    /// Translate into different flavor.
    fn translate(
        self,
        diag: &mut Diagnostics,
        translator: &T,
    ) -> Result<RpInterfaceBody<T::Target>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::Interface, self.name)?;
        let decls = self.decls.translate(diag, translator)?;
        let fields = translator::Fields(self.fields).translate(diag, translator)?;
        let sub_types = self.sub_types.translate(diag, translator)?;

        Ok(RpInterfaceBody {
            name,
            ident: self.ident,
            comment: self.comment,
            decls,
            decl_idents: self.decl_idents,
            fields,
            codes: self.codes,
            sub_types,
            sub_type_strategy: self.sub_type_strategy,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(
    bound = "F: Serialize, F::Field: Serialize, F::Endpoint: Serialize, F::Package: Serialize, \
             F::Name: Serialize, F::EnumType: Serialize"
)]
pub struct RpSubType<F>
where
    F: Flavor,
{
    pub name: F::Name,
    pub ident: String,
    pub comment: Vec<String>,
    /// Inner declarations.
    pub decls: Vec<RpDecl<F>>,
    pub decl_idents: LinkedHashMap<String, usize>,
    pub fields: Vec<Spanned<F::Field>>,
    pub codes: Vec<Spanned<RpCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type_name: Option<Spanned<String>>,
}

impl<F> RpSubType<F>
where
    F: Flavor,
{
    pub fn name(&self) -> &str {
        self.sub_type_name
            .as_ref()
            .map(|t| t.as_str())
            .unwrap_or(&self.ident)
    }

    /// Access all fields of the sub type.
    pub fn fields(&self) -> impl Iterator<Item = &Spanned<F::Field>> {
        self.fields.iter()
    }

    /// Access the set of fields which are used to make this sub-type unique.
    pub fn discriminating_fields(&self) -> impl Iterator<Item = &Spanned<F::Field>> {
        let fields = self
            .fields
            .iter()
            .filter(|f| f.is_discriminating())
            .collect::<Vec<_>>();

        fields.into_iter()
    }
}

impl<T> Translate<T> for RpSubType<T::Source>
where
    T: Translator,
{
    type Out = RpSubType<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpSubType<T::Target>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::SubType, self.name)?;
        let decls = self.decls.translate(diag, translator)?;
        let fields = translator::Fields(self.fields).translate(diag, translator)?;

        Ok(RpSubType {
            name,
            ident: self.ident,
            comment: self.comment,
            decls,
            decl_idents: self.decl_idents,
            fields,
            codes: self.codes,
            sub_type_name: self.sub_type_name,
        })
    }
}
