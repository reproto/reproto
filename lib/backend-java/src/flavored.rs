//! Java flavor.

#![allow(unused)]

use core::{self, CoreFlavor, Flavor, Translator, TypeTranslator};
use core::errors::Result;
use genco::{Cons, Java};
use genco::java::{imported, optional, Field, BOOLEAN, DOUBLE, FLOAT, INTEGER, LONG};
use java_field::JavaField;
use naming::{self, Naming};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct JavaFlavor;

impl Flavor for JavaFlavor {
    type Type = Java<'static>;
    type Field = JavaField<'static>;
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
    optional: Java<'static>,
    to_upper_camel: naming::ToUpperCamel,
    to_lower_camel: naming::ToLowerCamel,
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
            optional: imported("java.util", "Optional"),
            to_upper_camel: naming::to_upper_camel(),
            to_lower_camel: naming::to_lower_camel(),
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
    ) -> Result<JavaField<'static>>
    where
        T: Translator<Source = CoreFlavor, Target = JavaFlavor>,
    {
        let ty = translator.translate_type(field.ty.clone())?;

        let java_type = if field.is_optional() {
            optional(ty.clone(), self.optional.with_arguments(vec![ty.clone()]))
        } else {
            ty
        };

        let ident = Rc::new(self.to_lower_camel.convert(field.safe_ident()));
        let field_accessor = Rc::new(self.to_upper_camel.convert(field.ident()));

        let name = Cons::from(field.name().to_string());

        let mut spec = Field::new(java_type, ident.clone());

        if !field.comment.is_empty() {
            spec.comments.push("<pre>".into());
            spec.comments
                .extend(field.comment.into_iter().map(Cons::from));
            spec.comments.push("</pre>".into());
        }

        Ok(JavaField {
            name: name,
            ident: ident,
            field_accessor: field_accessor,
            spec: spec,
        })
    }
}

decl_flavor!(JavaFlavor, core);
