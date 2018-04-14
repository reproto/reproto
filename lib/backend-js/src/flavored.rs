//! JavaScript flavor.

#![allow(unused)]

use backend::package_processor;
use core::errors::Result;
use core::{self, CoreFlavor, Flavor, FlavorTranslator, Loc, PackageTranslator, Translate,
           Translator};
use genco::js::{self, JavaScript};
use genco::{Cons, Element, IntoTokens, Tokens};
use naming::{self, Naming};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;
use {Options, TYPE_SEP};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JavaScriptType<'el> {
    Native,
    Array {
        argument: Box<JavaScriptType<'el>>,
    },
    Map {
        key: Box<JavaScriptType<'el>>,
        value: Box<JavaScriptType<'el>>,
    },
    Name {
        js: JavaScript<'el>,
    },
}

impl<'el> JavaScriptType<'el> {
    /// Build decode method.
    pub fn decode(&self, var: Tokens<'el, JavaScript<'el>>) -> Tokens<'el, JavaScript<'el>> {
        use self::JavaScriptType::*;

        match *self {
            Native => toks![var],
            ref v if v.is_native() => toks![var],
            Array { ref argument } => {
                let a = argument.decode("v".into());
                toks![var, ".map(function(v) { return ", a, "; })"]
            }
            Map { ref key, ref value } => {
                let k = key.decode("k".into());
                let v = value.decode("data[k]".into());

                let mut t = Tokens::new();

                t.append("(function(data) {");
                t.append(" let o = {};");
                t.append(" for (let k in data) {");
                t.append(toks![" o[", k, "] = ", v, ";"]);
                t.append(" };");
                t.append(" return o;");
                t.append(toks![" })(", var, ")"]);

                t
            }
            Name { ref js } => toks![js.clone(), ".decode(", var, ")"],
        }
    }

    /// Build encode method.
    pub fn encode(&self, var: Tokens<'el, JavaScript<'el>>) -> Tokens<'el, JavaScript<'el>> {
        use self::JavaScriptType::*;

        match *self {
            Native => toks![var],
            ref v if v.is_native() => toks![var],
            Array { ref argument } => {
                let v = argument.encode("v".into());
                toks![var, ".map(function(v) { return ", v, "; })"]
            }
            Map { ref key, ref value } => {
                let k = key.encode("k".into());
                let v = value.encode("data[k]".into());

                let mut t = Tokens::new();

                t.append("(function(data) {");
                t.append(" let o = {};");
                t.append(" for (let k in data) {");
                t.append(toks![" o[", k, "] = ", v, ";"]);
                t.append(" };");
                t.append(" return o;");
                t.append(toks![" })(", var, ")"]);

                t
            }
            Name { ref js } => toks![var, ".encode()"],
        }
    }

    /// Check if the current type is completely native.
    fn is_native(&self) -> bool {
        use self::JavaScriptType::*;

        match *self {
            Native => true,
            Array { ref argument } => argument.is_native(),
            Map { ref key, ref value } => key.is_native() && value.is_native(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavaScriptName {
    pub name: JavaScript<'static>,
    pub package: RpPackage,
}

impl fmt::Display for JavaScriptName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(fmt)
    }
}

impl<'el> From<&'el JavaScriptName> for Element<'el, JavaScript<'el>> {
    fn from(value: &'el JavaScriptName) -> Element<'el, JavaScript<'el>> {
        Element::Literal(value.name.clone().to_string().into())
    }
}

impl package_processor::Name<JavaScriptFlavor> for JavaScriptName {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaScriptFlavor;

impl Flavor for JavaScriptFlavor {
    type Type = JavaScriptType<'static>;
    type Name = JavaScriptName;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
    type EnumType = RpEnumType;
}

/// Responsible for translating RpType -> JavaScript type.
pub struct JavaScriptFlavorTranslator {
    packages: Rc<Packages>,
}

impl JavaScriptFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        Self { packages }
    }
}

impl FlavorTranslator for JavaScriptFlavorTranslator {
    type Source = CoreFlavor;
    type Target = JavaScriptFlavor;

    translator_defaults!(Self, field, endpoint, enum_type);

    fn translate_i32(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_i64(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_u32(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_u64(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_float(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_double(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_boolean(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_string(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_datetime(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_array(
        &self,
        argument: JavaScriptType<'static>,
    ) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Array {
            argument: Box::new(argument),
        })
    }

    fn translate_map(
        &self,
        key: JavaScriptType<'static>,
        value: JavaScriptType<'static>,
    ) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Map {
            key: Box::new(key),
            value: Box::new(value),
        })
    }

    fn translate_any(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_bytes(&self) -> Result<JavaScriptType<'static>> {
        Ok(JavaScriptType::Native)
    }

    fn translate_name(&self, reg: RpReg, name: RpName) -> Result<JavaScriptType<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        if let Some(ref used) = name.prefix {
            let package = name.package.join(".");
            return Ok(JavaScriptType::Name {
                js: js::imported(package, ident).alias(used.to_string()),
            });
        }

        Ok(JavaScriptType::Name {
            js: js::local(ident),
        })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        Ok(self.packages.translate_package(source)?)
    }

    fn translate_local_name<T>(
        &self,
        translator: &T,
        reg: RpReg,
        name: core::RpName<CoreFlavor>,
    ) -> Result<JavaScriptName>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));
        let package = self.translate_package(name.package)?;

        Ok(JavaScriptName {
            name: js::local(ident),
            package,
        })
    }
}

decl_flavor!(JavaScriptFlavor, core);
