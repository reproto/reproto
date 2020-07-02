//! Dart flavor.

#![allow(unused)]

use crate::{EXT, TYPE_SEP};
use core::errors::Result;
use core::{
    self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Spanned, Translate, Translator,
};
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Import { import: dart::Import },
    Local { ident: ItemStr },
    Dynamic,
    Int,
    Double,
    Bool,
    String,
    List { argument: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
}

impl Type {
    pub fn map<K, V>(key: K, value: V) -> Self
    where
        K: Into<Type>,
        V: Into<Type>,
    {
        Self::Map {
            key: Box::new(key.into()),
            value: Box::new(value.into()),
        }
    }

    pub fn list<A>(argument: A) -> Self
    where
        A: Into<Type>,
    {
        Self::List {
            argument: Box::new(argument.into()),
        }
    }
}

impl<'a> FormatInto<Dart> for &'a Type {
    fn format_into(self, t: &mut dart::Tokens) {
        match self {
            Type::Import { import } => quote_in!(*t => #import),
            Type::Local { ident } => quote_in!(*t => #ident),
            Type::Dynamic => quote_in!(*t => dynamic),
            Type::Int => quote_in!(*t => int),
            Type::Double => quote_in!(*t => double),
            Type::Bool => quote_in!(*t => bool),
            Type::String => quote_in!(*t => String),
            Type::List { argument } => {
                quote_in!(*t => List<#(&**argument)>);
            }
            Type::Map { key, value } => {
                quote_in!(*t => Map<#(&**key), #(&**value)>);
            }
        }
    }
}

impl Type {
    /// Create an encode function appropriate for this type.
    pub fn encode(&self, i: dart::Tokens) -> dart::Tokens {
        match self {
            Type::Import { .. } => quote!(#i.encode()),
            Type::Local { .. } => quote!(#i.encode()),
            Type::Dynamic => i,
            Type::Int => i,
            Type::Double => i,
            Type::Bool => i,
            Type::String => i,
            Type::Map { key, value } => {
                let d = value.encode(quote!(e.value));
                quote!(Map.fromEntries(#i.entries.map((e) => MapEntry(e.key, #d))))
            }
            Type::List { argument } => {
                let d = argument.encode(quote!(e));
                quote!(List.from(#i.map((e) => #d)))
            }
        }
    }

    /// Create a decode function appropriate for this type.
    /// The first tuple element returned is the decoding procedure of the argument.
    /// The second optional tuple element is extra validation that needs to be evaluated.
    pub fn decode(&self, i: dart::Tokens) -> (dart::Tokens, dart::Tokens) {
        let ty = match self {
            Type::Import { import } => {
                return (quote!(#import.decode(#i)), quote!());
            }
            Type::Local { ident } => {
                return (quote!(#ident.decode(#i)), quote!());
            }
            Type::Map { key, value } => {
                let i = &i;
                let (d, e) = value.decode(quote!(e.value));

                let t = if e.is_empty() {
                    quote!(Map.fromEntries((#i as Map<#(&**key), dynamic>).entries.map((e) => MapEntry(e.key, #d))))
                } else {
                    quote! {
                        Map.fromEntries((#i as Map<#(&**key), dynamic>).entries.map((e) {
                            return MapEntry(e.key, #d);
                        }))
                    }
                };

                let e = quote! {
                    if (!(#i is Map<#(&**key), dynamic>)) {
                        throw #_(expected map, but was: $(#i));
                    }
                };

                return (t, e);
            }
            Type::List { argument } => {
                let i = &i;
                let (d, e) = argument.decode(quote!(e));

                let entries = quote!((#i as List<dynamic>));

                let t = if e.is_empty() {
                    quote!(List.of((#i as List<dynamic>).map((e) => #d)))
                } else {
                    quote! {
                        List.of((#i as List<dynamic>).map((e) {
                            return #d;
                        }))
                    }
                };

                // check that value is a list.
                let e = quote! {
                    if (!(#i is List<dynamic>)) {
                        throw #_(expected list, but was: $(#i));
                    }
                };

                return (t, e);
            }
            ty => ty,
        };

        let mut e = quote! {
            if (!(#(&i) is #ty)) {
                throw #_(expected $(#ty), but was: $(#(&i)));
            }
        };

        (i, e)
    }
}

#[derive(Debug, Clone)]
pub struct DartEndpoint {
    pub endpoint: RpEndpoint,
    pub http1: Option<RpEndpointHttp1>,
}

impl Deref for DartEndpoint {
    type Target = RpEndpoint;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DartFlavor {}

impl Flavor for DartFlavor {
    type Type = Type;
    type Name = Spanned<RpName>;
    type Field = core::RpField<DartFlavor>;
    type Endpoint = DartEndpoint;
    type Package = core::RpPackage;
    type EnumType = Type;
}

/// Responsible for translating RpType -> Dart type.
pub struct DartFlavorTranslator {
    packages: Rc<Packages>,
}

impl DartFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        Self { packages }
    }
}

impl FlavorTranslator for DartFlavorTranslator {
    type Source = CoreFlavor;
    type Target = DartFlavor;

    core::translator_defaults!(Self, local_name, field);

    fn translate_number(&self, number: RpNumberType) -> Result<Type> {
        Ok(Type::Int)
    }

    fn translate_float(&self) -> Result<Type> {
        Ok(Type::Double)
    }

    fn translate_double(&self) -> Result<Type> {
        Ok(Type::Double)
    }

    fn translate_boolean(&self) -> Result<Type> {
        Ok(Type::Bool)
    }

    fn translate_string(&self, _: RpStringType) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_datetime(&self) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_array(&self, argument: Type) -> Result<Type> {
        Ok(Type::List {
            argument: Box::new(argument),
        })
    }

    fn translate_map(&self, key: Type, value: Type) -> Result<Type> {
        Ok(Type::Map {
            key: Box::new(key),
            value: Box::new(value),
        })
    }

    fn translate_any(&self) -> Result<Type> {
        Ok(Type::Dynamic)
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_name(&self, from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let path = relative_path(
                from.parts().map(|s| s.as_str()),
                name.package.parts().map(|s| s.as_str()),
            );
            let path = format!("{}.{}", path.join("/"), EXT);

            let import = dart::import(path, ident).with_alias(prefix.to_string());
            return Ok(Type::Import { import });
        }

        return Ok(Type::Local {
            ident: ident.into(),
        });
    }

    fn translate_endpoint<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        endpoint: core::RpEndpoint<CoreFlavor>,
    ) -> Result<DartEndpoint>
    where
        T: Translator<Source = CoreFlavor, Target = DartFlavor>,
    {
        let endpoint = endpoint.translate(diag, translator)?;
        let http1 = RpEndpointHttp1::from_endpoint(&endpoint);

        Ok(DartEndpoint { endpoint, http1 })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.packages.translate_package(source)
    }

    fn translate_enum_type<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: core::RpEnumType,
    ) -> Result<Type>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        match enum_type {
            core::RpEnumType::String(string) => self.translate_string(string),
            core::RpEnumType::Number(number) => self.translate_number(number),
        }
    }
}

core::decl_flavor!(pub(crate) DartFlavor);

/// Takes two iterators as a path, strips common prefix, and makes the two paths relative to each
/// other.
fn relative_path<'a>(
    mut a: impl Clone + Iterator<Item = &'a str>,
    mut b: impl Clone + Iterator<Item = &'a str>,
) -> Vec<&'a str> {
    // strip common prefix.
    while let Some((sa, sb)) = take_pair(&mut a.clone(), &mut b.clone()) {
        if sa != sb {
            break;
        }

        a.next();
        b.next();
    }

    let out: Vec<_> = a.skip(1).map(|_| "..").chain(b).collect();

    if out.is_empty() {
        return vec!["."];
    }

    out
}

/// Take a pair from two iterators.
fn take_pair<'a>(
    a: &mut impl Iterator<Item = &'a str>,
    b: &mut impl Iterator<Item = &'a str>,
) -> Option<(&'a str, &'a str)> {
    let a = match a.next() {
        Some(a) => a,
        None => return None,
    };

    let b = match b.next() {
        Some(b) => b,
        None => return None,
    };

    Some((a, b))
}

#[cfg(test)]
mod tests {
    use super::relative_path;

    #[test]
    fn test_relative_path() {
        assert_eq!(
            vec!["bar"],
            relative_path(vec!["foo"].into_iter(), vec!["bar"].into_iter())
        );

        // NB: this might not be legal, since the from package is the empty package.
        assert_eq!(
            vec!["bar"],
            relative_path(vec![].into_iter(), vec!["bar"].into_iter())
        );

        assert_eq!(
            vec!["biz"],
            relative_path(
                vec!["foo", "baz"].into_iter(),
                vec!["foo", "biz"].into_iter()
            )
        );

        assert_eq!(
            vec!["..", "bar"],
            relative_path(vec!["foo", "baz"].into_iter(), vec!["bar"].into_iter())
        );

        assert_eq!(
            vec!["."],
            relative_path(vec!["foo", "baz"].into_iter(), vec!["foo"].into_iter())
        );

        assert_eq!(
            vec!["."],
            relative_path(vec![].into_iter(), vec![].into_iter())
        );
    }
}
