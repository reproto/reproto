//! C# flavor.

#![allow(unused)]

use core::errors::Result;
use core::{
    CoreFlavor, Diagnostics, Flavor, FlavorField, FlavorTranslator, PackageTranslator,
    RpNumberKind, RpNumberType, RpNumberValidate, RpStringType, Spanned, Translate, Translator,
};
use genco::prelude::*;
use genco::tokens::from_fn;
use naming::Naming;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use trans::Packages;

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Int,
    Long,
    UInt,
    ULong,
    Float,
    Double,
    Bool,
}

impl FormatInto<Csharp> for Primitive {
    fn format_into(self, t: &mut csharp::Tokens) {
        match self {
            Self::Int => quote_in!(*t => int),
            Self::Long => quote_in!(*t => long),
            Self::UInt => quote_in!(*t => uint),
            Self::ULong => quote_in!(*t => ulong),
            Self::Float => quote_in!(*t => float),
            Self::Double => quote_in!(*t => double),
            Self::Bool => quote_in!(*t => bool),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(Primitive),
    ByteArray,
    String {
        import: Rc<csharp::Import>,
    },
    DateTime {
        import: Rc<csharp::Import>,
    },
    Object {
        import: Rc<csharp::Import>,
    },
    Enum {
        import: Rc<csharp::Import>,
    },
    Import {
        import: csharp::Import,
    },
    Dictionary {
        import: Rc<csharp::Import>,
        key: Box<Type>,
        value: Box<Type>,
    },
    List {
        import: Rc<csharp::Import>,
        argument: Box<Type>,
    },
}

impl Type {
    pub fn dictionary<K, V>(import: Rc<csharp::Import>, key: K, value: V) -> Self
    where
        K: Into<Type>,
        V: Into<Type>,
    {
        Self::Dictionary {
            import,
            key: Box::new(key.into()),
            value: Box::new(value.into()),
        }
    }

    pub fn list<A>(import: Rc<csharp::Import>, argument: A) -> Self
    where
        A: Into<Type>,
    {
        Self::List {
            import,
            argument: Box::new(argument.into()),
        }
    }

    /// Check if the given type is nullable.
    pub(crate) fn is_nullable(&self) -> bool {
        match self {
            Self::Primitive { .. } => false,
            Self::Enum { .. } => false,
            Self::DateTime { .. } => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EnumType {
    String,
    Int,
    Long,
}

impl<'a> FormatInto<Csharp> for &'a Type {
    fn format_into(self, t: &mut csharp::Tokens) {
        match self {
            Type::Primitive(p) => quote_in!(*t => #(*p)),
            Type::ByteArray => quote_in!(*t => byte[]),
            Type::String { import }
            | Type::DateTime { import }
            | Type::Object { import }
            | Type::Enum { import } => {
                t.append(&**import);
            }
            Type::Import { import } => {
                t.append(import);
            }
            Type::Dictionary { import, key, value } => {
                quote_in!(*t => #(&**import)<#(&**key), #(&**value)>)
            }
            Type::List { import, argument } => quote_in!(*t => #(&**import)<#(&**argument)>),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    Enum { import: csharp::Import },
    Import { import: csharp::Import },
}

#[derive(Debug, Clone)]
pub(crate) struct Field {
    pub(crate) var: Rc<String>,
    inner: RpField,
}

impl Field {
    /// Resolve the type of the field.
    pub(crate) fn field_type(&self) -> impl FormatInto<Csharp> + '_ {
        quote_fn! {
            #(if self.is_optional() && !self.ty.is_nullable() {
                #(&self.ty)?
            } else {
                #(&self.ty)
            })
        }
    }
}

impl Deref for Field {
    type Target = RpField;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FlavorField for Field {
    fn is_discriminating(&self) -> bool {
        self.inner.is_discriminating()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CsharpFlavor {}

impl Flavor for CsharpFlavor {
    type Type = Type;
    type Name = Spanned<RpName>;
    type Field = Field;
    type Endpoint = RpEndpoint;
    type Package = core::RpPackage;
    type EnumType = EnumType;
}

/// Responsible for translating RpType -> Csharp type.
pub(crate) struct CsharpFlavorTranslator {
    packages: Rc<Packages>,
    object: Rc<csharp::Import>,
    string: Rc<csharp::Import>,
    date_time: Rc<csharp::Import>,
    list: Rc<csharp::Import>,
    dictionary: Rc<csharp::Import>,
    to_lower_camel: naming::ToLowerCamel,
    to_upper_camel: naming::ToUpperCamel,
}

impl CsharpFlavorTranslator {
    pub fn new(packages: Rc<Packages>) -> Self {
        Self {
            packages,
            object: Rc::new(csharp::import("System", "Object")),
            string: Rc::new(csharp::import("System", "String")),
            date_time: Rc::new(csharp::import("System", "DateTime")),
            list: Rc::new(csharp::import("System.Collections.Generic", "List")),
            dictionary: Rc::new(csharp::import("System.Collections.Generic", "Dictionary")),
            to_lower_camel: naming::to_lower_camel(),
            to_upper_camel: naming::to_upper_camel(),
        }
    }
}

impl FlavorTranslator for CsharpFlavorTranslator {
    type Source = CoreFlavor;
    type Target = CsharpFlavor;

    core::translator_defaults!(Self, local_name, endpoint);

    fn translate_field<T>(
        &self,
        translator: &T,
        diag: &mut core::Diagnostics,
        field: core::RpField<Self::Source>,
    ) -> Result<Field>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        let var = Rc::new(self.to_lower_camel.convert(field.safe_ident()));
        let inner = field.translate(diag, translator)?;

        Ok(Field { var, inner })
    }

    fn translate_number(&self, number: RpNumberType) -> Result<Type> {
        let p = match number.kind {
            RpNumberKind::I32 => Primitive::Int,
            RpNumberKind::I64 => Primitive::Long,
            RpNumberKind::U32 => Primitive::UInt,
            RpNumberKind::U64 => Primitive::ULong,
        };

        Ok(Type::Primitive(p))
    }

    fn translate_float(&self) -> Result<Type> {
        Ok(Type::Primitive(Primitive::Float))
    }

    fn translate_double(&self) -> Result<Type> {
        Ok(Type::Primitive(Primitive::Double))
    }

    fn translate_boolean(&self) -> Result<Type> {
        Ok(Type::Primitive(Primitive::Bool))
    }

    fn translate_string(&self, _: RpStringType) -> Result<Type> {
        Ok(Type::String {
            import: self.string.clone(),
        })
    }

    fn translate_datetime(&self) -> Result<Type> {
        Ok(Type::DateTime {
            import: self.date_time.clone(),
        })
    }

    fn translate_array(&self, argument: Type) -> Result<Type> {
        Ok(Type::list(self.list.clone(), argument))
    }

    fn translate_map(&self, key: Type, value: Type) -> Result<Type> {
        Ok(Type::dictionary(self.dictionary.clone(), key, value))
    }

    fn translate_any(&self) -> Result<Type> {
        Ok(Type::Object {
            import: self.object.clone(),
        })
    }

    fn translate_bytes(&self) -> Result<Type> {
        Ok(Type::ByteArray)
    }

    fn translate_name(&self, _from: &RpPackage, reg: RpReg, name: Spanned<RpName>) -> Result<Type> {
        let package_name = Rc::new(name.package.join("."));
        let name = Rc::new(reg.ident(&name, |p| p.join("."), |c| c.join(".")));

        let import = csharp::import(package_name, name);

        if reg.is_enum() {
            return Ok(Type::Enum {
                import: Rc::new(import),
            });
        } else {
            return Ok(Type::Import { import });
        }
    }

    fn translate_package(&self, source: RpVersionedPackage) -> Result<RpPackage> {
        self.packages.translate_package(source)
    }

    fn translate_enum_type<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: core::RpEnumType,
    ) -> Result<EnumType>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>,
    {
        Ok(match enum_type {
            core::RpEnumType::String(string) => EnumType::String,
            core::RpEnumType::Number(number) => match number.kind {
                RpNumberKind::U32 => EnumType::Int,
                RpNumberKind::I32 => EnumType::Int,
                RpNumberKind::U64 => EnumType::Long,
                RpNumberKind::I64 => EnumType::Long,
            },
        })
    }
}

core::decl_flavor!(pub(crate) CsharpFlavor);
