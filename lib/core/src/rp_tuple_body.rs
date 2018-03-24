//! Model for tuples.

use {Flavor, Loc, RpCode, RpField, Translate, Translator};
use errors::Result;
use std::slice;

decl_body!(pub struct RpTupleBody<F> {
    pub fields: Vec<Loc<RpField<F>>>,
    pub codes: Vec<Loc<RpCode>>,
});

/// Iterator over fields.
pub struct Fields<'a, F: 'static>
where
    F: Flavor,
{
    iter: slice::Iter<'a, Loc<RpField<F>>>,
}

impl<'a, F: 'static> Iterator for Fields<'a, F>
where
    F: Flavor,
{
    type Item = &'a Loc<RpField<F>>;

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
    type Source = F;
    type Out = RpTupleBody<T::Target>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<RpTupleBody<T::Target>> {
        translator.visit(&self.name)?;

        Ok(RpTupleBody {
            name: self.name,
            ident: self.ident,
            comment: self.comment,
            decls: self.decls.translate(translator)?,
            fields: self.fields.translate(translator)?,
            codes: self.codes,
        })
    }
}
