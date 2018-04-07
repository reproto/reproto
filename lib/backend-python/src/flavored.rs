//! Python flavor.

#![allow(unused)]

use backend::{package_processor, PackageUtils};
use core::errors::Result;
use core::{self, CoreFlavor, Flavor, FlavorTranslator, Loc, PackageTranslator, Translate,
           Translator};
use genco::python::{self, Python};
use genco::{Cons, Element, IntoTokens, Tokens};
use naming::{self, Naming};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use {Options, PythonPackageUtils, TYPE_SEP};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PythonType<'el> {
    Native,
    Array {
        argument: Box<PythonType<'el>>,
    },
    Map {
        key: Box<PythonType<'el>>,
        value: Box<PythonType<'el>>,
    },
    Name {
        python: Python<'el>,
    },
}

impl<'el> PythonType<'el> {
    /// Build decode method.
    pub fn decode(&self, var: Tokens<'el, Python<'el>>) -> Tokens<'el, Python<'el>> {
        use self::PythonType::*;

        match *self {
            Native => toks![var],
            ref v if v.is_native() => toks![var],
            Array { ref argument } => {
                toks!["[", argument.decode("v".into()), " for v in ", var, "]",]
            }
            Map { ref key, ref value } => {
                let k = key.decode("k".into());
                let v = value.decode("v".into());
                toks!["dict((", k, ", ", v, ") for (k, v) in ", var, ".items())",]
            }
            Name { ref python } => toks![python.clone(), ".decode(", var, ")"],
        }
    }

    /// Build encode method.
    pub fn encode(&self, var: Tokens<'el, Python<'el>>) -> Tokens<'el, Python<'el>> {
        use self::PythonType::*;

        match *self {
            Native => toks![var],
            ref v if v.is_native() => toks![var],
            Array { ref argument } => {
                let v = argument.encode("v".into());
                toks!["[", v, " for v in ", var, "]"]
            }
            Map { ref key, ref value } => {
                let k = key.encode("k".into());
                let v = value.encode("v".into());
                toks!["dict((", k, ", ", v, ") for (k, v) in ", var, ".items())",]
            }
            Name { ref python } => toks![var, ".encode()"],
        }
    }

    /// Check if the current type is completely native.
    fn is_native(&self) -> bool {
        use self::PythonType::*;

        match *self {
            Native => true,
            Array { ref argument } => argument.is_native(),
            Map { ref key, ref value } => key.is_native() && value.is_native(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonName {
    pub name: Python<'static>,
    pub package: RpPackage,
}

impl fmt::Display for PythonName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(fmt)
    }
}

impl<'el> From<&'el PythonName> for Element<'el, Python<'el>> {
    fn from(value: &'el PythonName) -> Element<'el, Python<'el>> {
        Element::Literal(value.name.clone().to_string().into())
    }
}

impl package_processor::Name<PythonFlavor> for PythonName {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PythonFlavor;

impl Flavor for PythonFlavor {
    type Type = PythonType<'static>;
    type Name = PythonName;
    type Field = RpField;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
}

/// Responsible for translating RpType -> Python type.
pub struct PythonFlavorTranslator {
    package_translator: HashMap<RpVersionedPackage, RpPackage>,
    package_utils: Rc<PythonPackageUtils>,
}

impl PythonFlavorTranslator {
    pub fn new(
        package_translator: HashMap<RpVersionedPackage, RpPackage>,
        package_utils: Rc<PythonPackageUtils>,
    ) -> Self {
        Self {
            package_translator,
            package_utils,
        }
    }
}

impl FlavorTranslator for PythonFlavorTranslator {
    type Source = CoreFlavor;
    type Target = PythonFlavor;

    translator_defaults!(Self, field, endpoint);

    fn translate_i32(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_i64(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_u32(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_u64(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_float(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_double(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_boolean(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_string(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_datetime(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_array(&self, argument: PythonType<'static>) -> Result<PythonType<'static>> {
        Ok(PythonType::Array {
            argument: Box::new(argument),
        })
    }

    fn translate_map(
        &self,
        key: PythonType<'static>,
        value: PythonType<'static>,
    ) -> Result<PythonType<'static>> {
        Ok(PythonType::Map {
            key: Box::new(key),
            value: Box::new(value),
        })
    }

    fn translate_any(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_bytes(&self) -> Result<PythonType<'static>> {
        Ok(PythonType::Native)
    }

    fn translate_name(&self, reg: RpReg, name: RpName) -> Result<PythonType<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));

        if let Some(ref used) = name.prefix {
            let package = name.package.join(".");

            return Ok(PythonType::Name {
                python: python::imported(package)
                    .alias(used.to_string())
                    .name(ident)
                    .into(),
            });
        }

        Ok(PythonType::Name {
            python: python::local(ident),
        })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        let package = self.package_translator.translate_package(source)?;
        Ok(package)
    }

    fn translate_local_name<T>(
        &self,
        translator: &T,
        reg: RpReg,
        name: core::RpName<CoreFlavor>,
    ) -> Result<PythonName>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));
        let package = self.translate_package(name.package)?;

        Ok(PythonName {
            name: python::local(ident),
            package,
        })
    }
}

decl_flavor!(PythonFlavor, core);
