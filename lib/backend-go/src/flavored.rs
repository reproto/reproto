//! Go flavor.

#![allow(unused)]

use backend::PackageUtils;
use core::errors::Result;
use core::{self, CoreFlavor, Flavor, FlavorTranslator, Loc, PackageTranslator, Translate,
           Translator};
use genco::Cons;
use genco::go::{array, imported, interface, local, map, Go};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use {GoPackageUtils, TYPE_SEP};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GoFlavor;

impl Flavor for GoFlavor {
    type Type = Go<'static>;
    type Name = RpName;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
}

/// Responsible for translating RpType -> Go type.
pub struct GoFlavorTranslator {
    package_translator: HashMap<RpVersionedPackage, RpPackage>,
    package_utils: Rc<GoPackageUtils>,
}

impl GoFlavorTranslator {
    pub fn new(
        package_translator: HashMap<RpVersionedPackage, RpPackage>,
        package_utils: Rc<GoPackageUtils>,
    ) -> Self {
        Self {
            package_translator,
            package_utils,
        }
    }
}

impl FlavorTranslator for GoFlavorTranslator {
    type Source = CoreFlavor;
    type Target = GoFlavor;

    translator_defaults!(Self, local_name, field, endpoint);

    fn translate_i32(&self) -> Result<Go<'static>> {
        Ok(local("int32"))
    }

    fn translate_i64(&self) -> Result<Go<'static>> {
        Ok(local("int64"))
    }

    fn translate_u32(&self) -> Result<Go<'static>> {
        Ok(local("uint32"))
    }

    fn translate_u64(&self) -> Result<Go<'static>> {
        Ok(local("uint64"))
    }

    fn translate_float(&self) -> Result<Go<'static>> {
        Ok(local("float32"))
    }

    fn translate_double(&self) -> Result<Go<'static>> {
        Ok(local("float64"))
    }

    fn translate_boolean(&self) -> Result<Go<'static>> {
        Ok(local("bool"))
    }

    fn translate_string(&self) -> Result<Go<'static>> {
        Ok(local("string"))
    }

    fn translate_datetime(&self) -> Result<Go<'static>> {
        Ok(local("string"))
    }

    fn translate_array(&self, argument: Go<'static>) -> Result<Go<'static>> {
        Ok(array(argument))
    }

    fn translate_map(&self, key: Go<'static>, value: Go<'static>) -> Result<Go<'static>> {
        Ok(map(key, value))
    }

    fn translate_any(&self) -> Result<Go<'static>> {
        Ok(interface())
    }

    fn translate_bytes(&self) -> Result<Go<'static>> {
        Ok(local("string"))
    }

    fn translate_name(&self, reg: RpReg, name: RpName) -> Result<Go<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        // imported
        if let Some(_) = name.prefix {
            let module = name.package.join(TYPE_SEP);
            let module = format!("../{}", module);
            return Ok(imported(module, ident));
        }

        // same package
        return Ok(local(ident));
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        let package = self.package_translator.translate_package(source)?;
        Ok(package)
    }
}

decl_flavor!(GoFlavor, core);
