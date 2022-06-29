//! Swift flavor.

#![allow(unused)]

use crate::{Options, TYPE_SEP};
use backend::package_processor;
use genco::prelude::*;
use genco::tokens::{from_fn, FormatInto, ItemStr};
use naming::Naming;
use reproto_core::errors::Result;
use reproto_core::{
    CoreFlavor, Diagnostics, Flavor, FlavorField, FlavorTranslator, PackageTranslator,
    RpNumberKind, RpNumberType, RpStringType, Spanned, Translate, Translator,
};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Primitive {
    Bool,
    UInt32,
    UInt64,
    Int32,
    Int64,
    Float,
    Double,
}

impl FormatInto<Swift> for Primitive {
    fn format_into(self, t: &mut swift::Tokens) {
        match self {
            Self::Bool => quote_in!(*t => Bool),
            Self::UInt32 => quote_in!(*t => UInt32),
            Self::UInt64 => quote_in!(*t => UInt64),
            Self::Int32 => quote_in!(*t => Int32),
            Self::Int64 => quote_in!(*t => Int64),
            Self::Float => quote_in!(*t => Float),
            Self::Double => quote_in!(*t => Double),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Type {
    Primitive { primitive: Primitive },
    Any,
    String,
    DateTime { formatter: Rc<swift::Import> },
    Bytes { data: Rc<swift::Import> },
    Array { argument: Box<Type> },
    Dictionary { key: Box<Type>, value: Box<Type> },
    Local { ident: ItemStr },
    Name { name: swift::Import },
}

impl Type {
    pub(crate) fn local(ident: impl Into<ItemStr>) -> Self {
        Self::Local {
            ident: ident.into(),
        }
    }

    pub(crate) fn array(argument: impl Into<Type>) -> Self {
        Self::Array {
            argument: Box::new(argument.into()),
        }
    }

    pub(crate) fn map(key: impl Into<Type>, value: impl Into<Type>) -> Self {
        Self::Dictionary {
            key: Box::new(key.into()),
            value: Box::new(value.into()),
        }
    }

    /// Decode the given value.
    pub(crate) fn decode_value(&self, name: ItemStr, var: swift::Tokens) -> swift::Tokens {
        let name = &name;

        let unbox = match self {
            Type::DateTime { formatter } => {
                let string = quote!(try decode_value($var as? String));
                let date = quote!($(&**formatter)().date(from: $string));
                quote!(try decode_value($date))
            }
            Type::Bytes { data } => quote! {
                $(&**data)(base64Encoded: try decode_value($var as? String))
            },
            Type::Array { argument } => {
                let argument = argument.decode_value(name.clone(), quote!(inner));

                return quote! {
                    try decode_array($var, name: $(quoted(name)), inner: { inner in $argument })
                };
            }
            Type::Dictionary { value, .. } => {
                let value = value.decode_value(name.clone(), quote!(value));

                return quote! {
                    try decode_map($var, name: $(quoted(name)), value: { value in $value })
                };
            }
            Type::Local { ident } => {
                return quote!(try $ident.decode(json: $var));
            }
            Type::Name { name } => {
                return quote!(try $name.decode(json: $var));
            }
            Type::Any => var,
            Type::Primitive { primitive } => quote!(unbox($var, as: $(*primitive).self)),
            Type::String => quote!(unbox($var, as: String.self)),
        };

        quote! {
            try decode_name($unbox, name: $(quoted(name)))
        }
    }

    /// Decode the given value.
    pub(crate) fn encode_value(&self, name: &str, var: swift::Tokens) -> swift::Tokens {
        match self {
            Type::Primitive { .. } | Type::Any | Type::String => var,
            Type::DateTime { formatter } => quote!($(&**formatter)().string(from: $var)),
            Type::Bytes { .. } => quote!($var.base64EncodedString()),
            Type::Array { argument } => {
                let argument = argument.encode_value(name, quote!(inner));
                quote!(try encode_array($var, name: $(quoted(name)), inner: { inner in $argument }))
            }
            Type::Dictionary { value, .. } => {
                let value = value.encode_value(name, quote!(value));
                quote!(try encode_map($var, name: $(quoted(name)), value: { value in $value }))
            }
            Type::Name { .. } | Type::Local { .. } => quote!(try $var.encode()),
        }
    }
}

impl<'a> FormatInto<Swift> for &'a Type {
    fn format_into(self, t: &mut swift::Tokens) {
        match self {
            Type::Primitive { primitive } => {
                t.append(*primitive);
            }
            Type::String => {
                quote_in!(*t => String);
            }
            Type::DateTime { .. } => {
                quote_in!(*t => Date);
            }
            Type::Bytes { data } => {
                quote_in!(*t => $(&**data));
            }
            Type::Array { argument } => {
                quote_in!(*t => [$(&**argument)]);
            }
            Type::Dictionary { key, value } => {
                quote_in!(*t => [$(&**key): $(&**value)]);
            }
            Type::Local { ident } => {
                t.append(ident);
            }
            Type::Name { name } => {
                quote_in!(*t => $name);
            }
            Type::Any => {
                quote_in!(*t => Any);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Name {
    pub(crate) name: Rc<String>,
    pub(crate) package: RpPackage,
}

impl fmt::Display for Name {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(fmt)
    }
}

impl<'a> FormatInto<Swift> for &'a Name {
    fn format_into(self, tokens: &mut swift::Tokens) {
        tokens.append(self.name.clone())
    }
}

impl package_processor::Name<SwiftFlavor> for Name {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Field {
    inner: RpField,
}

impl Field {
    pub(crate) fn field_type(&self) -> impl FormatInto<Swift> + '_ {
        quote_fn! {
            $(if self.inner.is_optional() {
                $(&self.inner.ty)?
            } else {
                $(&self.inner.ty)
            })
        }
    }
}

impl FlavorField for Field {
    fn is_discriminating(&self) -> bool {
        self.inner.is_discriminating()
    }
}

impl Deref for Field {
    type Target = RpField;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SwiftFlavor {}

impl Flavor for SwiftFlavor {
    type Type = Type;
    type Name = Name;
    type Field = Field;
    type Endpoint = RpEndpoint;
    type Package = RpPackage;
    type EnumType = Type;
}

/// Responsible for translating RpType -> Swift type.
pub(crate) struct SwiftFlavorTranslator {
    packages: Rc<Packages>,
    formatter: Rc<swift::Import>,
    data: Rc<swift::Import>,
    date: Rc<swift::Import>,
    any: Type,
    to_upper_camel: naming::ToUpperCamel,
}

impl SwiftFlavorTranslator {
    pub(crate) fn new(packages: Rc<Packages>, options: &Options) -> Result<Self> {
        let any = {
            let mut any_types = options.any_type.iter().cloned();

            if let Some((first_mod, any_type)) = any_types.next() {
                if let Some((second_mod, _)) = any_types.next() {
                    return Err(format!(
                        "Any type provided by more than one module: {}, {}",
                        first_mod, second_mod
                    )
                    .into());
                }

                any_type.clone().into()
            } else {
                Type::Any
            }
        };

        Ok(Self {
            packages,
            formatter: Rc::new(swift::import("Foundation", "ISO8601DateFormatter")),
            data: Rc::new(swift::import("Foundation", "Data")),
            date: Rc::new(swift::import("Foundation", "Date")),
            any,
            to_upper_camel: naming::to_upper_camel(),
        })
    }
}

impl FlavorTranslator for SwiftFlavorTranslator {
    type Source = CoreFlavor;
    type Target = SwiftFlavor;

    reproto_core::translator_defaults!(Self, endpoint);

    fn translate_field<T>(
        &self,
        translator: &T,
        diag: &mut reproto_core::Diagnostics,
        field: RpField<Self::Source>,
    ) -> Result<Field>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let inner = field.translate(diag, translator)?;

        Ok(Field { inner })
    }

    fn translate_number(&self, number: RpNumberType) -> Result<Type> {
        Ok(match number.kind {
            RpNumberKind::U32 => Type::Primitive {
                primitive: Primitive::UInt32,
            },
            RpNumberKind::U64 => Type::Primitive {
                primitive: Primitive::UInt64,
            },
            RpNumberKind::I32 => Type::Primitive {
                primitive: Primitive::Int32,
            },
            RpNumberKind::I64 => Type::Primitive {
                primitive: Primitive::Int64,
            },
            ty => return Err(format!("unsupported number type: {}", ty).into()),
        })
    }

    fn translate_float(&self) -> Result<Type> {
        Ok(Type::Primitive {
            primitive: Primitive::Float,
        })
    }

    fn translate_double(&self) -> Result<Type> {
        Ok(Type::Primitive {
            primitive: Primitive::Double,
        })
    }

    fn translate_boolean(&self) -> Result<Type> {
        Ok(Type::Primitive {
            primitive: Primitive::Bool,
        })
    }

    fn translate_string(&self, _: RpStringType) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_datetime(&self) -> Result<Type> {
        Ok(Type::DateTime {
            formatter: self.formatter.clone(),
        })
    }

    fn translate_array(&self, argument: Type) -> Result<Type> {
        Ok(Type::array(argument))
    }

    fn translate_map(&self, key: Type, value: Type) -> Result<Type> {
        Ok(Type::map(key, value))
    }

    fn translate_any(&self) -> Result<Type> {
        Ok(self.any.clone())
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::Bytes {
            data: self.data.clone(),
        })
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let package_name = name.package.join("_");
        Ok(Type::local(format!("{}_{}", package_name, ident)))
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.packages.translate_package(source)
    }

    fn translate_local_name<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: Spanned<RpName<CoreFlavor>>,
    ) -> Result<Name>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let name = name.translate(diag, translator)?;
        let (name, _) = Spanned::take_pair(name);

        let package_name = name.package.join("_");
        let ident = reg.ident(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let ident = format!("{}_{}", package_name, ident);

        Ok(Name {
            name: Rc::new(ident),
            package: name.package,
        })
    }

    fn translate_enum_type<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: RpEnumType,
    ) -> Result<Type>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        match enum_type {
            RpEnumType::String(string) => self.translate_string(string),
            RpEnumType::Number(number) => self.translate_number(number),
        }
    }
}

reproto_core::decl_flavor!(pub(crate) SwiftFlavor);
