//! Python flavor.

#![allow(unused)]

use backend::package_processor;
use core::errors::Result;
use core::{self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, Loc, PackageTranslator,
           Translate, Translator};
use genco::python::{self, Python};
use genco::{Cons, Element, IntoTokens, Tokens};
use naming::{self, Naming};
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;
use utils::{Exception, VersionHelper};
use {Options, TYPE_SEP};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PythonKind<'el> {
    Native,
    Integer,
    Float,
    Boolean,
    String,
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

impl<'el> PythonKind<'el> {
    /// Check if the current type is completely native.
    fn is_native(&self) -> bool {
        use self::PythonKind::*;

        match *self {
            Native => true,
            Integer | Float | Boolean | String => true,
            Array { ref argument } => argument.kind.is_native(),
            Map { ref key, ref value } => key.kind.is_native() && value.kind.is_native(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PythonType<'el> {
    helper: Rc<Box<VersionHelper>>,
    kind: PythonKind<'el>,
}

impl<'el> cmp::PartialEq for PythonType<'el> {
    fn eq(&self, other: &PythonType<'el>) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl<'el> cmp::Eq for PythonType<'el> {}

impl<'el> PythonType<'el> {
    /// Build decode method.
    ///
    /// `var` is the name of the variable we finally want to assign.
    /// `l` helps us generate unique local variables, and should be incremented one level for every
    /// nested call of `decode`.
    pub fn decode<V>(&self, var: V, l: usize) -> Option<Tokens<'el, Python<'el>>>
    where
        V: Into<Cons<'el>>,
    {
        use self::PythonKind::*;

        let var = var.into();

        match self.kind {
            Integer => {
                let mut t = Tokens::new();
                push!(t, "if not isinstance(", var, ", int):");
                nested!(t, "raise ", Exception("not an integer"));
                Some(t)
            }
            Float => {
                let mut t = Tokens::new();
                push!(t, "if not isinstance(", var, ", float):");
                nested!(t, "raise ", Exception("not a float"));
                Some(t)
            }
            Boolean => {
                let mut t = Tokens::new();
                push!(t, "if not isinstance(", var, ", bool):");
                nested!(t, "raise ", Exception("not a boolean"));
                Some(t)
            }
            String => {
                let test = self.helper.is_string(var);

                let mut t = Tokens::new();
                push!(t, "if not ", test, ":");
                nested!(t, "raise ", Exception("not a string"));
                Some(t)
            }
            Native => None,
            Array { ref argument } => {
                let mut t = Tokens::new();

                let v = Rc::new(format!("_v{}", l));
                let a = Rc::new(format!("_a{}", l));

                t.push_into(|t| {
                    push!(t, "if not isinstance(", var, ", list):");
                    nested!(t, "raise ", Exception("not an array"));
                });

                push!(t, a, " = []");

                t.push_into(|t| {
                    push!(t, "for ", v, " in ", var, ":");

                    t.nested_into(|mut t| {
                        if let Some(d) = argument.decode(v.clone(), l + 1) {
                            t.push(d);
                        }

                        push!(t, a, ".append(", v, ")");
                    });
                });

                push!(t, var, " = ", a);
                Some(t.join_line_spacing())
            }
            Map { ref key, ref value } => {
                let mut t = Tokens::new();

                let o = Rc::new(format!("_o{}", l));
                let k = Rc::new(format!("_k{}", l));
                let v = Rc::new(format!("_v{}", l));

                t.push_into(|t| {
                    push!(t, "if not isinstance(", var, ", dict):");
                    nested!(t, "raise ", Exception("not an object"));
                });

                push!(t, o, " = {}");

                t.push_into(|t| {
                    push!(t, "for ", k, ", ", v, " in ", var, ".items():");

                    t.nested_into(|mut t| {
                        if let Some(d) = key.decode(k.clone(), l + 1) {
                            t.push(d);
                        }

                        if let Some(d) = value.decode(v.clone(), l + 1) {
                            t.push(d);
                        }

                        push!(t, o, "[", k, "] = ", v);
                    });
                });

                push!(t, var, " = ", o);
                Some(t.join_line_spacing())
            }
            Name { ref python } => Some(toks!(
                var.clone(),
                " = ",
                python.clone(),
                ".decode(",
                var.clone(),
                ")"
            )),
        }
    }

    /// Build encode method.
    pub fn encode(&self, var: Tokens<'el, Python<'el>>) -> Tokens<'el, Python<'el>> {
        use self::PythonKind::*;

        match self.kind {
            Integer | Float | Boolean | Native | String => toks![var],
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
    type EnumType = RpEnumType;
}

/// Responsible for translating RpType -> Python type.
pub struct PythonFlavorTranslator {
    packages: Rc<Packages>,
    helper: Rc<Box<VersionHelper>>,
}

impl PythonFlavorTranslator {
    pub fn new(packages: Rc<Packages>, helper: Rc<Box<VersionHelper>>) -> Self {
        Self { packages, helper }
    }

    fn ty(&self, kind: PythonKind<'static>) -> PythonType<'static> {
        PythonType {
            helper: self.helper.clone(),
            kind: kind,
        }
    }
}

impl FlavorTranslator for PythonFlavorTranslator {
    type Source = CoreFlavor;
    type Target = PythonFlavor;

    translator_defaults!(Self, field, endpoint, enum_type);

    fn translate_i32(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Integer))
    }

    fn translate_i64(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Integer))
    }

    fn translate_u32(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Integer))
    }

    fn translate_u64(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Integer))
    }

    fn translate_float(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Float))
    }

    fn translate_double(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Float))
    }

    fn translate_boolean(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Boolean))
    }

    fn translate_string(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::String))
    }

    fn translate_datetime(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::String))
    }

    fn translate_array(&self, argument: PythonType<'static>) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Array {
            argument: Box::new(argument),
        }))
    }

    fn translate_map(
        &self,
        key: PythonType<'static>,
        value: PythonType<'static>,
    ) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Map {
            key: Box::new(key),
            value: Box::new(value),
        }))
    }

    fn translate_any(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::Native))
    }

    fn translate_bytes(&self) -> Result<PythonType<'static>> {
        Ok(self.ty(PythonKind::String))
    }

    fn translate_name(&self, reg: RpReg, name: Loc<RpName>) -> Result<PythonType<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));

        if let Some(ref used) = name.prefix {
            let package = name.package.join(".");

            return Ok(self.ty(PythonKind::Name {
                python: python::imported(package)
                    .alias(used.to_string())
                    .name(ident)
                    .into(),
            }));
        }

        Ok(self.ty(PythonKind::Name {
            python: python::local(ident),
        }))
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
    ) -> Result<PythonName>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let (name, span) = Loc::take_pair(name);

        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |v| v.join(TYPE_SEP));
        let package = self.translate_package(name.package)?;

        Ok(PythonName {
            name: python::local(ident),
            package,
        })
    }
}

decl_flavor!(PythonFlavor, core);
