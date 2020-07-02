//! Java flavor.

#![allow(unused)]

use backend::package_processor;
use core::errors::Result;
use core::{
    CoreFlavor, Diagnostics, Flavor, FlavorField, FlavorTranslator, PackageTranslator,
    RpNumberKind, RpNumberType, RpStringType, Spanned, Translate, Translator,
};
use genco::prelude::*;
use genco::tokens::{from_fn, FormatInto};
use naming::Naming;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone)]
pub(crate) struct JavaHttp {
    pub(crate) request: java::Import,
    pub(crate) response: java::Import,
    pub(crate) path: RpPathSpec,
    pub(crate) method: RpHttpMethod,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Name {
    pub(crate) name: Rc<String>,
    pub(crate) package: RpPackage,
}

impl package_processor::Name<JavaFlavor> for Name {
    fn package(&self) -> &RpPackage {
        &self.package
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Primitive {
    Boolean,
    Integer,
    Long,
    Float,
    Double,
}

impl Primitive {
    /// Use the appropriate toString implementation.
    pub(crate) fn to_string(self, t: &mut java::Tokens, arg: impl FormatInto<Java>) {
        quote_in! {*t =>
            #(match self {
                Self::Boolean => Boolean.toString(#arg),
                Self::Integer => Integer.toString(#arg),
                Self::Long => Long.toString(#arg),
                Self::Float => Float.toString(#arg),
                Self::Double => Double.toString(#arg),
            })
        }
    }

    /// Use the appropriate hashCode implementation.
    pub(crate) fn hash_code(self, t: &mut java::Tokens, arg: impl FormatInto<Java>) {
        quote_in! {*t =>
            #(match self {
                Self::Boolean => Boolean.valueOf(#arg).hashCode(),
                Self::Integer => Integer.valueOf(#arg).hashCode(),
                Self::Long => Long.valueOf(#arg).hashCode(),
                Self::Float => Float.valueOf(#arg).hashCode(),
                Self::Double => Double.valueOf(#arg).hashCode(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Type {
    Object,
    Primitive {
        primitive: Primitive,
    },
    Boxed {
        primitive: Primitive,
    },
    String,
    DateTime {
        import: Rc<java::Import>,
    },
    Import {
        import: Rc<java::Import>,
    },
    List {
        list: Rc<java::Import>,
        argument: Box<Type>,
    },
    Map {
        map: Rc<java::Import>,
        key: Box<Type>,
        value: Box<Type>,
    },
}

impl Type {
    /// Convert into the underlying primitive type if appropriate.
    pub(crate) fn as_primitive(&self) -> Option<Primitive> {
        match self {
            Self::Primitive { primitive } | Self::Boxed { primitive } => Some(*primitive),
            _ => None,
        }
    }

    /// Test if type is primitive.
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Primitive { .. } => true,
            _ => false,
        }
    }

    /// Treat as an argument.
    pub(crate) fn optional_type<'a>(
        &'a self,
        optional: &'a Rc<java::Import>,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            #(&**optional)<#(match self {
                Type::Primitive { primitive } => #(Type::Boxed {
                    primitive: *primitive
                }),
                other => #(other),
            })>
        }
    }

    /// Generate an equals function for this type.
    pub(crate) fn equals(&self, a: java::Tokens, b: java::Tokens) -> impl FormatInto<Java> + '_ {
        from_fn(move |t| match self {
            Type::Primitive { primitive } | Type::Boxed { primitive } => quote_in!(*t => #a == #b),
            other => quote_in!(*t => #a.equals(#b)),
        })
    }

    /// Convert the type into a boxed type.
    fn into_boxed(self) -> Self {
        match self {
            Self::Primitive { primitive } => Self::Boxed { primitive },
            other => other,
        }
    }
}

impl<'a> FormatInto<Java> for &'a Type {
    fn format_into(self, t: &mut java::Tokens) {
        self.clone().format_into(t);
    }
}

impl FormatInto<Java> for Type {
    fn format_into(self, t: &mut java::Tokens) {
        match self {
            Type::Object => quote_in!(*t => Object),
            Type::Primitive { primitive } => match primitive {
                Primitive::Boolean => quote_in!(*t => boolean),
                Primitive::Integer => quote_in!(*t => int),
                Primitive::Long => quote_in!(*t => long),
                Primitive::Float => quote_in!(*t => float),
                Primitive::Double => quote_in!(*t => double),
            },
            Type::Boxed { primitive } => match primitive {
                Primitive::Boolean => quote_in!(*t => Boolean),
                Primitive::Integer => quote_in!(*t => Integer),
                Primitive::Long => quote_in!(*t => Long),
                Primitive::Float => quote_in!(*t => Float),
                Primitive::Double => quote_in!(*t => Double),
            },
            Type::String => quote_in!(*t => String),
            Type::Import { import } => quote_in!(*t => #(&*import)),
            Type::DateTime { import } => quote_in!(*t => #(&*import)),
            Type::List { list, argument } => {
                quote_in!(*t => #(&*list)<#(&*argument)>);
            }
            Type::Map { map, key, value } => {
                quote_in!(*t => #(&*map)<#(&*key), #(&*value)>);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Field {
    optional: Rc<java::Import>,
    inner: RpField,
}

impl Field {
    pub(crate) fn field_type(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            #(if self.is_optional() {
                #(self.ty.optional_type(&self.optional))
            } else {
                #(&self.ty)
            })
        }
    }

    pub(crate) fn optional_type(&self) -> impl FormatInto<Java> + '_ {
        self.ty.optional_type(&self.optional)
    }

    pub(crate) fn to_string(
        &self,
        arg: impl FormatInto<Java> + 'static,
    ) -> impl FormatInto<Java> + '_ {
        from_fn(move |t| {
            if self.is_optional() {
                return quote_in!(*t => #arg.toString());
            }

            match &self.ty {
                Type::Primitive { primitive } | Type::Boxed { primitive } => {
                    primitive.to_string(t, arg)
                }
                other => quote_in!(*t => #arg.toString()),
            }
        })
    }

    pub(crate) fn hash_code(
        &self,
        arg: impl FormatInto<Java> + 'static,
    ) -> impl FormatInto<Java> + '_ {
        from_fn(move |t| {
            if self.is_optional() {
                return quote_in!(*t => #arg.hashCode());
            }

            match &self.ty {
                Type::Primitive { primitive } | Type::Boxed { primitive } => {
                    primitive.hash_code(t, arg)
                }
                other => quote_in!(*t => #arg.hashCode()),
            }
        })
    }

    pub(crate) fn not_equals(
        &self,
        a: java::Tokens,
        b: java::Tokens,
    ) -> impl FormatInto<Java> + '_ {
        from_fn(move |t| {
            if self.is_optional() {
                return quote_in!(*t => !#a.equals(#b));
            }

            match &self.ty {
                Type::Primitive { primitive } | Type::Boxed { primitive } => {
                    quote_in!(*t => #a != #b)
                }
                other => quote_in!(*t => !#a.equals(#b)),
            }
        })
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
pub(crate) enum JavaFlavor {}

impl Flavor for JavaFlavor {
    type Type = Type;
    type Name = Name;
    type Field = Field;
    type Endpoint = RpEndpoint;
    type Package = core::RpPackage;
    type EnumType = Type;
}

/// Responsible for translating RpType -> Java type.
pub(crate) struct JavaFlavorTranslator {
    packages: Rc<Packages>,
    list: Rc<java::Import>,
    map: Rc<java::Import>,
    instant: Rc<java::Import>,
    byte_buffer: Rc<java::Import>,
    optional: Rc<java::Import>,
    to_upper_camel: naming::ToUpperCamel,
    to_lower_camel: naming::ToLowerCamel,
}

impl JavaFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        Self {
            packages,
            list: Rc::new(java::import("java.util", "List")),
            map: Rc::new(java::import("java.util", "Map")),
            instant: Rc::new(java::import("java.time", "Instant")),
            byte_buffer: Rc::new(java::import("java.nio", "ByteBuffer")),
            optional: Rc::new(java::import("java.util", "Optional")),
            to_upper_camel: naming::to_upper_camel(),
            to_lower_camel: naming::to_lower_camel(),
        }
    }
}

impl FlavorTranslator for JavaFlavorTranslator {
    type Source = CoreFlavor;
    type Target = JavaFlavor;

    core::translator_defaults!(Self, endpoint);

    fn translate_field<T>(
        &self,
        translator: &T,
        diag: &mut core::Diagnostics,
        field: core::RpField<Self::Source>,
    ) -> Result<Field>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let inner = field.translate(diag, translator)?;

        Ok(Field {
            optional: self.optional.clone(),
            inner,
        })
    }

    fn translate_number(&self, number: RpNumberType) -> Result<Type> {
        let primitive = match number.kind {
            RpNumberKind::U32 | RpNumberKind::I32 => Primitive::Integer,
            RpNumberKind::U64 | RpNumberKind::I64 => Primitive::Long,
            ty => return Err(format!("unsupported number type: {}", ty).into()),
        };

        Ok(Type::Primitive { primitive })
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
            primitive: Primitive::Boolean,
        })
    }

    fn translate_string(&self, _: RpStringType) -> Result<Type> {
        Ok(Type::String)
    }

    fn translate_datetime(&self) -> Result<Type> {
        Ok(Type::DateTime {
            import: self.instant.clone(),
        })
    }

    fn translate_array(&self, argument: Type) -> Result<Type> {
        Ok(Type::List {
            list: self.list.clone(),
            argument: Box::new(argument.into_boxed()),
        })
    }

    fn translate_map(&self, key: Type, value: Type) -> Result<Type> {
        Ok(Type::Map {
            map: self.map.clone(),
            key: Box::new(key.into_boxed()),
            value: Box::new(value.into_boxed()),
        })
    }

    fn translate_any(&self) -> Result<Type> {
        Ok(Type::Object)
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::Import {
            import: self.byte_buffer.clone(),
        })
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let ident = Rc::new(reg.ident(&name, |p| p.join("."), |c| c.join(".")));
        let package = name.package.join(".");

        Ok(Type::Import {
            import: Rc::new(java::import(package, ident)),
        })
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.packages.translate_package(source)
    }

    fn translate_local_name<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: Spanned<core::RpName<CoreFlavor>>,
    ) -> Result<Name>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let (name, span) = Spanned::take_pair(name);

        let ident = Rc::new(reg.ident(&name, |p| p.join("."), |c| c.join(".")));
        let package = self.translate_package(name.package)?;

        Ok(Name {
            name: ident,
            package,
        })
    }

    fn translate_enum_type<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: core::RpEnumType,
    ) -> Result<Type>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        match enum_type {
            core::RpEnumType::String(string) => self.translate_string(string),
            core::RpEnumType::Number(number) => self.translate_number(number),
        }
    }
}

core::decl_flavor!(pub(crate) JavaFlavor);
