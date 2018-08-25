//! Model for tuples.

use errors::Result;
use translator;
use {Diagnostics, Flavor, Loc, RpCode, RpReg, Translate, Translator};

decl_body!(pub struct RpTypeBody<F> {
    pub fields: Vec<Loc<F::Field>>,
    pub codes: Vec<Loc<RpCode>>,
});

impl<F> RpTypeBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> impl Iterator<Item = &Loc<F::Field>> {
        self.fields.iter()
    }
}

impl<F: 'static, T> Translate<T> for RpTypeBody<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = RpTypeBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpTypeBody<T::Target>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::Type, self.name)?;
        let decls = self.decls.translate(diag, translator)?;
        let fields = translator::Fields(self.fields).translate(diag, translator)?;

        Ok(RpTypeBody {
            name,
            ident: self.ident,
            comment: self.comment,
            decls,
            decl_idents: self.decl_idents,
            fields,
            codes: self.codes,
        })
    }
}
