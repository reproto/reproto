//! Model for tuples.

use crate::errors::Result;
use crate::translator;
use crate::{Diagnostics, Flavor, RpCode, RpReg, Spanned, Translate, Translator};

decl_body!(
    pub struct RpTupleBody<F> {
        pub fields: Vec<Spanned<F::Field>>,
        pub codes: Vec<Spanned<RpCode>>,
    }
);

impl<F> RpTupleBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> impl Iterator<Item = &Spanned<F::Field>> {
        self.fields.iter()
    }
}

impl<T> Translate<T> for RpTupleBody<T::Source>
where
    T: Translator,
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
