//! Model for sub-types

use {Flavor, Loc, RpCode, RpDecl, RpField, RpName};

#[derive(Debug, Clone, Serialize)]
pub struct RpSubType<F: 'static>
where
    F: Flavor,
{
    pub name: RpName,
    pub ident: String,
    pub comment: Vec<String>,
    /// Inner declarations.
    pub decls: Vec<RpDecl<F>>,
    pub fields: Vec<Loc<RpField<F>>>,
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
