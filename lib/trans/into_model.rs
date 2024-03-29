//! IntoModel is the trait that performs AST to RpIR translation.

use crate::attributes;
use crate::scope::Scope;
use ast::*;
use linked_hash_map::LinkedHashMap;
use naming::Naming;
use reproto_core::errors::Error;
use reproto_core::flavored::*;
use reproto_core::{
    BigInt, Diagnostics, Import, Range, RpEnabledFeature, RpNumberKind, RpNumberType,
    RpNumberValidate, RpStringType, RpStringValidate, Span, Spanned, SymbolKind, WithSpan,
};
use std::borrow::Cow;
use std::collections::{hash_map, BTreeSet, HashMap};
use std::option;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;

/// All error information is propagated to the diagnostics argument, but we signal that an error
/// occurred by returning Err(()).
pub type Result<T> = result::Result<T, ()>;

/// Helper macro to deal with a unit error in a loop.
///
/// This assumes that it's being called in a loop, and will continue on errors.
/// NOTE: it is critical that `diag.has_errors()` is checked _after_ the loop.
#[macro_export]
macro_rules! try_loop {
    ($e:expr) => {
        match $e {
            Err(()) => continue,
            Ok(ok) => ok,
        }
    };
}

/// Check for conflicting items and generate appropriate error messages if they are.
/// This assumes that it's being called in a loop, and will continue on errors.
/// NOTE: it is critical that `diag.has_errors()` is checked _after_ the loop.
macro_rules! check_conflict {
    ($diag:expr, $existing:expr, $item:expr, $accessor:expr, $what:expr) => {
        if let Some(other) = $existing.insert($accessor.to_string(), Span::from(&$item).clone()) {
            $diag.err(
                Span::from(&$item),
                format!(concat!($what, " `{}` is already defined"), $accessor),
            );

            $diag.info(other, "previously defined here");
            continue;
        }
    };
}

/// Checks if a given field matches a sub-type tag.
/// This assumes that it's being called in a loop, and will continue on errors.
/// NOTE: it is critical that `diag.has_errors()` is checked _after_ the loop.
macro_rules! check_field_tag {
    ($diag:ident, $field:expr, $strategy:expr) => {
        match $strategy {
            RpSubTypeStrategy::Tagged { ref tag, .. } => {
                if $field.name() == tag {
                    $diag.err(
                        &$field.span(),
                        format!(
                            "field with name `{}` is the same as tag used in type_info",
                            tag
                        ),
                    );

                    continue;
                }
            }
            _ => {}
        }
    };
}

/// Check if matching a reserved field.
/// This assumes that it's being called in a loop, and will continue on errors.
/// NOTE: it is critical that `diag.has_errors()` is checked _after_ the loop.
macro_rules! check_field_reserved {
    ($diag:ident, $field:expr, $reserved:expr) => {
        if let Some(reserved) = $reserved.get($field.name()) {
            $diag.err(
                &$field.span(),
                format!("field with name `{}` is reserved", $field.name()),
            );

            $diag.info(reserved, "reserved here");
            continue;
        }
    };
}

#[derive(Debug, Default)]
pub struct MemberConstraint<'input> {
    sub_type_strategy: Option<&'input RpSubTypeStrategy>,
    reserved: Option<&'input HashMap<String, Span>>,
}

#[derive(Debug)]
pub struct SubTypeConstraint<'input> {
    sub_type_strategy: &'input RpSubTypeStrategy,
    reserved: &'input HashMap<String, Span>,
    field_idents: &'input HashMap<String, Span>,
    field_names: &'input HashMap<String, Span>,
    untagged: &'input mut LinkedHashMap<BTreeSet<String>, Span>,
}

#[derive(Debug)]
pub struct Members {
    fields: Vec<Spanned<RpField>>,
    codes: Vec<Spanned<RpCode>>,
    decls: Vec<RpDecl>,
    decl_idents: LinkedHashMap<String, usize>,
    field_names: HashMap<String, Span>,
    field_idents: HashMap<String, Span>,
}

/// Adds a method for all types that supports conversion into core types.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import;
}

/// Generic implementation for vectors.
impl<T> IntoModel for Spanned<T>
where
    T: IntoModel,
{
    type Output = Spanned<T::Output>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let (value, span) = Spanned::take_pair(self);
        Ok(Spanned::new(value.into_model(diag, scope)?, span))
    }
}

/// Generic implementation for vectors.
impl<T> IntoModel for Vec<T>
where
    T: IntoModel,
{
    type Output = Vec<T::Output>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let mut out = Vec::new();

        for v in self {
            out.push(v.into_model(diag, scope)?);
        }

        Ok(out)
    }
}

impl<T> IntoModel for Option<T>
where
    T: IntoModel,
{
    type Output = Option<T::Output>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        match self {
            Some(value) => Ok(Some(value.into_model(diag, scope)?)),
            None => Ok(None),
        }
    }
}

impl<T> IntoModel for Box<T>
where
    T: IntoModel,
{
    type Output = Box<T::Output>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        Ok(Box::new((*self).into_model(diag, scope)?))
    }
}

impl<'a> IntoModel for Cow<'a, str> {
    type Output = String;

    fn into_model<I>(self, _: &mut Diagnostics, _scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        Ok(self.to_string())
    }
}

impl IntoModel for String {
    type Output = String;

    fn into_model<I>(self, _: &mut Diagnostics, _scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        Ok(self)
    }
}

/// Helper model to strip whitespace prefixes from comment lines.
pub struct Comment<I>(I);

impl<C: IntoIterator<Item = S>, S: AsRef<str>> IntoModel for Comment<C> {
    type Output = Vec<String>;

    fn into_model<I>(self, _: &mut Diagnostics, _scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let comment = self.0.into_iter().collect::<Vec<_>>();

        let pfx = comment
            .iter()
            .flat_map(|s| s.as_ref().find(|c: char| !c.is_whitespace()))
            .min()
            .unwrap_or(0);

        let comment: Vec<String> = comment
            .into_iter()
            .map(|s| {
                let s = s.as_ref();
                s[usize::min(s.len(), pfx)..].to_string()
            })
            .collect();

        Ok(comment)
    }
}

impl<'input> IntoModel for Spanned<Type<'input>> {
    type Output = RpType;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        (None, self).into_model(diag, scope)
    }
}

impl<'input> IntoModel for (Option<&'input mut Attributes>, Spanned<Type<'input>>) {
    type Output = RpType;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::Type::*;

        let (attributes, ty) = self;
        let (ty, span) = Spanned::take_pair(ty);

        let out = match ty {
            Double => RpType::Double,
            Float => RpType::Float,
            Unsigned { size: 32 } => RpType::Number(RpNumberType {
                kind: RpNumberKind::U32,
                validate: RpNumberValidate::default(),
            }),
            Unsigned { size: 64 } => RpType::Number(RpNumberType {
                kind: RpNumberKind::U64,
                validate: RpNumberValidate::default(),
            }),
            Signed { size: 32 } => RpType::Number(RpNumberType {
                kind: RpNumberKind::I32,
                validate: RpNumberValidate::default(),
            }),
            Signed { size: 64 } => RpType::Number(RpNumberType {
                kind: RpNumberKind::I64,
                validate: RpNumberValidate::default(),
            }),
            Boolean => RpType::Boolean,
            String => {
                let validate = match attributes {
                    Some(attributes) => attributes::string_validate(diag, attributes)?,
                    None => RpStringValidate::default(),
                };

                RpType::String(RpStringType { validate })
            }
            DateTime => RpType::DateTime,
            Name { name } => RpType::Name {
                name: name.into_model(diag, scope)?,
            },
            Array { inner } => RpType::Array {
                inner: inner.into_model(diag, scope)?,
            },
            Map { key, value } => RpType::Map {
                key: key.into_model(diag, scope)?,
                value: value.into_model(diag, scope)?,
            },
            Any => RpType::Any,
            Bytes => RpType::Bytes,
            Error { .. } => {
                diag.err(span, "expected type, like: `string`, `u32`, or `MyType`");
                return Err(());
            }
            _ => {
                diag.err(span, "unsupported type");
                return Err(());
            }
        };

        Ok(out)
    }
}

impl<'input> IntoModel for Decl<'input> {
    type Output = RpDecl;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::Decl::*;

        scope.push(Spanned::take(self.name()));

        let out = match self {
            Type(body) => body.into_model(diag, scope).map(RpDecl::Type),
            Interface(body) => body.into_model(diag, scope).map(RpDecl::Interface),
            Enum(body) => body.into_model(diag, scope).map(RpDecl::Enum),
            Tuple(body) => body.into_model(diag, scope).map(RpDecl::Tuple),
            Service(body) => body.into_model(diag, scope).map(RpDecl::Service),
        };

        scope.pop();

        out
    }
}

impl<'input> IntoModel for Item<'input, EnumBody<'input>> {
    type Output = Spanned<RpEnumBody>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        macro_rules! variants {
            (
                $diag:expr, $enum_type:expr, $variants:expr, $type_field:ident,
                $(($ty:ident, $out:ident, $default:ident)),*
            ) => {
            match $enum_type {
                $(
                RpEnumType::$ty(ref $type_field) => {
                    let mut out = Vec::new();

                    let mut idents = HashMap::new();
                    let mut values = HashMap::new();
                    let mut default = $default::new($type_field);

                    for v in $variants {
                        let v = try_loop!((v, &mut default).into_model(diag, scope));

                        check_conflict!($diag, idents, v, v.ident, "variant");
                        check_conflict!($diag, values, v, v.value(), "variant value");

                        out.push(v);
                    }

                    if diag.has_errors() {
                        return Err(());
                    }

                    RpVariants::$out { variants: out }
                }
                )*
            }
            };
        }

        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        let name = scope.as_name(item.name.span());

        diag.symbol(SymbolKind::Enum, &span, &name);

        let mut codes = Vec::new();

        for member in item.members {
            match member {
                EnumMember::Code(code) => {
                    codes.push(code.into_model(diag, scope)?);
                }
            };
        }

        let enum_type = {
            let span = &item.ty.span();
            let enum_type = item.ty.into_model(diag, scope)?;

            match enum_type.as_enum_type() {
                Some(enum_type) => enum_type,
                None => {
                    diag.err(
                        span,
                        "illegal enum type, expected `string`, `u32`, `u64`, `i32`, or `i64`",
                    );
                    return Err(());
                }
            }
        };

        let variants = variants!(
            diag,
            enum_type,
            item.variants,
            ty,
            (String, String, StringDefaultVariant),
            (Number, Number, NumberDefaultVariant)
        );

        let attributes = attributes.into_model(diag, scope)?;
        check_attributes!(diag, attributes);

        return Ok(Spanned::new(
            RpEnumBody {
                name,
                ident: item.name.to_string(),
                comment: Comment(&comment).into_model(diag, scope)?,
                decls: vec![],
                decl_idents: LinkedHashMap::new(),
                enum_type: enum_type,
                variants: variants,
                codes: codes,
            },
            span,
        ));

        struct NumberDefaultVariant<'a> {
            state: BigInt,
            number_type: &'a RpNumberType,
        }

        impl<'a> NumberDefaultVariant<'a> {
            fn new(number_type: &'a RpNumberType) -> Self {
                Self {
                    state: 0.into(),
                    number_type,
                }
            }
        }

        impl<'a> DefaultVariant for NumberDefaultVariant<'a> {
            type Type = RpNumber;

            fn next<'input>(&mut self, _: &EnumVariant<'input>) -> result::Result<RpNumber, Error> {
                let next = self.state.clone();
                self.state = self.state.clone() + BigInt::from(1);
                let number = RpNumber::from(next);
                self.number_type.validate_number(&number)?;
                Ok(number)
            }

            fn process(&mut self, value: RpValue) -> result::Result<RpNumber, Error> {
                let number = value.into_number()?;

                {
                    let value = number
                        .to_bigint()
                        .ok_or_else(|| "value can't be used with generator")?;

                    self.state = value.clone();
                }

                self.number_type.validate_number(&number)?;
                Ok(number)
            }
        }

        struct StringDefaultVariant;

        impl StringDefaultVariant {
            pub fn new<'a>(_: &'a RpStringType) -> Self {
                StringDefaultVariant
            }
        }

        impl DefaultVariant for StringDefaultVariant {
            type Type = String;

            fn next<'input>(
                &mut self,
                variant: &EnumVariant<'input>,
            ) -> result::Result<String, Error> {
                Ok(variant.name.to_string())
            }

            fn process(&mut self, value: RpValue) -> result::Result<String, Error> {
                value
                    .as_string()
                    .map(|s| s.to_string())
                    .map_err(|_| format!("expected `string`, did you mean \"{}\"?", value).into())
            }
        }
    }
}

/// Type that generates a variant value.
pub trait DefaultVariant {
    type Type;

    /// Get the next default variant value.
    fn next<'input>(&mut self, variant: &EnumVariant<'input>) -> result::Result<Self::Type, Error>;

    /// Process the value, attempting to convert it to the destination type.
    fn process(&mut self, value: RpValue) -> result::Result<Self::Type, Error>;
}

/// enum value with assigned ordinal
impl<'input, 'a, D> IntoModel for (Item<'input, EnumVariant<'input>>, &'a mut D)
where
    D: DefaultVariant,
{
    type Output = Spanned<RpVariant<D::Type>>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let (variant, default) = self;

        let Item {
            comment,
            attributes,
            item,
        } = variant;

        let (item, span) = Spanned::take_pair(item);

        let name = Spanned::map(scope.as_name(item.name.span()), |n| {
            n.push(item.name.to_string())
        });

        let value = if let Some(argument) = item.argument {
            let (value, span) = Spanned::take_pair(argument.into_model(diag, scope)?);

            match default.process(value) {
                Err(e) => {
                    diag.err(span, e.display());
                    return Err(());
                }
                Ok(value) => value,
            }
        } else {
            default.next(&item).with_span(diag, span)?
        };

        let attributes = attributes.into_model(diag, scope)?;
        check_attributes!(diag, attributes);

        Ok(Spanned::new(
            RpVariant {
                name,
                ident: Spanned::map(item.name.clone(), |s| s.to_string()),
                comment: Comment(&comment).into_model(diag, scope)?,
                value: value,
            },
            span,
        ))
    }
}

/// Helper function to build a safe identifier.
fn build_safe_ident<I, N>(scope: &mut Scope<I>, ident: &str, naming: N) -> Option<String>
where
    I: Import,
    N: for<'a> FnOnce(&'a Scope<I>) -> Option<&'a dyn Naming>,
{
    if let Some(ident_naming) = naming(scope) {
        let converted = ident_naming.convert(ident);

        match scope.keyword(converted.as_str()) {
            Some(ident) => Some(ident.to_string()),
            None if converted.as_str() != ident => Some(converted),
            None => None,
        }
    } else {
        scope.keyword(ident).map(|s| s.to_string())
    }
}

/// Helper function to build a safe name.
fn build_item_name<I, A, B>(
    scope: &mut Scope<I>,
    ident: &str,
    name: Option<&str>,
    default_naming: A,
    default_ident_naming: B,
) -> (String, Option<String>, Option<String>)
where
    A: for<'a> FnOnce(&'a Scope<I>) -> Option<&'a dyn Naming>,
    B: for<'a> FnOnce(&'a Scope<I>) -> Option<&'a dyn Naming>,
    I: Import,
{
    let safe_ident = build_safe_ident(scope, ident, default_ident_naming);

    // Apply specification-wide naming convention unless field name explicitly specified.
    let name = name
        .map(|s| s.to_string())
        .or_else(|| default_naming(scope).map(|n| n.convert(ident)));

    // Don't include field alias if same as name.
    let name = match name {
        // Explicit alias, but it's exactly the same as translated field.
        Some(ref name) if name == ident => None,
        // Explicit alias that differs from field.
        Some(name) => Some(name),
        // Name matches ident
        _ => None,
    };

    (ident.to_string(), safe_ident, name)
}

impl<'input> IntoModel for Item<'input, Field<'input>> {
    type Output = Spanned<RpField>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        if !item.endl {
            diag.err(span.end(), "missing `;`");
        }

        let field_as = item.field_as.into_model(diag, scope)?;

        let (ident, safe_ident, field_as) = build_item_name(
            scope,
            item.name.as_ref(),
            field_as.as_ref().map(|s| s.as_str()),
            Scope::field_naming,
            Scope::field_ident_naming,
        );

        let mut attributes = attributes.into_model(diag, scope)?;

        let ty = handle_format_attribute(diag, scope, &mut attributes, item.ty)?;

        let ty = (Some(&mut attributes), ty).into_model(diag, scope)?;

        check_attributes!(diag, attributes);

        return Ok(Spanned::new(
            RpField {
                required: item.required,
                safe_ident,
                ident,
                comment: Comment(&comment).into_model(diag, scope)?,
                ty,
                field_as,
            },
            span,
        ));

        fn handle_format_attribute<'input, I>(
            diag: &mut Diagnostics,
            scope: &mut Scope<I>,
            attributes: &mut Attributes,
            ty: Spanned<Type<'input>>,
        ) -> Result<Spanned<Type<'input>>>
        where
            I: Import,
        {
            let format = attributes::string_format(diag, attributes)?;

            // TODO: convert String into a richer type instead of just punting it.
            let feature = match scope.feature("format_attribute") {
                None => {
                    // not allowed unless feature is active.
                    if let Some(span) = format.as_ref().map(Spanned::span) {
                        diag.err(span, "attribute not supported");
                        diag.info(span, "HINT: use #![feature(format_attribute)] to enable");
                        return Err(());
                    }

                    return Ok(ty);
                }
                Some(feature) => feature,
            };

            let (ty, span) = Spanned::take_pair(ty);

            // report error on types that should be declared using a format attribute.
            let ty = match ty {
                Type::Bytes => {
                    scope.feature_err(diag, feature, span, "type not supported");

                    diag.info(
                        span,
                        "HINT: use #[format(\"bytes\")] attribute on a `string` field instead",
                    );
                    return Err(());
                }
                Type::DateTime => {
                    scope.feature_err(diag, feature, span, "type not supported");

                    diag.info(
                        span,
                        "HINT: use #[format(\"datetime\")] attribute on a `string` field instead",
                    );
                    return Err(());
                }
                Type::String => {
                    if let Some(format) = format.map(Spanned::take) {
                        match format {
                            attributes::StringFormat::DateTime => Type::DateTime,
                            attributes::StringFormat::Bytes => Type::Bytes,
                        }
                    } else {
                        Type::String
                    }
                }
                ty => ty,
            };

            if diag.has_errors() {
                return Err(());
            }

            Ok(Spanned::new(ty, span))
        }
    }
}

/// Process use declarations found at the top of each object.
impl<'input> IntoModel for Vec<Spanned<UseDecl<'input>>> {
    type Output = HashMap<String, RpVersionedPackage>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use std::collections::hash_map::Entry;

        let mut prefixes = HashMap::new();

        for use_decl in self {
            let (use_decl, span) = Spanned::take_pair(use_decl);

            if use_decl.endl.is_none() {
                diag.err(span.end(), "missing `;`");
            }

            let range = {
                match use_decl.range {
                    Some(range) => {
                        let (range, span) = Spanned::take_pair(range);

                        match Range::parse(&range) {
                            Ok(range) => range,
                            Err(e) => {
                                diag.err(span, format!("bad version range: {}", e));
                                continue;
                            }
                        }
                    }
                    None => Range::any(),
                }
            };

            let (package, span) = Spanned::take_pair(use_decl.package);

            // Handle Error.
            let package = match package {
                Package::Package { parts } => {
                    RpPackage::new(parts.into_iter().map(|p| p.to_string()).collect())
                }
                Package::Error => {
                    diag.err(span, format!("not a valid package"));
                    continue;
                }
            };

            let required = RpRequiredPackage::new(package.clone(), range);
            let use_package = scope.import(&required).with_span(diag, span)?;

            if let Some(use_package) = use_package {
                if let Some(used) = package.parts().last() {
                    let (alias, span) = match use_decl.alias.as_ref() {
                        Some(alias) => {
                            let (alias, span) = Spanned::borrow_pair(alias);
                            (alias.as_ref(), span)
                        }
                        None => (used.as_str(), span),
                    };

                    match prefixes.entry(alias.to_string()) {
                        Entry::Vacant(entry) => entry.insert(use_package.clone()),
                        Entry::Occupied(_) => {
                            diag.err(span, format!("alias {} already in use", alias));
                            continue;
                        }
                    };
                }

                continue;
            }

            diag.err(
                span,
                format!("imported package `{}` does not exist", required),
            );
        }

        if diag.has_errors() {
            return Err(());
        }

        Ok(prefixes)
    }
}

impl<'input> IntoModel for File<'input> {
    type Output = RpFile;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let prefixes = self.uses.into_model(diag, scope)?;
        scope.prefixes = prefixes;

        let mut attributes = self.attributes.into_model(diag, scope)?;

        let reproto = attributes::reproto(diag, &mut attributes)?;
        scope.declared_version = reproto.version;

        let mut features = LinkedHashMap::new();
        let mut activated_features = HashMap::new();

        for feature in attributes::features(scope, diag, &mut attributes)? {
            let (feature, span) = Spanned::borrow_pair(&feature);

            if let Some(e) = features.insert(feature.name, RpEnabledFeature { span }) {
                diag.err(span, "feature already activated");
                diag.info(e.span, "already activated here");
                return Err(());
            }

            activated_features.insert(feature.name, span);
        }

        scope.activated_features = activated_features;

        if let Some(endpoint_naming) = attributes.take_selection("endpoint_naming") {
            let (mut endpoint_naming, span) = Spanned::take_pair(endpoint_naming);

            scope.endpoint_naming = endpoint_naming
                .take_word()
                .ok_or_else(|| Error::from("expected argument"))
                .and_then(|n| {
                    n.as_identifier()
                        .map_err(|_| Error::from("expected identifier"))
                        .and_then(parse_naming)
                })
                .with_span(diag, &span)?;

            check_selection!(diag, endpoint_naming);
        }

        if let Some(field_naming) = attributes.take_selection("field_naming") {
            let (mut field_naming, span) = Spanned::take_pair(field_naming);

            scope.field_naming = field_naming
                .take_word()
                .ok_or_else(|| Error::from("expected argument"))
                .and_then(|n| {
                    n.as_identifier()
                        .map_err(|_| Error::from("expected identifier"))
                        .and_then(parse_naming)
                })
                .with_span(diag, &span)?;

            check_selection!(diag, field_naming);
        }

        check_attributes!(diag, attributes);

        let mut decls = Vec::new();
        let mut decl_idents = LinkedHashMap::new();

        for (index, d) in self.decls.into_iter().enumerate() {
            let d = try_loop!(d.into_model(diag, scope));
            decl_idents.insert(d.ident().to_string(), index);
            decls.push(d);
        }

        if diag.has_errors() {
            return Err(());
        }

        return Ok(RpFile {
            comment: Comment(&self.comment).into_model(diag, scope)?,
            version: scope.version().clone(),
            features,
            decls,
            decl_idents,
        });

        /// Parse a naming option.
        ///
        /// Since lower_camel is default, do nothing on that case.
        fn parse_naming(naming: &str) -> result::Result<Option<Box<dyn Naming>>, Error> {
            let result: Option<Box<dyn Naming>> = match naming {
                "upper_camel" => Some(Box::new(naming::to_upper_camel())),
                "lower_camel" => Some(Box::new(naming::to_lower_camel())),
                "upper_snake" => Some(Box::new(naming::to_upper_snake())),
                "lower_snake" => None,
                _ => return Err("illegal value".into()),
            };

            Ok(result)
        }
    }
}

impl<'input> IntoModel for Item<'input, InterfaceBody<'input>> {
    type Output = Spanned<RpInterfaceBody>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        let name = scope.as_name(item.name.span());

        diag.symbol(SymbolKind::Interface, &span, &name);

        let mut attributes = attributes.into_model(diag, scope)?;

        let reserved = attributes::reserved(diag, &mut attributes)?;

        let mut sub_type_strategy = RpSubTypeStrategy::default();

        if let Some(mut type_info) = attributes.take_selection("type_info") {
            sub_type_strategy = push_type_info(diag, &mut type_info)?;
            check_selection!(diag, type_info);
        }

        check_attributes!(diag, attributes);

        let Members {
            fields,
            codes,
            decls,
            decl_idents,
            field_idents,
            field_names,
            ..
        } = {
            let constraint = MemberConstraint {
                sub_type_strategy: Some(&sub_type_strategy),
                ..MemberConstraint::default()
            };

            (item.members, constraint).into_model(diag, scope)?
        };

        let mut names = HashMap::new();
        let mut idents = HashMap::new();
        let mut sub_types = Vec::new();
        let mut untagged = LinkedHashMap::new();

        for sub_type in item.sub_types {
            let constraint = SubTypeConstraint {
                sub_type_strategy: &sub_type_strategy,
                reserved: &reserved,
                field_idents: &field_idents,
                field_names: &field_names,
                untagged: &mut untagged,
            };

            scope.push(Spanned::borrow(&sub_type.name));
            let out = (sub_type, constraint).into_model(diag, scope);
            scope.pop();

            let sub_type = try_loop!(out);

            check_conflict!(diag, idents, sub_type, sub_type.ident, "sub-type");
            check_conflict!(diag, names, sub_type, sub_type.name(), "sub-type with name");

            sub_types.push(sub_type);
        }

        if diag.has_errors() {
            return Err(());
        }

        // check that we are not violating any constraints.
        match *&sub_type_strategy {
            RpSubTypeStrategy::Untagged => {
                check_untagged(diag, &sub_types, &untagged)?;

                // Check that - in the order sub-types appear, any the key for any give
                // sub-type is not a subset of any sub-sequent sub-types.

                let mut it = untagged.iter();

                while let Some((k0, span0)) = it.next() {
                    let mut sub = it.clone();

                    while let Some((k1, span1)) = sub.next() {
                        if !k0.is_subset(k1) {
                            continue;
                        }

                        let names = k0.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");

                        diag.err(
                            span0,
                            &format!(
                                "fields with names `{}` are present in another sub-type, this \
                                 would cause deserialization to be ambiguous for certain cases.",
                                names,
                            ),
                        );

                        diag.info(
                            span0,
                            "HINT: re-order or change your sub-types to avoid this",
                        );

                        let names = k1.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");

                        diag.info(
                            span1,
                            &format!(
                                "conflicting sub-type with fields `{}` is defined here",
                                names
                            ),
                        );
                    }
                }

                if diag.has_errors() {
                    return Err(());
                }
            }
            _ => {}
        }

        return Ok(Spanned::new(
            RpInterfaceBody {
                name,
                ident: item.name.to_string(),
                comment: Comment(&comment).into_model(diag, scope)?,
                decls,
                decl_idents,
                fields,
                codes,
                sub_types,
                sub_type_strategy,
            },
            span,
        ));

        /// Check invariants that need to be enforced with unique fields
        fn check_untagged<'a, I>(
            diag: &mut Diagnostics,
            sub_types: &Vec<Spanned<RpSubType>>,
            untagged: I,
        ) -> Result<()>
        where
            I: Clone + IntoIterator<Item = (&'a BTreeSet<String>, &'a Span)>,
        {
            for sub_type in sub_types {
                let required = sub_type
                    .fields
                    .iter()
                    .filter(|f| f.is_required())
                    .map(|f| f.name().to_string())
                    .collect::<BTreeSet<_>>();

                for (key, span) in untagged.clone() {
                    // skip own
                    if *key == required {
                        continue;
                    }

                    let mut any = false;

                    let optional = sub_type.fields.iter().filter(|f| f.is_optional());

                    for f in optional.filter(|f| key.contains(f.name())) {
                        any = true;
                        diag.err(f.span(), "is a required field of another sub-type");
                    }

                    if any {
                        diag.info(span.clone(), "sub-type defined here");
                    }
                }
            }

            if diag.has_errors() {
                return Err(());
            }

            Ok(())
        }

        /// Extract type_info attribute.
        fn push_type_info(
            diag: &mut Diagnostics,
            selection: &mut Selection,
        ) -> Result<RpSubTypeStrategy> {
            if let Some(strategy) = selection.take("strategy") {
                let (strategy, span) = Spanned::take_pair(strategy);
                let id = strategy.as_string().with_span(diag, span)?;

                match id {
                    "tagged" => {
                        if let Some(tag) = selection.take("tag") {
                            let (tag, span) = Spanned::take_pair(tag);
                            let tag = tag.as_string().with_span(diag, span)?;

                            return Ok(RpSubTypeStrategy::Tagged {
                                tag: tag.to_string(),
                            });
                        }
                    }
                    "untagged" => {
                        return Ok(RpSubTypeStrategy::Untagged);
                    }
                    _ => {
                        diag.err(span, "bad strategy");
                        return Err(());
                    }
                }
            }

            Ok(RpSubTypeStrategy::default())
        }
    }
}

impl<'input> IntoModel for Spanned<Name<'input>> {
    type Output = Spanned<RpName>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::Name::*;

        let (name, span) = Spanned::take_pair(self);

        let out = match name {
            Relative { path } => {
                let path = path.into_model(diag, scope)?;

                scope
                    .as_name(span)
                    .extend(path.into_iter().map(|p| Spanned::take(p)))
            }
            Absolute { prefix, path } => {
                let path = path
                    .into_model(diag, scope)?
                    .into_iter()
                    .map(|s| Spanned::take(s))
                    .collect();

                let (prefix, package) = match prefix {
                    Some(prefix) => {
                        let (prefix, span) = Spanned::take_pair(prefix);

                        match scope.lookup_prefix(prefix.as_ref()) {
                            Some(package) => {
                                let prefix = prefix.to_string();
                                (Some(Spanned::new(prefix, span)), package.clone())
                            }
                            None => {
                                diag.err(span, format!("missing prefix `{}`", prefix.clone()));
                                return Err(());
                            }
                        }
                    }
                    None => (None, scope.package()),
                };

                RpName {
                    prefix,
                    package,
                    path,
                }
            }
        };

        Ok(Spanned::new(out, span))
    }
}

impl<'input> IntoModel for (&'input Path, usize, usize) {
    type Output = (PathBuf, usize, usize);

    fn into_model<I>(self, _: &mut Diagnostics, _scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        Ok((self.0.to_owned(), self.1, self.2))
    }
}

impl<'input> IntoModel for Item<'input, ServiceBody<'input>> {
    type Output = Spanned<RpServiceBody>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        let name = scope.as_name(item.name.span());

        diag.symbol(SymbolKind::Service, &span, &name);

        let mut decl_idents = LinkedHashMap::new();
        let mut idents = HashMap::new();
        let mut endpoint_names = HashMap::new();
        let mut endpoint_idents = HashMap::new();

        let mut endpoints = Vec::new();
        let mut decls = Vec::new();

        for member in item.members {
            match member {
                ServiceMember::Endpoint(e) => {
                    let e = try_loop!(e.into_model(diag, scope));

                    check_conflict!(diag, endpoint_idents, e, e.ident(), "endpoint");
                    check_conflict!(diag, endpoint_names, e, e.name(), "endpoint with name");

                    endpoints.push(e);
                }
                ServiceMember::InnerDecl(d) => {
                    let d = d.into_model(diag, scope)?;
                    check_conflict!(diag, idents, d, d.ident(), "inner declaration");
                    decl_idents.insert(d.ident().to_string(), decls.len());
                    decls.push(d);
                }
            };
        }

        if diag.has_errors() {
            return Err(());
        }

        let mut attributes = attributes.into_model(diag, scope)?;

        let mut http = RpServiceBodyHttp::default();

        if let Some(selection) = attributes.take_selection("http") {
            let (mut selection, _) = Spanned::take_pair(selection);
            push_http(diag, &mut selection, &mut http)?;
            check_selection!(diag, selection);
        }

        check_attributes!(diag, attributes);

        return Ok(Spanned::new(
            RpServiceBody {
                name,
                ident: item.name.to_string(),
                comment: Comment(&comment).into_model(diag, scope)?,
                decls,
                decl_idents,
                http,
                endpoints,
            },
            span,
        ));

        fn push_http(
            diag: &mut Diagnostics,
            selection: &mut Selection,
            http: &mut RpServiceBodyHttp,
        ) -> Result<()> {
            if let Some(url) = selection.take("url") {
                let (url, span) = Spanned::take_pair(url);
                let url = url.as_string().with_span(diag, span)?.to_string();
                http.url = Some(Spanned::new(url, span));
            }

            Ok(())
        }
    }
}

impl<'input> IntoModel for EndpointArgument<'input> {
    type Output = RpEndpointArgument;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let ident = self.ident.into_model(diag, scope)?;
        let safe_ident = build_safe_ident(scope, ident.as_str(), Scope::field_ident_naming);

        let argument = RpEndpointArgument {
            ident: Rc::new(ident),
            safe_ident: Rc::new(safe_ident),
            channel: self.channel.into_model(diag, scope)?,
        };

        Ok(argument)
    }
}

impl<'input> IntoModel for Item<'input, Endpoint<'input>> {
    type Output = Spanned<RpEndpoint>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        let id = item.id.into_model(diag, scope)?;
        let alias = item.alias.into_model(diag, scope)?;

        let (ident, safe_ident, name) = build_item_name(
            scope,
            id.as_str(),
            alias.as_ref().map(|s| s.as_str()),
            Scope::endpoint_naming,
            Scope::endpoint_ident_naming,
        );

        let mut arguments = Vec::new();
        let mut seen = HashMap::new();

        for argument in item.arguments {
            let argument = argument.into_model(diag, scope)?;

            if let Some(other) = seen.insert(argument.ident.to_string(), argument.ident.span()) {
                diag.err(&argument.ident.span(), "argument already present");
                diag.info(other, "argument present here");
                return Err(());
            }

            arguments.push(argument);
        }

        let response = item.response.into_model(diag, scope)?;
        let mut request = arguments.iter().cloned().next();

        let mut attributes = attributes.into_model(diag, scope)?;

        let http = attributes::endpoint_http(
            diag,
            scope,
            &mut attributes,
            &mut request,
            response.as_ref(),
            &arguments,
        )?;

        check_attributes!(diag, attributes);

        Ok(Spanned::new(
            RpEndpoint {
                ident: ident,
                safe_ident: safe_ident,
                name: name,
                comment: Comment(&comment).into_model(diag, scope)?,
                attributes: attributes,
                arguments: arguments,
                request: request,
                response: response,
                http: http,
            },
            span,
        ))
    }
}

impl<'input> IntoModel for Channel<'input> {
    type Output = RpChannel;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::Channel::*;

        let result = match self {
            Unary { ty, .. } => RpChannel::Unary {
                ty: ty.into_model(diag, scope)?,
            },
            Streaming { ty, .. } => RpChannel::Streaming {
                ty: ty.into_model(diag, scope)?,
            },
        };

        Ok(result)
    }
}

impl<'input> IntoModel for (Item<'input, SubType<'input>>, SubTypeConstraint<'input>) {
    type Output = Spanned<RpSubType>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::TypeMember::*;

        let (item, constraint) = self;

        let SubTypeConstraint {
            reserved: interface_reserved,
            field_idents,
            field_names,
            sub_type_strategy,
            untagged,
        } = constraint;

        let Item {
            comment,
            attributes,
            item,
        } = item;

        let (item, span) = Spanned::take_pair(item);

        let name = scope.as_name(item.name.span());

        let mut attributes = attributes.into_model(diag, scope)?;
        let reserved = attributes::reserved(diag, &mut attributes)?;
        check_attributes!(diag, attributes);

        let mut fields = Vec::new();
        let mut codes = Vec::new();
        let mut decls = Vec::new();

        let mut decl_idents = LinkedHashMap::new();

        let mut idents = HashMap::new();
        let mut field_idents = field_idents.clone();
        let mut field_names = field_names.clone();

        for member in item.members {
            match member {
                Field(field) => {
                    let field = try_loop!(field.into_model(diag, scope));

                    check_conflict!(diag, field_idents, field, &field.ident, "field");
                    check_conflict!(diag, field_names, field, field.name(), "field with name");

                    check_field_tag!(diag, field, *sub_type_strategy);

                    check_field_reserved!(diag, field, interface_reserved);
                    check_field_reserved!(diag, field, reserved);

                    fields.push(field);
                }
                Code(code) => {
                    codes.push(try_loop!(code.into_model(diag, scope)));
                }
                InnerDecl(d) => {
                    let d = try_loop!(d.into_model(diag, scope));
                    check_conflict!(diag, idents, d, d.ident(), "inner declaration");
                    decl_idents.insert(d.ident().to_string(), decls.len());
                    decls.push(d);
                }
            }
        }

        if diag.has_errors() {
            return Err(());
        }

        let sub_type_name = sub_type_name(diag, item.alias, scope)?;

        match *sub_type_strategy {
            RpSubTypeStrategy::Untagged => {
                let fields = fields
                    .iter()
                    .filter(|f| f.is_required())
                    .map(|f| f.name().to_string())
                    .collect::<BTreeSet<_>>();

                if let Some(other) = untagged.insert(fields, span.clone()) {
                    diag.err(span, "does not have a unique set of fields");
                    diag.info(other, "previously defined here");
                    return Err(());
                }
            }
            _ => {}
        }

        return Ok(Spanned::new(
            RpSubType {
                name,
                ident: item.name.to_string(),
                comment: Comment(&comment).into_model(diag, scope)?,
                decls,
                decl_idents,
                fields,
                codes,
                sub_type_name,
            },
            span,
        ));

        /// Extract all names provided.
        fn alias_name<'input, I>(
            diag: &mut Diagnostics,
            alias: Spanned<Value<'input>>,
            scope: &mut Scope<I>,
        ) -> Result<Spanned<String>>
        where
            I: Import,
        {
            let (alias, span) = Spanned::take_pair(alias.into_model(diag, scope)?);

            match alias {
                RpValue::String(string) => Ok(Spanned::new(string, span)),
                _ => {
                    diag.err(span, "expected string");
                    return Err(());
                }
            }
        }

        fn sub_type_name<'input, I>(
            diag: &mut Diagnostics,
            alias: option::Option<Spanned<Value<'input>>>,
            scope: &mut Scope<I>,
        ) -> Result<::std::option::Option<Spanned<String>>>
        where
            I: Import,
        {
            if let Some(alias) = alias {
                alias_name(diag, alias, scope).map(Some)
            } else {
                Ok(None)
            }
        }
    }
}

impl<'input> IntoModel for Item<'input, TupleBody<'input>> {
    type Output = Spanned<RpTupleBody>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        let name = scope.as_name(item.name.span());

        diag.symbol(SymbolKind::Tuple, &span, &name);

        let Members {
            fields,
            codes,
            decls,
            decl_idents,
            ..
        } = item.members.into_model(diag, scope)?;

        let attributes = attributes.into_model(diag, scope)?;
        check_attributes!(diag, attributes);

        Ok(Spanned::new(
            RpTupleBody {
                name,
                ident: item.name.to_string(),
                comment: Comment(&comment).into_model(diag, scope)?,
                decls,
                decl_idents,
                fields,
                codes,
            },
            span,
        ))
    }
}

impl<'input> IntoModel for Item<'input, TypeBody<'input>> {
    type Output = Spanned<RpTypeBody>;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let Item {
            comment,
            attributes,
            item,
        } = self;

        let (item, span) = Spanned::take_pair(item);

        let name = scope.as_name(item.name.span());

        diag.symbol(SymbolKind::Type, &span, &name);

        let mut attributes = attributes.into_model(diag, scope)?;
        let reserved = attributes::reserved(diag, &mut attributes)?;

        check_attributes!(diag, attributes);

        let Members {
            fields,
            codes,
            decls,
            decl_idents,
            ..
        } = {
            let constraint = MemberConstraint {
                reserved: Some(&reserved),
                ..MemberConstraint::default()
            };

            (item.members, constraint).into_model(diag, scope)?
        };

        Ok(Spanned::new(
            RpTypeBody {
                name,
                ident: item.name.to_string(),
                comment: Comment(&comment).into_model(diag, scope)?,
                decls,
                decl_idents,
                fields,
                codes,
            },
            span,
        ))
    }
}

/// Default constraints.
impl<'input> IntoModel for Vec<TypeMember<'input>> {
    type Output = Members;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        (self, MemberConstraint::default()).into_model(diag, scope)
    }
}

impl<'input> IntoModel for (Vec<TypeMember<'input>>, MemberConstraint<'input>) {
    type Output = Members;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::TypeMember::*;

        let (members, constraint) = self;

        let MemberConstraint {
            sub_type_strategy,
            reserved,
        } = constraint;

        let mut fields: Vec<Spanned<RpField>> = Vec::new();
        let mut codes = Vec::new();
        let mut decls = Vec::new();
        let mut decl_idents = LinkedHashMap::new();

        let mut field_idents = HashMap::new();
        let mut field_names = HashMap::new();
        let mut idents = HashMap::new();

        for member in members {
            match member {
                Field(field) => {
                    let field = try_loop!(field.into_model(diag, scope));

                    check_conflict!(diag, field_idents, field, &field.ident, "field");
                    check_conflict!(diag, field_names, field, field.name(), "field with name");

                    if let Some(sub_type_strategy) = sub_type_strategy {
                        check_field_tag!(diag, field, *sub_type_strategy);
                    }

                    if let Some(reserved) = reserved {
                        check_field_reserved!(diag, field, reserved);
                    }

                    fields.push(field);
                }
                Code(code) => codes.push(try_loop!(code.into_model(diag, scope))),
                InnerDecl(d) => {
                    let d = try_loop!(d.into_model(diag, scope));
                    check_conflict!(diag, idents, d, d.ident(), "inner declaration");
                    decl_idents.insert(d.ident().to_string(), decls.len());
                    decls.push(d);
                }
            }
        }

        if diag.has_errors() {
            return Err(());
        }

        Ok(Members {
            fields,
            codes,
            decls,
            decl_idents,
            field_names,
            field_idents,
        })
    }
}

impl<'input> IntoModel for Code<'input> {
    type Output = RpCode;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let mut attributes = self.attributes.into_model(diag, scope)?;
        let context = self.context.into_model(diag, scope)?;

        // Context-specific settings.
        let context = {
            let (context, span) = Spanned::take_pair(context);

            match context.as_str() {
                "csharp" => RpContext::Csharp {},
                "go" => RpContext::Go {},
                "java" => {
                    let imports = attributes::import(diag, &mut attributes)?;
                    RpContext::Java { imports: imports }
                }
                "js" => RpContext::Js {},
                "python" => RpContext::Python {},
                "reproto" => RpContext::Reproto {},
                "rust" => RpContext::Rust {},
                "swift" => RpContext::Swift {},
                context => {
                    diag.err(span, format!("context `{}` not recognized", context));
                    return Err(());
                }
            }
        };

        check_attributes!(diag, attributes);

        Ok(RpCode {
            context: context,
            lines: self.content.into_iter().map(|s| s.to_string()).collect(),
        })
    }
}

impl<'input> IntoModel for Value<'input> {
    type Output = RpValue;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::Value::*;

        let out = match self {
            String(string) => RpValue::String(string),
            Number(number) => RpValue::Number(number),
            Identifier(identifier) => RpValue::Identifier(identifier.to_string()),
            Array(inner) => RpValue::Array(inner.into_model(diag, scope)?),
            Name(name) => RpValue::Name(name.into_model(diag, scope)?),
        };

        Ok(out)
    }
}

impl<'input> IntoModel for Vec<Spanned<Attribute<'input>>> {
    type Output = Attributes;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        use self::Attribute::*;

        let mut words = HashMap::new();
        let mut selections = HashMap::new();

        for attribute in self {
            let (attr, attr_pos) = Spanned::take_pair(attribute);

            match attr {
                Word(word) => {
                    let (word, span) = Spanned::take_pair(word.into_model(diag, scope)?);

                    if let Some(old) = words.insert(word, span.clone()) {
                        diag.err(span, "word already present");
                        diag.info(old, "old attribute here");
                        return Err(());
                    }
                }
                List(key, name_values) => {
                    let key = Spanned::take(key.into_model(diag, scope)?);

                    match selections.entry(key) {
                        hash_map::Entry::Vacant(entry) => {
                            let mut words = Vec::new();
                            let mut values = HashMap::new();

                            for name_value in name_values {
                                match name_value {
                                    AttributeItem::Word(word) => {
                                        words.push(word.into_model(diag, scope)?);
                                    }
                                    AttributeItem::NameValue { name, value } => {
                                        let name = name.into_model(diag, scope)?;
                                        let value = value.into_model(diag, scope)?;
                                        values
                                            .insert(Spanned::borrow(&name).clone(), (name, value));
                                    }
                                }
                            }

                            let selection = Selection::new(words, values);
                            entry.insert(Spanned::new(selection, attr_pos));
                        }
                        hash_map::Entry::Occupied(entry) => {
                            diag.err(attr_pos, "attribute already present");
                            diag.info(entry.get().span(), "attribute here");
                            return Err(());
                        }
                    }
                }
            }
        }

        Ok(Attributes::new(words, selections))
    }
}

#[allow(unused)]
type Variables<'a> = HashMap<&'a str, &'a RpEndpointArgument>;

impl<'input, 'a: 'input> IntoModel for (Span, &'input mut Variables<'a>, PathSpec<'input>) {
    type Output = RpPathSpec;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let (span, vars, spec) = self;

        let mut out = Vec::new();

        for s in spec.steps {
            out.push((span, &mut *vars, s).into_model(diag, scope)?);
        }

        Ok(RpPathSpec { steps: out })
    }
}

impl<'input, 'a: 'input> IntoModel for (Span, &'input mut Variables<'a>, PathStep<'input>) {
    type Output = RpPathStep;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let (span, vars, step) = self;

        let mut out = Vec::new();

        for p in step.parts {
            out.push((span, &mut *vars, p).into_model(diag, scope)?);
        }

        Ok(RpPathStep { parts: out })
    }
}

impl<'input, 'a: 'input> IntoModel for (Span, &'input mut Variables<'a>, PathPart<'input>) {
    type Output = RpPathPart;

    fn into_model<I>(self, diag: &mut Diagnostics, scope: &mut Scope<I>) -> Result<Self::Output>
    where
        I: Import,
    {
        let (span, vars, part) = self;

        use self::PathPart::*;

        let out = match part {
            Variable(variable) => {
                let var = variable.into_model(diag, scope)?;

                let var = match vars.remove(var.as_str()) {
                    Some(rp) => rp.clone(),
                    None => {
                        diag.err(
                            span,
                            format!("path variable `{}` is not an argument to endpoint", var),
                        );

                        return Err(());
                    }
                };

                RpPathPart::Variable(var)
            }
            Segment(segment) => RpPathPart::Segment(segment),
        };

        Ok(out)
    }
}
