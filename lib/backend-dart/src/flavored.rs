//! Dart flavor.

#![allow(unused)]

use core::errors::Result;
use core::{
    self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, Loc, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Translate, Translator,
};
use genco::dart;
use genco::{Cons, Dart};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;
use {SCOPE_SEP, TYPE_SEP};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DartFlavor;

impl Flavor for DartFlavor {
    type Type = Dart<'static>;
    type Name = Loc<RpName>;
    type Field = core::RpField<DartFlavor>;
    type Endpoint = DartEndpoint;
    type Package = core::RpPackage;
    type EnumType = Dart<'static>;
}

/// Responsible for translating RpType -> Dart type.
pub struct DartFlavorTranslator {
    packages: Rc<Packages>,
    map: Dart<'static>,
}

impl DartFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        Self {
            packages,
            map: dart::imported("dart:core").name("HashMap"),
        }
    }
}

impl FlavorTranslator for DartFlavorTranslator {
    type Source = CoreFlavor;
    type Target = DartFlavor;

    translator_defaults!(Self, local_name, field);

    fn translate_number(&self, number: RpNumberType) -> Result<Dart<'static>> {
        let out = match number.kind {
            RpNumberKind::U32 => dart::local("u32"),
            RpNumberKind::U64 => dart::local("u64"),
            RpNumberKind::I32 => dart::local("i32"),
            RpNumberKind::I64 => dart::local("i64"),
        };

        Ok(out)
    }

    fn translate_float(&self) -> Result<Dart<'static>> {
        Ok(dart::local("f32"))
    }

    fn translate_double(&self) -> Result<Dart<'static>> {
        Ok(dart::local("f64"))
    }

    fn translate_boolean(&self) -> Result<Dart<'static>> {
        Ok(dart::local("bool"))
    }

    fn translate_string(&self, _: RpStringType) -> Result<Dart<'static>> {
        Ok(dart::local("String"))
    }

    fn translate_datetime(&self) -> Result<Dart<'static>> {
        Ok(dart::local("String"))
    }

    fn translate_array(&self, argument: Dart<'static>) -> Result<Dart<'static>> {
        Ok(dart::local("Vec").with_arguments(vec![argument]))
    }

    fn translate_map(&self, key: Dart<'static>, value: Dart<'static>) -> Result<Dart<'static>> {
        Ok(self.map.clone().with_arguments(vec![key, value]))
    }

    fn translate_any(&self) -> Result<Dart<'static>> {
        Ok(Dart::Dynamic)
    }

    fn translate_bytes(&self) -> Result<Dart<'static>> {
        Ok(dart::local("String"))
    }

    fn translate_name(&self, reg: RpReg, name: Loc<RpName>) -> Result<Dart<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(SCOPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let package_name = name.package.join("/");
            return Ok(dart::imported(package_name).name(ident).alias(prefix.to_string()));
        }

        Ok(dart::local(ident))
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
    ) -> Result<Dart<'static>>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        use core::RpEnumType::*;

        match enum_type {
            String(_) => Ok(dart::local("String")),
            Number(number) => self.translate_number(number),
        }
    }
}

decl_flavor!(DartFlavor, core);
