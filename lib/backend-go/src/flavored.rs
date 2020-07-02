//! Go flavor.

#![allow(unused)]

use crate::TYPE_SEP;
use backend::package_processor;
use core::errors::Result;
use core::{
    CoreFlavor, Diagnostics, Flavor, FlavorTranslator, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Spanned, Translate, Translator,
};
use genco::prelude::*;
use genco::tokens::{FormatInto, Item, ItemStr};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    Bool,
}

impl FormatInto<Go> for Primitive {
    fn format_into(self, t: &mut go::Tokens) {
        match self {
            Self::U32 => quote_in!(*t => uint32),
            Self::U64 => quote_in!(*t => uint64),
            Self::I32 => quote_in!(*t => int32),
            Self::I64 => quote_in!(*t => int64),
            Self::F32 => quote_in!(*t => float32),
            Self::F64 => quote_in!(*t => float64),
            Self::Bool => quote_in!(*t => bool),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive { primitive: Primitive },
    String,
    Interface,
    Array { argument: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
    Import { import: go::Import },
    Local { ident: ItemStr },
}

impl Type {
    pub fn import<T>(import: T) -> Self
    where
        T: Into<go::Import>,
    {
        Self::Import {
            import: import.into(),
        }
    }

    pub fn local<T>(ident: T) -> Self
    where
        T: Into<ItemStr>,
    {
        Self::Local {
            ident: ident.into(),
        }
    }
}

impl<'a> FormatInto<Go> for &'a Type {
    fn format_into(self, t: &mut go::Tokens) {
        match self {
            Type::Primitive { primitive } => {
                t.append(*primitive);
            }
            Type::String => {
                quote_in!(*t => string);
            }
            Type::Array { argument } => {
                quote_in!(*t => []#(&**argument));
            }
            Type::Map { key, value } => {
                quote_in!(*t => map[#(&**key)]#(&**value));
            }
            Type::Interface => {
                quote_in!(*t => interface{});
            }
            Type::Import { import } => {
                quote_in!(*t => #import);
            }
            Type::Local { ident } => {
                quote_in!(*t => #ident);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GoFlavor;

impl Flavor for GoFlavor {
    type Type = Type;
    type Name = GoName;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
    type EnumType = Type;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoName {
    pub name: Rc<String>,
    pub package: RpPackage,
}

impl<'a> FormatInto<Go> for &'a GoName {
    fn format_into(self, tokens: &mut go::Tokens) {
        tokens.append(Item::Literal(self.name.clone().into()))
    }
}

impl package_processor::Name<GoFlavor> for GoName {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

/// Responsible for translating RpType -> Go type.
pub struct GoFlavorTranslator {
    package_translator: Rc<Packages>,
}

impl GoFlavorTranslator {
    pub fn new(package_translator: Rc<Packages>) -> Self {
        Self { package_translator }
    }
}

impl FlavorTranslator for GoFlavorTranslator {
    type Source = CoreFlavor;
    type Target = GoFlavor;

    core::translator_defaults!(Self, field, endpoint);

    fn translate_number(&self, number: RpNumberType) -> Result<Type> {
        Ok(match number.kind {
            RpNumberKind::U32 => Type::Primitive {
                primitive: Primitive::U32,
            },
            RpNumberKind::U64 => Type::Primitive {
                primitive: Primitive::U64,
            },
            RpNumberKind::I32 => Type::Primitive {
                primitive: Primitive::I32,
            },
            RpNumberKind::I64 => Type::Primitive {
                primitive: Primitive::I64,
            },
        })
    }

    fn translate_float(&self) -> Result<Type> {
        Ok(Type::Primitive {
            primitive: Primitive::F32,
        })
    }

    fn translate_double(&self) -> Result<Type> {
        Ok(Type::Primitive {
            primitive: Primitive::F64,
        })
    }

    fn translate_boolean(&self) -> Result<Type> {
        Ok(Type::Primitive {
            primitive: Primitive::Bool,
        })
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
        Ok(Type::Interface)
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        // imported
        if let Some(_) = name.prefix {
            let module = name.package.join(TYPE_SEP);
            let module = format!("../{}", module);

            return Ok(Type::import(go::import(module, ident)));
        }

        // same package
        return Ok(Type::local(ident));
    }

    fn translate_local_name<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: Spanned<core::RpName<CoreFlavor>>,
    ) -> Result<GoName>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let (name, _) = Spanned::take_pair(name);

        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let package = self.translate_package(name.package)?;

        // same package
        return Ok(GoName {
            name: Rc::new(ident),
            package,
        });
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.package_translator.translate_package(source)
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

core::decl_flavor!(pub(crate) GoFlavor, core);
