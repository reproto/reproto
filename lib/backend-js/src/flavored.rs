//! JavaScript flavor.

use crate::TYPE_SEP;
use backend::package_processor;
use core::errors::Result;
use core::{
    self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, PackageTranslator, RpNumberType,
    RpStringType, Spanned, Translate, Translator,
};
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr};
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
    Float,
    String,
    Bool,
    Object,
    Array { argument: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
    Import { import: js::Import },
    Local { ident: ItemStr },
}

impl Type {
    pub fn decode(&self, t: &mut js::Tokens, var: js::Tokens) {
        self.decode_depth(t, &var, 0);
    }

    /// Build decode method which also performs type checking.
    pub fn decode_depth<T>(&self, t: &mut js::Tokens, var: T, d: usize)
    where
        T: FormatInto<JavaScript> + Copy,
    {
        match self {
            Self::Object => (),
            Self::Integer => {
                quote_in! { *t =>
                    if (!Number.isInteger(#var)) {
                        throw Error("expected integer");
                    }
                }
            }
            Self::Float => {
                quote_in! { *t =>
                    if (!Number.isFinite(#var)) {
                        throw Error("expected float");
                    }
                }
            }
            Self::Bool => {
                quote_in! { *t =>
                    if (typeof #var !== "boolean") {
                        throw Error("expected boolean");
                    }
                }
            }
            Self::String => {
                quote_in! { *t =>
                    if (typeof #var !== "string") {
                        throw Error("expected string");
                    }
                }
            }
            Self::Array { argument } => {
                let o = &format!("o{}", d);
                let i = &format!("i{}", d);
                let l = &format!("l{}", d);
                let v = &format!("v{}", d);

                quote_in! { *t =>
                    if (!Array.isArray(#var)) {
                        throw Error("expected array");
                    }

                    let #o = [];

                    for (let #i = 0, #l = #var.length; #i < #l; #i++) {
                        let #v = #var[#i];

                        #(ref t => argument.decode_depth(t, v, d + 1))

                        #o.push(#v);
                    }

                    #var = #o;
                }
            }
            Self::Map { key, value } => {
                let o = &format!("o{}", d);
                let k = &format!("k{}", d);
                let v = &format!("v{}", d);

                quote_in! { *t =>
                    if (typeof #var !== "object") {
                        throw Error("expected object");
                    }

                    let #o = {};

                    for (let [#k, #v] of Object.entries(#var)) {
                        #(ref t => key.decode_depth(t, k, d + 1))
                        #(ref t => value.decode_depth(t, v, d + 1))

                        #o[#k] = #v;
                    }

                    #var = #o;
                }
            }
            Self::Import { import } => quote_in! { *t =>
                #var = #import.decode(#var);
            },
            Self::Local { ident } => quote_in! { *t =>
                #var = #ident.decode(#var);
            },
        }
    }

    /// Build encode method.
    pub fn encode(&self, var: js::Tokens) -> js::Tokens {
        match self {
            Self::String => quote!(#var),
            Self::Float => quote!(#var),
            Self::Integer => quote!(#var),
            Self::Bool => quote!(#var),
            Self::Object => quote!(#var),
            Self::Array { argument } => {
                let v = argument.encode(quote!(v));
                quote!(#var.map(function(v) { return #v; }))
            }
            Self::Map { key, value } => {
                let k = &key.encode(quote!(k));
                let v = &value.encode(quote!(data[#k]));

                quote! {
                    (function(data) {
                        let o = {};

                        for (let k in data) {
                            o[#k] = #v;
                        }

                        return o;
                    })(#var)
                }
            }
            Self::Import { .. } => quote!(#var.encode()),
            Self::Local { .. } => quote!(#var.encode()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    ident: ItemStr,
    package: RpPackage,
}

impl<'a> FormatInto<JavaScript> for &'a Name {
    fn format_into(self, tokens: &mut Tokens<JavaScript>) {
        tokens.append(&self.ident);
    }
}

impl package_processor::Name<JavaScriptFlavor> for Name {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaScriptFlavor;

impl Flavor for JavaScriptFlavor {
    type Type = Type;
    type Name = Name;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
    type EnumType = Type;
}

/// Responsible for translating RpType -> JavaScript type.
pub struct JavaScriptFlavorTranslator {
    packages: Rc<Packages>,
}

impl JavaScriptFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        Self { packages }
    }
}

impl FlavorTranslator for JavaScriptFlavorTranslator {
    type Source = CoreFlavor;
    type Target = JavaScriptFlavor;

    core::translator_defaults!(Self, field, endpoint);

    fn translate_number(&self, _: RpNumberType) -> Result<Type> {
        Ok(Type::Integer)
    }

    fn translate_float(&self) -> Result<Type> {
        Ok(Type::Float)
    }

    fn translate_double(&self) -> Result<Type> {
        Ok(Type::Float)
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
        Ok(Type::Array {
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
        Ok(Type::Object)
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        if let Some(used) = &name.prefix {
            let module = js::Module::Path(format!("{}.js", name.package.join("/")).into());

            return Ok(Type::Import {
                import: js::import(module, ident).with_alias(used.to_string()),
            });
        }

        Ok(Type::Local {
            ident: ident.into(),
        })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        Ok(self.packages.translate_package(source)?)
    }

    fn translate_local_name<T>(
        &self,
        _: &T,
        _: &mut Diagnostics,
        reg: RpReg,
        name: Spanned<core::RpName<CoreFlavor>>,
    ) -> Result<Name>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let (name, _) = Spanned::take_pair(name);

        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));
        let package = self.translate_package(name.package)?;

        Ok(Name {
            ident: ident.into(),
            package,
        })
    }

    fn translate_enum_type<T>(
        &self,
        _: &T,
        _: &mut Diagnostics,
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

core::decl_flavor!(pub(crate) JavaScriptFlavor, core);
