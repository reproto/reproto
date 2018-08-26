//! Rust flavor.

#![allow(unused)]

use core::errors::Result;
use core::{
    self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, Loc, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Translate, Translator,
};
use genco::rust;
use genco::{Cons, Rust};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;
use {SCOPE_SEP, TYPE_SEP};

#[derive(Debug, Clone)]
pub struct RustEndpoint {
    pub endpoint: RpEndpoint,
    pub http1: Option<RpEndpointHttp1>,
}

impl Deref for RustEndpoint {
    type Target = RpEndpoint;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustFlavor;

impl Flavor for RustFlavor {
    type Type = Rust<'static>;
    type Name = Loc<RpName>;
    type Field = core::RpField<RustFlavor>;
    type Endpoint = RustEndpoint;
    type Package = core::RpPackage;
    type EnumType = Rust<'static>;
}

/// Responsible for translating RpType -> Rust type.
pub struct RustFlavorTranslator {
    packages: Rc<Packages>,
    map: Rust<'static>,
    json_value: Rust<'static>,
    datetime: Option<Rust<'static>>,
}

impl RustFlavorTranslator {
    pub fn new(packages: Rc<Packages>, datetime: Option<Rust<'static>>) -> Self {
        Self {
            packages,
            map: rust::imported("std::collections", "HashMap"),
            json_value: rust::imported("serde_json", "Value").alias("json"),
            datetime: datetime,
        }
    }
}

impl FlavorTranslator for RustFlavorTranslator {
    type Source = CoreFlavor;
    type Target = RustFlavor;

    translator_defaults!(Self, local_name, field);

    fn translate_number(&self, number: RpNumberType) -> Result<Rust<'static>> {
        let out = match number.kind {
            RpNumberKind::U32 => rust::local("u32"),
            RpNumberKind::U64 => rust::local("u64"),
            RpNumberKind::I32 => rust::local("i32"),
            RpNumberKind::I64 => rust::local("i64"),
        };

        Ok(out)
    }

    fn translate_float(&self) -> Result<Rust<'static>> {
        Ok(rust::local("f32"))
    }

    fn translate_double(&self) -> Result<Rust<'static>> {
        Ok(rust::local("f64"))
    }

    fn translate_boolean(&self) -> Result<Rust<'static>> {
        Ok(rust::local("bool"))
    }

    fn translate_string(&self, _: RpStringType) -> Result<Rust<'static>> {
        Ok(rust::local("String"))
    }

    fn translate_datetime(&self) -> Result<Rust<'static>> {
        if let Some(ref datetime) = self.datetime {
            return Ok(datetime.clone());
        }

        Err("Missing implementation for `datetime`, try: -m chrono".into())
    }

    fn translate_array(&self, argument: Rust<'static>) -> Result<Rust<'static>> {
        Ok(rust::local("Vec").with_arguments(vec![argument]))
    }

    fn translate_map(&self, key: Rust<'static>, value: Rust<'static>) -> Result<Rust<'static>> {
        Ok(self.map.clone().with_arguments(vec![key, value]))
    }

    fn translate_any(&self) -> Result<Rust<'static>> {
        Ok(self.json_value.clone())
    }

    fn translate_bytes(&self) -> Result<Rust<'static>> {
        Ok(rust::local("String"))
    }

    fn translate_name(&self, reg: RpReg, name: Loc<RpName>) -> Result<Rust<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(SCOPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let package_name = name.package.join("::");
            return Ok(rust::imported(package_name, ident).alias(prefix.to_string()));
        }

        Ok(rust::local(ident))
    }

    fn translate_endpoint<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        endpoint: core::RpEndpoint<CoreFlavor>,
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
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: core::RpEnumType,
    ) -> Result<Rust<'static>>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        use core::RpEnumType::*;

        match enum_type {
            String(_) => Ok(rust::local("str").reference(rust::StaticRef)),
            Number(number) => self.translate_number(number),
        }
    }
}

decl_flavor!(RustFlavor, core);
