//! C# flavor.

#![allow(unused)]

use CsharpPackageUtils;
use backend::PackageUtils;
use core::errors::Result;
use core::{self, CoreFlavor, Flavor, FlavorTranslator, Loc, PackageTranslator, Translate,
           Translator};
use genco::csharp::{self, array, struct_, using};
use genco::{Cons, Csharp};
use naming::{self, Naming};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CsharpFlavor;

impl Flavor for CsharpFlavor {
    type Type = Csharp<'static>;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = core::RpPackage;
}

/// Responsible for translating RpType -> Csharp type.
pub struct CsharpFlavorTranslator {
    package_translator: HashMap<RpVersionedPackage, RpPackage>,
    package_utils: Rc<CsharpPackageUtils>,
    list: Csharp<'static>,
    dictionary: Csharp<'static>,
    string: Csharp<'static>,
    date_time: Csharp<'static>,
    object: Csharp<'static>,
    pub void: Csharp<'static>,
    to_upper_camel: naming::ToUpperCamel,
}

impl CsharpFlavorTranslator {
    pub fn new(
        package_translator: HashMap<RpVersionedPackage, RpPackage>,
        package_utils: Rc<CsharpPackageUtils>,
    ) -> Self {
        Self {
            package_translator,
            package_utils,
            list: using("System.Collections.Generic", "List"),
            dictionary: using("System.Collections.Generic", "Dictionary"),
            string: using("System", "String"),
            date_time: struct_(using("System", "DateTime")),
            object: using("System", "Object"),
            void: using("java.lang", "Void"),
            to_upper_camel: naming::to_upper_camel(),
        }
    }
}

impl FlavorTranslator for CsharpFlavorTranslator {
    type Source = CoreFlavor;
    type Target = CsharpFlavor;

    fn translate_i32(&self) -> Result<Csharp<'static>> {
        Ok(csharp::INT32.into())
    }

    fn translate_i64(&self) -> Result<Csharp<'static>> {
        Ok(csharp::INT64.into())
    }

    fn translate_u32(&self) -> Result<Csharp<'static>> {
        Ok(csharp::UINT32.into())
    }

    fn translate_u64(&self) -> Result<Csharp<'static>> {
        Ok(csharp::UINT64.into())
    }

    fn translate_float(&self) -> Result<Csharp<'static>> {
        Ok(csharp::SINGLE.into())
    }

    fn translate_double(&self) -> Result<Csharp<'static>> {
        Ok(csharp::DOUBLE.into())
    }

    fn translate_boolean(&self) -> Result<Csharp<'static>> {
        Ok(csharp::BOOLEAN.into())
    }

    fn translate_string(&self) -> Result<Csharp<'static>> {
        Ok(self.string.clone())
    }

    fn translate_datetime(&self) -> Result<Csharp<'static>> {
        Ok(self.date_time.clone())
    }

    fn translate_array(&self, inner: Csharp<'static>) -> Result<Csharp<'static>> {
        Ok(self.list.with_arguments(vec![inner]).into())
    }

    fn translate_map(
        &self,
        key: Csharp<'static>,
        value: Csharp<'static>,
    ) -> Result<Csharp<'static>> {
        Ok(self.dictionary.with_arguments(vec![key, value]).into())
    }

    fn translate_any(&self) -> Result<Csharp<'static>> {
        Ok(self.object.clone())
    }

    fn translate_bytes(&self) -> Result<Csharp<'static>> {
        Ok(array(csharp::BYTE))
    }

    fn translate_name(&self, name: RpName, reg: RpReg) -> Result<Csharp<'static>> {
        let package_name = Rc::new(name.package.join("."));
        let name = Rc::new(reg.ident(&name, |p| p.join("."), |c| c.join(".")));

        let ty = using(package_name, name);

        if reg.is_enum() {
            return Ok(ty.into_enum());
        } else {
            return Ok(ty);
        }
    }

    fn translate_field<T>(
        &self,
        translator: &T,
        field: core::RpField<CoreFlavor>,
    ) -> Result<RpField>
    where
        T: Translator<Source = CoreFlavor, Target = CsharpFlavor>,
    {
        field.translate(translator)
    }

    fn translate_endpoint<T>(
        &self,
        translator: &T,
        endpoint: core::RpEndpoint<CoreFlavor>,
    ) -> Result<RpEndpoint>
    where
        T: Translator<Source = CoreFlavor, Target = CsharpFlavor>,
    {
        endpoint.translate(translator)
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        let package = self.package_translator.translate_package(source)?;
        let package = package.with_naming(|p| self.to_upper_camel.convert(p));
        Ok(self.package_utils.package(&package))
    }
}

decl_flavor!(CsharpFlavor, core);
