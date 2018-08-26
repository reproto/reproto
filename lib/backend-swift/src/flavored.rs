//! Swift flavor.

#![allow(unused)]

use backend::package_processor;
use core::errors::Result;
use core::{
    self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, Loc, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Translate, Translator,
};
use genco::swift::{self, Swift};
use genco::{Cons, Element, IntoTokens, Tokens};
use module::simple::Simple;
use naming::{self, Naming};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;
use {Options, TYPE_SEP};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwiftType<'el> {
    simple: Simple<'el>,
    ty: Swift<'el>,
}

impl<'el> SwiftType<'el> {
    /// Build a plain swift type.
    pub fn from_type(ty: Swift<'el>) -> SwiftType<'el> {
        SwiftType {
            simple: Simple::Type { ty: ty.clone() },
            ty: ty,
        }
    }

    /// Access the swift type.
    pub fn ty(&self) -> &Swift<'el> {
        &self.ty
    }

    /// Access the simpel type.
    pub fn simple(&self) -> &Simple<'el> {
        &self.simple
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwiftName {
    pub name: Rc<String>,
    pub package: RpPackage,
}

impl fmt::Display for SwiftName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(fmt)
    }
}

impl<'el> From<&'el SwiftName> for Element<'el, Swift<'el>> {
    fn from(value: &'el SwiftName) -> Element<'el, Swift<'el>> {
        Element::Literal(value.name.clone().into())
    }
}

impl package_processor::Name<SwiftFlavor> for SwiftName {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwiftFlavor;

impl Flavor for SwiftFlavor {
    type Type = SwiftType<'static>;
    type Name = SwiftName;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
    type EnumType = SwiftType<'static>;
}

/// Responsible for translating RpType -> Swift type.
pub struct SwiftFlavorTranslator {
    packages: Rc<Packages>,
    data: Swift<'static>,
    date: Swift<'static>,
    any: Swift<'static>,
    to_upper_camel: naming::ToUpperCamel,
}

impl SwiftFlavorTranslator {
    pub fn new(packages: Rc<Packages>, options: &Options) -> Result<Self> {
        let any = {
            let mut any_types = options.any_type.iter().cloned();

            if let Some((first_mod, any_type)) = any_types.next() {
                if let Some((second_mod, _)) = any_types.next() {
                    return Err(format!(
                        "Any type provided by more than one module: {}, {}",
                        first_mod, second_mod
                    ).into());
                }

                any_type.clone()
            } else {
                swift::local("Any")
            }
        };

        Ok(Self {
            packages,
            data: swift::imported("Foundation", "Data"),
            date: swift::imported("Foundation", "Date"),
            any,
            to_upper_camel: naming::to_upper_camel(),
        })
    }
}

impl FlavorTranslator for SwiftFlavorTranslator {
    type Source = CoreFlavor;
    type Target = SwiftFlavor;

    translator_defaults!(Self, field, endpoint);

    fn translate_number(&self, number: RpNumberType) -> Result<SwiftType<'static>> {
        let out = match number.kind {
            RpNumberKind::U32 => swift::local("UInt32"),
            RpNumberKind::U64 => swift::local("UInt64"),
            RpNumberKind::I32 => swift::local("Int32"),
            RpNumberKind::I64 => swift::local("Int64"),
            ty => return Err(format!("unsupported number type: {}", ty).into()),
        };

        Ok(SwiftType::from_type(out))
    }

    fn translate_float(&self) -> Result<SwiftType<'static>> {
        Ok(SwiftType::from_type(swift::local("Float")))
    }

    fn translate_double(&self) -> Result<SwiftType<'static>> {
        Ok(SwiftType::from_type(swift::local("Double")))
    }

    fn translate_boolean(&self) -> Result<SwiftType<'static>> {
        Ok(SwiftType::from_type(swift::local("Bool")))
    }

    fn translate_string(&self, _: RpStringType) -> Result<SwiftType<'static>> {
        Ok(SwiftType::from_type(swift::local("String")))
    }

    fn translate_datetime(&self) -> Result<SwiftType<'static>> {
        Ok(SwiftType {
            simple: Simple::DateTime,
            ty: self.date.clone(),
        })
    }

    fn translate_array(&self, argument: SwiftType<'static>) -> Result<SwiftType<'static>> {
        Ok(SwiftType {
            simple: Simple::Array {
                argument: Box::new(argument.simple.clone()),
            },
            ty: swift::array(argument.ty),
        })
    }

    fn translate_map(
        &self,
        key: SwiftType<'static>,
        value: SwiftType<'static>,
    ) -> Result<SwiftType<'static>> {
        Ok(SwiftType {
            simple: Simple::Map {
                key: Box::new(key.simple.clone()),
                value: Box::new(value.simple.clone()),
            },
            ty: swift::map(key.ty, value.ty),
        })
    }

    fn translate_any(&self) -> Result<SwiftType<'static>> {
        Ok(SwiftType {
            simple: Simple::Any {
                ty: self.any.clone(),
            },
            ty: self.any.clone(),
        })
    }

    fn translate_bytes(&self) -> Result<SwiftType<'static>> {
        Ok(SwiftType {
            simple: Simple::Bytes,
            ty: self.data.clone(),
        })
    }

    fn translate_name(&self, reg: RpReg, name: Loc<RpName>) -> Result<SwiftType<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let package_name = name.package.join("_");
        let ty = swift::local(format!("{}_{}", package_name, ident));

        Ok(SwiftType {
            simple: Simple::Name { name: ty.clone() },
            ty: ty,
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
    ) -> Result<SwiftName>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let name = name.translate(diag, translator)?;
        let (name, _) = Loc::take_pair(name);

        let package_name = name.package.join("_");
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let ident = format!("{}_{}", package_name, ident);

        Ok(SwiftName {
            name: Rc::new(ident),
            package: name.package,
        })
    }

    fn translate_enum_type<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: core::RpEnumType,
    ) -> Result<SwiftType<'static>>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        use core::RpEnumType::*;

        match enum_type {
            String(string) => self.translate_string(string),
            Number(number) => self.translate_number(number),
        }
    }
}

decl_flavor!(SwiftFlavor, core);
