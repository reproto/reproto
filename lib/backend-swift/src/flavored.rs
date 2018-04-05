//! Swift flavor.

#![allow(unused)]

use backend::PackageUtils;
use core::errors::Result;
use core::{self, CoreFlavor, Flavor, FlavorTranslator, Loc, PackageTranslator, Translate,
           Translator};
use genco::Cons;
use genco::swift::{self, Swift};
use naming::{self, Naming};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use {Options, SwiftPackageUtils, TYPE_SEP};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwiftFlavor;

impl Flavor for SwiftFlavor {
    type Type = Swift<'static>;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
}

/// Responsible for translating RpType -> Swift type.
pub struct SwiftFlavorTranslator {
    package_translator: HashMap<RpVersionedPackage, RpPackage>,
    package_utils: Rc<SwiftPackageUtils>,
    data: Swift<'static>,
    date: Swift<'static>,
    any: Swift<'static>,
    to_upper_camel: naming::ToUpperCamel,
}

impl SwiftFlavorTranslator {
    pub fn new(
        package_translator: HashMap<RpVersionedPackage, RpPackage>,
        package_utils: Rc<SwiftPackageUtils>,
        options: &Options,
    ) -> Result<Self> {
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
            package_translator,
            package_utils,
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

    fn translate_i32(&self) -> Result<Swift<'static>> {
        Ok(swift::local("Int32"))
    }

    fn translate_i64(&self) -> Result<Swift<'static>> {
        Ok(swift::local("Int64"))
    }

    fn translate_u32(&self) -> Result<Swift<'static>> {
        Ok(swift::local("UInt32"))
    }

    fn translate_u64(&self) -> Result<Swift<'static>> {
        Ok(swift::local("UInt64"))
    }

    fn translate_float(&self) -> Result<Swift<'static>> {
        Ok(swift::local("Float2"))
    }

    fn translate_double(&self) -> Result<Swift<'static>> {
        Ok(swift::local("Double"))
    }

    fn translate_boolean(&self) -> Result<Swift<'static>> {
        Ok(swift::local("Bool"))
    }

    fn translate_string(&self) -> Result<Swift<'static>> {
        Ok(swift::local("String"))
    }

    fn translate_datetime(&self) -> Result<Swift<'static>> {
        Ok(self.date.clone())
    }

    fn translate_array(&self, argument: Swift<'static>) -> Result<Swift<'static>> {
        Ok(swift::array(argument))
    }

    fn translate_map(&self, key: Swift<'static>, value: Swift<'static>) -> Result<Swift<'static>> {
        Ok(swift::map(key, value))
    }

    fn translate_any(&self) -> Result<Swift<'static>> {
        Ok(self.any.clone())
    }

    fn translate_bytes(&self) -> Result<Swift<'static>> {
        Ok(swift::local("string"))
    }

    fn translate_name(&self, name: RpName, reg: RpReg) -> Result<Swift<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let package_name = name.package.join("_");
        return Ok(swift::local(format!("{}_{}", package_name, ident)));
    }

    fn translate_field<T>(
        &self,
        translator: &T,
        field: core::RpField<CoreFlavor>,
    ) -> Result<core::RpField<SwiftFlavor>>
    where
        T: Translator<Source = CoreFlavor, Target = SwiftFlavor>,
    {
        field.translate(translator)
    }

    fn translate_endpoint<T>(
        &self,
        translator: &T,
        endpoint: core::RpEndpoint<CoreFlavor>,
    ) -> Result<RpEndpoint>
    where
        T: Translator<Source = CoreFlavor, Target = SwiftFlavor>,
    {
        endpoint.translate(translator)
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        let package = self.package_translator.translate_package(source)?;
        let package = package.with_naming(|n| self.to_upper_camel.convert(n));
        Ok(package)
    }
}

decl_flavor!(SwiftFlavor, core);
