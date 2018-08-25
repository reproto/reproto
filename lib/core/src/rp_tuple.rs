//! Model for tuples.

use errors::Result;
use translator;
use {Diagnostics, Flavor, Loc, RpCode, RpReg, Translate, Translator};

decl_body!(pub struct RpTupleBody<F> {
    pub fields: Vec<Loc<F::Field>>,
    pub codes: Vec<Loc<RpCode>>,
});

impl<F: 'static> RpTupleBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> impl Iterator<Item = &Loc<F::Field>> {
        self.fields.iter()
    }
}

impl<F: 'static, T> Translate<T> for RpTupleBody<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = RpTupleBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpTupleBody<T::Target>> {
        translator.visit(diag, &self.name)?;

        let name = translator.translate_local_name(diag, RpReg::Tuple, self.name)?;
        let decls = self.decls.translate(diag, translator)?;
        let fields = translator::Fields(self.fields).translate(diag, translator)?;

        Ok(RpTupleBody {
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
