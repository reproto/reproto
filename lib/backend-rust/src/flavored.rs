//! Rust flavor.

use crate::{SCOPE_SEP, TYPE_SEP};
use genco::prelude::*;
use genco::tokens;
use reproto_core::errors::Result;
use reproto_core::{
    CoreFlavor, Diagnostics, Flavor, FlavorTranslator, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Spanned, Translate, Translator,
};
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone)]
pub(crate) struct RustEndpoint {
    pub(crate) endpoint: RpEndpoint,
    pub(crate) http1: Option<RpEndpointHttp1>,
}

impl Deref for RustEndpoint {
    type Target = RpEndpoint;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Primitive {
    Bool,
    F32,
    F64,
    U32,
    U64,
    I32,
    I64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Type {
    Local(tokens::ItemStr),
    Primitive(Primitive),
    String,
    Vec(Box<Type>),
    Map(Rc<rust::Import>, Box<Type>, Box<Type>),
    Option(Box<Type>),
    Custom(Rc<rust::Import>),
    Generic(Rc<rust::Import>, Box<Type>),
    /// `&'static str`.
    StaticStr,
}

impl From<rust::Import> for Type {
    fn from(value: rust::Import) -> Self {
        Self::Custom(Rc::new(value))
    }
}

impl Type {
    pub(crate) fn local(local: impl Into<tokens::ItemStr>) -> Self {
        Self::Local(local.into())
    }

    pub(crate) fn option(a: impl Into<Type>) -> Self {
        Self::Option(Box::new(a.into()))
    }

    pub(crate) fn generic(base: impl Into<rust::Import>, a: impl Into<Type>) -> Self {
        Self::Generic(Rc::new(base.into()), Box::new(a.into()))
    }
}

impl<'a> tokens::FormatInto<Rust> for &'a Type {
    fn format_into(self, t: &mut rust::Tokens) {
        match self {
            Type::Local(local) => {
                t.append(local);
            }
            Type::Primitive(p) => {
                let s = match p {
                    Primitive::Bool => "bool",
                    Primitive::F32 => "f32",
                    Primitive::F64 => "f64",
                    Primitive::U32 => "u32",
                    Primitive::U64 => "u64",
                    Primitive::I32 => "i32",
                    Primitive::I64 => "i64",
                };

                t.append(tokens::static_literal(s));
            }
            Type::String => {
                quote_in!(*t => String);
            }
            Type::Vec(inner) => {
                quote_in!(*t => Vec<$(&**inner)>);
            }
            Type::Map(map, key, value) => {
                quote_in!(*t => $(&**map)<$(&**key), $(&**value)>);
            }
            Type::StaticStr => {
                quote_in!(*t => &'static str);
            }
            Type::Option(a) => {
                quote_in!(*t => Option<$(&**a)>);
            }
            Type::Custom(import) => {
                (&**import).format_into(t);
            }
            Type::Generic(base, a) => {
                quote_in!(*t => $(&**base)<$(&**a)>);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RustFlavor {}

impl Flavor for RustFlavor {
    type Type = Type;
    type Name = Spanned<RpName>;
    type Field = RpField<RustFlavor>;
    type Endpoint = RustEndpoint;
    type Package = RpPackage;
    type EnumType = Type;
}

/// Responsible for translating RpType -> Rust type.
pub(crate) struct RustFlavorTranslator {
    packages: Rc<Packages>,
    map: Rc<rust::Import>,
    json_value: Rc<rust::Import>,
    datetime: Option<Type>,
}

impl RustFlavorTranslator {
    pub(crate) fn new(packages: Rc<Packages>, datetime: Option<Type>) -> Self {
        Self {
            packages,
            map: Rc::new(rust::import("std::collections", "HashMap")),
            json_value: Rc::new(rust::import("serde_json", "Value").with_module_alias("json")),
            datetime,
        }
    }
}

impl FlavorTranslator for RustFlavorTranslator {
    type Source = CoreFlavor;
    type Target = RustFlavor;

    reproto_core::translator_defaults!(Self, local_name, field);

    fn translate_number(&self, number: RpNumberType) -> Result<Type> {
        let out = match number.kind {
            RpNumberKind::U32 => Type::Primitive(Primitive::U32),
            RpNumberKind::U64 => Type::Primitive(Primitive::U64),
            RpNumberKind::I32 => Type::Primitive(Primitive::I32),
            RpNumberKind::I64 => Type::Primitive(Primitive::I64),
        };

        Ok(out)
    }

    fn translate_float(&self) -> Result<Type> {
        Ok(Type::Primitive(Primitive::F32))
    }

    fn translate_double(&self) -> Result<Type> {
        Ok(Type::Primitive(Primitive::F64))
    }

    fn translate_boolean(&self) -> Result<Type> {
        Ok(Type::Primitive(Primitive::Bool))
    }

    fn translate_string(&self, _: RpStringType) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_datetime(&self) -> Result<Type> {
        if let Some(datetime) = &self.datetime {
            return Ok(datetime.clone());
        }

        Err("Missing implementation for `datetime`, try: -m chrono".into())
    }

    fn translate_array(&self, argument: Type) -> Result<Type> {
        Ok(Type::Vec(Box::new(argument)))
    }

    fn translate_map(&self, key: Type, value: Type) -> Result<Type> {
        Ok(Type::Map(self.map.clone(), Box::new(key), Box::new(value)))
    }

    fn translate_any(&self) -> Result<Type> {
        Ok(Type::Custom(self.json_value.clone()))
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(SCOPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let package_name = format!("crate::{}", name.package.join("::"));

            return Ok(Type::from(
                rust::import(package_name, ident).with_module_alias(prefix.to_string()),
            ));
        }

        Ok(Type::local(ident))
    }

    fn translate_endpoint<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        endpoint: RpEndpoint<CoreFlavor>,
    ) -> Result<RustEndpoint>
    where
        T: Translator<Source = CoreFlavor, Target = RustFlavor>,
    {
        let endpoint = endpoint.translate(diag, translator)?;
        let http1 = RpEndpointHttp1::from_endpoint(&endpoint);

        Ok(RustEndpoint { endpoint, http1 })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.packages.translate_package(source)
    }

    fn translate_enum_type<T>(
        &self,
        _: &T,
        _: &mut Diagnostics,
        enum_type: RpEnumType,
    ) -> Result<Type>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        match enum_type {
            RpEnumType::String(_) => Ok(Type::StaticStr),
            RpEnumType::Number(number) => self.translate_number(number),
        }
    }
}

reproto_core::decl_flavor!(pub(crate) RustFlavor);
