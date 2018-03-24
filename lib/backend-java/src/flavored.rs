//! Java flavor.

#![allow(unused)]

use core::{self, CoreFlavor, Flavor, Translator, TypeTranslator};
use core::errors::Result;
use genco::Java;
use genco::java::{imported, BOOLEAN, DOUBLE, FLOAT, INTEGER, LONG};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct JavaFlavor;

impl Flavor for JavaFlavor {
    type Type = Java<'static>;
    type Field = RpField;
}

/// Responsible for translating RpType -> Java type.
pub struct JavaTypeTranslator {
    list: Java<'static>,
    map: Java<'static>,
    string: Java<'static>,
    instant: Java<'static>,
    object: Java<'static>,
    byte_buffer: Java<'static>,
    pub void: Java<'static>,
}

impl JavaTypeTranslator {
    pub fn new() -> Self {
        Self {
            list: imported("java.util", "List"),
            map: imported("java.util", "Map"),
            string: imported("java.lang", "String"),
            instant: imported("java.time", "Instant"),
            object: imported("java.lang", "Object"),
            byte_buffer: imported("java.nio", "ByteBuffer"),
            void: imported("java.lang", "Void"),
        }
    }
}

impl TypeTranslator for JavaTypeTranslator {
    type Source = CoreFlavor;
    type Target = JavaFlavor;

    fn translate_i32(&self) -> Result<Java<'static>> {
        Ok(INTEGER.into())
    }

    fn translate_i64(&self) -> Result<Java<'static>> {
        Ok(LONG.into())
    }

    fn translate_u32(&self) -> Result<Java<'static>> {
        Ok(INTEGER.into())
    }

    fn translate_u64(&self) -> Result<Java<'static>> {
        Ok(LONG.into())
    }

    fn translate_float(&self) -> Result<Java<'static>> {
        Ok(FLOAT.into())
    }

    fn translate_double(&self) -> Result<Java<'static>> {
        Ok(DOUBLE.into())
    }

    fn translate_boolean(&self) -> Result<Java<'static>> {
        Ok(BOOLEAN.into())
    }

    fn translate_string(&self) -> Result<Java<'static>> {
        Ok(self.string.clone().into())
    }

    fn translate_datetime(&self) -> Result<Java<'static>> {
        Ok(self.instant.clone().into())
    }

    fn translate_array(&self, argument: Java<'static>) -> Result<Java<'static>> {
        Ok(self.list.with_arguments(vec![argument]))
    }

    fn translate_map(&self, key: Java<'static>, value: Java<'static>) -> Result<Java<'static>> {
        Ok(self.map.with_arguments(vec![key, value]))
    }

    fn translate_any(&self) -> Result<Java<'static>> {
        Ok(self.object.clone())
    }

    fn translate_bytes(&self) -> Result<Java<'static>> {
        Ok(self.byte_buffer.clone())
    }

    fn translate_name(&self, name: RpName, reg: RpReg) -> Result<Java<'static>> {
        let pkg = name.package
            .as_package(|version| format!("_{}", version).replace(".", "_").replace("-", "_"));
        let package_name = Rc::new(pkg.parts.join("."));
        let name = Rc::new(reg.ident(&name, |p| p.join("."), |c| c.join(".")));
        Ok(imported(package_name, name))
    }

    fn translate_field<T>(
        &self,
        translator: &T,
        field: core::RpField<CoreFlavor>,
    ) -> Result<core::RpField<JavaFlavor>>
    where
        T: Translator<Source = CoreFlavor, Target = JavaFlavor>,
    {
        Ok(RpField {
            required: field.required,
            safe_ident: field.safe_ident,
            ident: field.ident,
            comment: field.comment,
            ty: translator.translate_type(field.ty)?,
            field_as: field.field_as,
        })
    }
}

decl_flavor!(JavaFlavor, core);
