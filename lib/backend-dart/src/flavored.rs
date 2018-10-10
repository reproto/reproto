//! Dart flavor.

#![allow(unused)]

use core::errors::Result;
use core::{
    self, CoreFlavor, Diagnostics, Flavor, FlavorTranslator, Loc, PackageTranslator, RpNumberKind,
    RpNumberType, RpStringType, Translate, Translator,
};
use genco::dart;
use genco::{Cons, Dart, Tokens};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;
use TYPE_SEP;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DartType<'el> {
    Native {
        dart: Dart<'el>,
    },
    Dynamic,
    Int,
    Double,
    Bool,
    String,
    Array {
        argument: Box<DartType<'el>>,
    },
    Map {
        key: Box<DartType<'el>>,
        value: Box<DartType<'el>>,
    },
}

impl<'el> DartType<'el> {
    /// Get the dart type for the type.
    pub fn ty(&self) -> Dart<'el> {
        match *self {
            DartType::Native { ref dart } => dart.clone(),
            DartType::Dynamic => Dart::Dynamic,
            DartType::Int => dart::INT,
            DartType::Double => dart::DOUBLE,
            DartType::Bool => dart::BOOL,
            DartType::String => dart::imported(dart::DART_CORE).name("String"),
            DartType::Array { ref argument } => {
                let argument = argument.ty();
                dart::imported(dart::DART_CORE).name("List").with_arguments(vec![argument])
            }
            DartType::Map { ref key, ref value } => {
                let key = key.ty();
                let value = value.ty();
                dart::imported(dart::DART_CORE).name("Map").with_arguments(vec![key, value])
            }
        }
    }

    /// Create a decode function appropriate for this type.
    pub fn encode(&self, i: Tokens<'el, Dart<'el>>) -> Result<Tokens<'el, Dart<'el>>> {
        let _ = match *self {
            DartType::Native { ref dart } => {
                return Ok(toks!(i, ".encode()"));
            },
            DartType::Dynamic => Dart::Dynamic,
            DartType::Int => dart::INT,
            DartType::Double => dart::DOUBLE,
            DartType::Bool => dart::BOOL,
            DartType::String => dart::imported(dart::DART_CORE).name("String"),
            DartType::Map { ref key, ref value } => {
                let d = value.encode(toks!("e.value"))?;
                return Ok(toks!("Map.fromEntries(", i, ".entries.map((e) => MapEntry(e.key, ", d, ")))"));
            }
            DartType::Array { ref argument } => {
                let d = argument.encode(toks!("e"))?;
                return Ok(toks!("List.from(", i, ".map((e) => ", d, "))"));
            }
        };

        Ok(i)
    }

    /// Create a decode function appropriate for this type.
    /// The first tuple element returned is the decoding procedure of the argument.
    /// The second optional tuple element is extra validation that needs to be evaluated.
    pub fn decode(&self, i: impl Into<Cons<'el>>) -> Result<(
        Tokens<'el, Dart<'el>>,
        Option<Tokens<'el, Dart<'el>>>
    )> {
        let i = i.into();

        let ty = match *self {
            DartType::Native { ref dart } => {
                return Ok((toks!(dart.clone(), ".decode(", i.clone(), ")"), None));
            },
            DartType::Dynamic => Dart::Dynamic,
            DartType::Int => dart::INT,
            DartType::Double => dart::DOUBLE,
            DartType::Bool => dart::BOOL,
            DartType::String => dart::imported(dart::DART_CORE).name("String"),
            DartType::Map { ref key, ref value } => {
                let (d, e) = value.decode("e.value")?;

                let core = dart::imported(dart::DART_CORE);
                let dyn_ty = core.name("Map").with_arguments(vec![key.ty(), Dart::Dynamic]);
                // let ty = core.name("Map").with_arguments(vec![key.ty(), value.ty()]);

                let entries = toks!("(", i.clone(), " as ", dyn_ty.clone(), ").entries");

                let t = if let Some(e) = e {
                    let mut t = Tokens::new();
                    t.append(toks!("Map.fromEntries(", entries, ".map((e) {"));
                    t.nested(e);
                    nested!(t, "return MapEntry(e.key, ", d, ");");
                    push!(t, "}));");
                    t
                } else {
                    toks!("Map.fromEntries(", entries, ".map((e) => MapEntry(e.key, ", d, ")))")
                };

                // check that value is a map.
                let mut e = Tokens::new();
                push!(e, "if (!(", i, " is ", dyn_ty, ")) {");
                nested!(e, "throw 'expected ", dyn_ty, ", but was: $", i, "';");
                push!(e, "}");

                return Ok((t, Some(e)));
            }
            DartType::Array { ref argument } => {
                let (d, e) = argument.decode("e")?;

                let core = dart::imported(dart::DART_CORE);
                let string = core.name("String");
                let dyn_ty = core.name("List").with_arguments(vec![Dart::Dynamic]);

                let entries = toks!("(", i.clone(), " as ", dyn_ty.clone(), ")");

                let t = if let Some(e) = e {
                    let mut t = Tokens::new();
                    t.append(toks!("List.of(", entries, ".map((e) {"));
                    t.nested(e);
                    nested!(t, "return ", d, ";");
                    push!(t, "}))");
                    t
                } else {
                    toks!("List.of(", entries, ".map((e) => ", d, "))")
                };

                // check that value is a list.
                let mut e = Tokens::new();
                push!(e, "if (!(", i, " is ", dyn_ty, ")) {");
                nested!(e, "throw 'expected ", dyn_ty, ", but was: $", i, "';");
                push!(e, "}");

                return Ok((t, Some(e)));
            }
        };

        let mut e = Tokens::new();
        push!(e, "if (!(", i, " is ", ty, ")) {");
        nested!(e, "throw 'expected ", ty, ", but was: $", i, "';");
        push!(e, "}");
        Ok((toks!(i), Some(e)))
    }
}

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
    type Type = DartType<'static>;
    type Name = Loc<RpName>;
    type Field = core::RpField<DartFlavor>;
    type Endpoint = DartEndpoint;
    type Package = core::RpPackage;
    type EnumType = DartType<'static>;
}

/// Responsible for translating RpType -> Dart type.
pub struct DartFlavorTranslator {
    packages: Rc<Packages>,
    map: Dart<'static>,
    list: Dart<'static>,
    string: Dart<'static>,
}

impl DartFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        let core = dart::imported(dart::DART_CORE);

        Self {
            packages,
            map: core.name("Map"),
            list: core.name("List"),
            string: core.name("String"),
        }
    }
}

impl FlavorTranslator for DartFlavorTranslator {
    type Source = CoreFlavor;
    type Target = DartFlavor;

    translator_defaults!(Self, local_name, field);

    fn translate_number(&self, number: RpNumberType) -> Result<DartType<'static>> {
        Ok(DartType::Int)
    }

    fn translate_float(&self) -> Result<DartType<'static>> {
        Ok(DartType::Double)
    }

    fn translate_double(&self) -> Result<DartType<'static>> {
        Ok(DartType::Double)
    }

    fn translate_boolean(&self) -> Result<DartType<'static>> {
        Ok(DartType::Bool)
    }

    fn translate_string(&self, _: RpStringType) -> Result<DartType<'static>> {
        Ok(DartType::String)
    }

    fn translate_datetime(&self) -> Result<DartType<'static>> {
        Ok(DartType::String)
    }

    fn translate_array(&self, argument: DartType<'static>) -> Result<DartType<'static>> {
        Ok(DartType::Array {
            argument: Box::new(argument),
        })
    }

    fn translate_map(&self, key: DartType<'static>, value: DartType<'static>) -> Result<DartType<'static>> {
        Ok(DartType::Map {
            key: Box::new(key),
            value: Box::new(value),
        })
    }

    fn translate_any(&self) -> Result<DartType<'static>> {
        Ok(DartType::Dynamic)
    }

    fn translate_bytes(&self) -> Result<DartType<'static>> {
        Ok(DartType::String)
    }

    fn translate_name(&self, reg: RpReg, name: Loc<RpName>) -> Result<DartType<'static>> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        if let Some(ref prefix) = name.prefix {
            let package_name = name.package.join("/");
            let dart = dart::imported(package_name)
                .name(ident)
                .alias(prefix.to_string());
            return Ok(DartType::Native { dart });
        }

        Ok(DartType::Native {
            dart: dart::local(ident),
        })
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
    ) -> Result<DartType<'static>>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        use core::RpEnumType::*;

        match enum_type {
            String(string) => self.translate_string(string),
            Number(number) => self.translate_number(number),
        }
    }
}

decl_flavor!(DartFlavor, core);
