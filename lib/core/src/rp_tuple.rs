//! Model for tuples.

use errors::Result;
use std::slice;
use translator;
use {Diagnostics, Flavor, Loc, RpCode, RpReg, Translate, Translator};

decl_body!(pub struct RpTupleBody<F> {
    pub fields: Vec<Loc<F::Field>>,
    pub codes: Vec<Loc<RpCode>>,
});

/// Iterator over fields.
pub struct Fields<'a, F: 'static>
where
    F: Flavor,
{
    iter: slice::Iter<'a, Loc<F::Field>>,
}

impl<'a, F: 'static> Iterator for Fields<'a, F>
where
    F: Flavor,
{
    type Item = &'a Loc<F::Field>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<F: 'static> RpTupleBody<F>
where
    F: Flavor,
{
    pub fn fields(&self) -> Fields<F> {
        Fields {
            iter: self.fields.iter(),
        }
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

        Ok(RpTupleBody {
            name: name,
            ident: self.ident,
            comment: self.comment,
            decls: self.decls.translate(diag, translator)?,
            decl_idents: self.decl_idents,
            fields: translator::Fields(self.fields).translate(diag, translator)?,
            codes: self.codes,
        })
    }
}
