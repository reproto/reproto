//! Python flavor.

#![allow(unused)]

use crate::utils::VersionHelper;
use crate::{Options, TYPE_SEP};
use backend::package_processor;
use core::errors::Result;
use core::{
    CoreFlavor, Diagnostics, Flavor, FlavorTranslator, Loc, PackageTranslator, RpNumberType,
    RpStringType, Translate, Translator,
};
use genco::prelude::*;
use genco::tokens::{FormatInto, Item, ItemStr};
use naming::Naming;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone)]
pub enum Type {
    Native,
    Integer,
    Float,
    Boolean,
    String { helper: Rc<dyn VersionHelper> },
    Array { argument: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
    Name { import: python::Import },
    Local { ident: ItemStr },
}

impl Type {
    pub fn name<T>(import: T) -> Self
    where
        T: Into<python::Import>,
    {
        Self::Name {
            import: import.into(),
        }
    }

    /// Check if the current type is completely native.
    fn is_native(&self) -> bool {
        match self {
            Self::Native => true,
            Self::Integer | Self::Float | Self::Boolean | Self::String { .. } => true,
            Self::Array { argument } => argument.is_native(),
            Self::Map { key, value } => key.is_native() && value.is_native(),
            _ => false,
        }
    }
}

impl Type {
    /// Build decode method.
    ///
    /// `var` is the name of the variable we finally want to assign.
    /// `l` helps us generate unique local variables, and should be incremented one level for every
    /// nested call of `decode`.
    pub fn decode<V>(&self, var: V, l: usize) -> Option<Tokens<Python>>
    where
        V: Into<ItemStr>,
    {
        use self::Type::*;

        let var = &var.into();

        match self {
            Self::Integer => Some(quote! {
                if not isinstance(#var, int):
                    raise Exception("not an integer")
            }),
            Self::Float => Some(quote! {
                if not isinstance(#var, float):
                    raise Exception("not a float")
            }),
            Self::Boolean => Some(quote! {
                if not isinstance(#var, bool):
                    raise Exception("not a boolean")
            }),
            Self::String { helper } => Some(quote! {
                if not #(helper.is_string(var)):
                    raise Exception("not a string")
            }),
            Self::Native => None,
            Self::Array { argument } => {
                let v = &Rc::new(format!("_v{}", l));
                let a = &Rc::new(format!("_a{}", l));

                Some(quote! {
                    if not isinstance(#var, list):
                        raise Exception("not an array")

                    #a = []

                    for #v in #var:
                        #(if let Some(d) = argument.decode(v.clone(), l + 1) {
                            #d
                            #<line>
                        })
                        #a.append(#v)

                    #var = #a
                })
            }
            Self::Map { key, value } => {
                let o = &Rc::new(format!("_o{}", l));
                let k = &Rc::new(format!("_k{}", l));
                let v = &Rc::new(format!("_v{}", l));

                Some(quote! {
                    if not isinstance(#var, dict):
                        raise Exception("not an object")

                    #o = {}

                    for #k, #v in #var.items():
                        #(if let Some(d) = key.decode(k.clone(), l + 1) =>
                            #d
                        )
                        #(if let Some(d) = value.decode(v.clone(), l + 1) =>
                            #d
                        )
                        #o[#k] = #v

                    #var = #o
                })
            }
            Self::Local { ident } => Some(quote!(#(var.clone()) = #ident.decode(#(var.clone())))),
            Self::Name { import } => Some(quote!(#(var.clone()) = #import.decode(#(var.clone())))),
        }
    }

    /// Build encode method.
    pub fn encode(&self, var: Tokens<Python>) -> Tokens<Python> {
        match self {
            Self::Integer | Self::Float | Self::Boolean | Self::Native | Self::String { .. } => {
                quote!(#var)
            }
            v if v.is_native() => quote!(#var),
            Self::Array { argument } => {
                let v = argument.encode(quote!(v));
                quote!([#v for v in #var])
            }
            Self::Map { key, value } => {
                let k = key.encode(quote!(k));
                let v = value.encode(quote!(v));
                quote!(dict((#k, #v) for (k, v) in #var.items()))
            }
            Self::Local { .. } | Self::Name { .. } => quote!(#var.encode()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub ident: ItemStr,
    pub package: RpPackage,
}

impl<'a> FormatInto<Python> for &'a Name {
    fn format_into(self, out: &mut Tokens<Python>) {
        out.append(&self.ident)
    }
}

impl package_processor::Name<PythonFlavor> for Name {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PythonFlavor;

impl Flavor for PythonFlavor {
    type Type = Type;
    type Name = Name;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
    type EnumType = RpEnumType;
}

/// Responsible for translating RpType -> Python type.
pub struct PythonFlavorTranslator {
    packages: Rc<Packages>,
    helper: Rc<dyn VersionHelper>,
}

impl PythonFlavorTranslator {
    pub fn new(packages: Rc<Packages>, helper: Rc<dyn VersionHelper>) -> Self {
        Self { packages, helper }
    }
}

impl FlavorTranslator for PythonFlavorTranslator {
    type Source = CoreFlavor;
    type Target = PythonFlavor;

    core::translator_defaults!(Self, endpoint, enum_type, field);

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
        Ok(Type::Boolean)
    }

    fn translate_string(&self, _: RpStringType) -> Result<Type> {
        Ok(Type::String {
            helper: self.helper.clone(),
        })
    }

    fn translate_datetime(&self) -> Result<Type> {
        Ok(Type::String {
            helper: self.helper.clone(),
        })
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
        Ok(Type::Native)
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::String {
            helper: self.helper.clone(),
        })
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Loc<RpName>) -> Result<Type> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));

        if let Some(used) = &name.prefix {
            let package = name.package.join(".");

            return Ok(Type::Name {
                import: python::import(package, ident)
                    .with_alias(used.to_string())
                    .into(),
            });
        }

        Ok(Type::Local {
            ident: ident.into(),
        })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.packages.translate_package(source)
    }

    fn translate_local_name<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: Loc<core::RpName<CoreFlavor>>,
    ) -> Result<Name>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let (name, span) = Loc::take_pair(name);

        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));
        let package = self.translate_package(name.package)?;

        Ok(Name {
            ident: ident.into(),
            package,
        })
    }
}

core::decl_flavor!(pub(crate) PythonFlavor, core);
