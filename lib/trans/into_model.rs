use ast::*;
use core::*;
use core::errors::{Error, Result};
use linked_hash_map::{self, LinkedHashMap};
use path_parser;
use scope::Scope;
use std::borrow::Cow;
use std::collections::{hash_map, BTreeMap, HashMap, HashSet};
use std::option;
use std::path::{Path, PathBuf};
use std::rc::Rc;

macro_rules! check_attributes {
    ($scope:expr, $attr:expr) => {{
        let mut __a_r = $scope.ctx().report();

        for unused in $attr.unused() {
            __a_r = __a_r.err(unused, "unknown attribute");
        }

        if let Some(e) = __a_r.close() {
            return Err(e.into());
        }
    }}
}

macro_rules! check_selection {
    ($scope:expr, $sel:expr) => {{
        let mut __a_r = $scope.ctx().report();

        for unused in $sel.unused() {
            __a_r = __a_r.err(unused, "unknown attribute");
        }

        if let Some(e) = __a_r.close() {
            return Err(e.into());
        }
    }}
}

/// Adds a method for all types that supports conversion into core types.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, scope: &Scope) -> Result<Self::Output>;
}

/// Generic implementation for vectors.
impl<T> IntoModel for Loc<T>
where
    T: IntoModel,
{
    type Output = Loc<T::Output>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (value, pos) = Loc::take_pair(self);
        Ok(Loc::new(value.into_model(scope)?, pos))
    }
}

/// Generic implementation for vectors.
impl<T> IntoModel for Vec<T>
where
    T: IntoModel,
{
    type Output = Vec<T::Output>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let mut out = Vec::new();

        for v in self {
            out.push(v.into_model(scope)?);
        }

        Ok(out)
    }
}

impl<T> IntoModel for Option<T>
where
    T: IntoModel,
{
    type Output = Option<T::Output>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        if let Some(value) = self {
            return Ok(Some(value.into_model(scope)?));
        }

        Ok(None)
    }
}

impl<T> IntoModel for Box<T>
where
    T: IntoModel,
{
    type Output = Box<T::Output>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        Ok(Box::new((*self).into_model(scope)?))
    }
}

impl<'a> IntoModel for Cow<'a, str> {
    type Output = String;

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
        Ok(self.to_string())
    }
}

impl IntoModel for String {
    type Output = String;

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
        Ok(self)
    }
}

/// Helper model to strip whitespace prefixes from comment lines.
pub struct Comment<I>(I);

impl<I: IntoIterator<Item = S>, S: AsRef<str>> IntoModel for Comment<I> {
    type Output = Vec<String>;

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
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

impl IntoModel for Type {
    type Output = RpType;

    fn into_model(self, scope: &Scope) -> Result<RpType> {
        use self::Type::*;

        let out = match self {
            Double => RpType::Double,
            Float => RpType::Float,
            Signed { size } => RpType::Signed { size: size },
            Unsigned { size } => RpType::Unsigned { size: size },
            Boolean => RpType::Boolean,
            String => RpType::String,
            DateTime => RpType::DateTime,
            Name { name } => RpType::Name {
                name: name.into_model(scope)?,
            },
            Array { inner } => RpType::Array {
                inner: inner.into_model(scope)?,
            },
            Map { key, value } => RpType::Map {
                key: key.into_model(scope)?,
                value: value.into_model(scope)?,
            },
            Any => RpType::Any,
            Bytes => RpType::Bytes,
        };

        Ok(out)
    }
}

impl<'input> IntoModel for Decl<'input> {
    type Output = RpDecl;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::Decl::*;

        let s = scope.child(self.name().to_owned());

        let out = match self {
            Type(body) => RpDecl::Type(Rc::new(body.into_model(&s)?)),
            Interface(body) => RpDecl::Interface(Rc::new(body.into_model(&s)?)),
            Enum(body) => RpDecl::Enum(Rc::new(body.into_model(&s)?)),
            Tuple(body) => RpDecl::Tuple(Rc::new(body.into_model(&s)?)),
            Service(body) => RpDecl::Service(Rc::new(body.into_model(&s)?)),
        };

        Ok(out)
    }
}

impl<'input> IntoModel for Item<'input, EnumBody<'input>> {
    type Output = Loc<RpEnumBody>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        self.map(|comment, attributes, item| {
            let mut variants: Vec<Rc<Loc<RpVariant>>> = Vec::new();

            let mut codes = Vec::new();
            let mut options = Vec::new();
            let mut decls = Vec::new();

            for member in item.members {
                match member {
                    EnumMember::Code(code) => {
                        codes.push(code.into_model(scope)?);
                    }
                    EnumMember::Option(option) => {
                        options.push(option.into_model(scope)?);
                    }
                    EnumMember::InnerDecl(decl) => {
                        decls.push(decl.into_model(scope)?);
                    }
                };
            }

            let ty = item.ty.into_model(scope)?;

            let variant_type = if let Some(ty) = ty {
                Loc::take(Loc::and_then(ty, |ty| {
                    ty.as_enum_type()
                        .ok_or_else(|| "expected string or absent".into())
                        as Result<RpEnumType>
                })?)
            } else {
                RpEnumType::Generated
            };

            for variant in item.variants {
                let variant = (variants.as_slice(), variant, &variant_type).into_model(scope)?;
                variants.push(variant);
            }

            let attributes = attributes.into_model(scope)?;
            check_attributes!(scope, attributes);

            Ok(RpEnumBody {
                name: scope.as_name(),
                local_name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                decls: decls,
                variant_type: variant_type,
                variants: variants,
                codes: codes,
            })
        })
    }
}

impl<'input, 'a> IntoModel
    for (
        &'input [Rc<Loc<RpVariant>>],
        Item<'input, EnumVariant<'input>>,
        &'a RpEnumType,
    ) {
    type Output = Rc<Loc<RpVariant>>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (variants, variant, ty) = self;

        let ctx = scope.ctx();
        let variant = (variant, ty).into_model(scope)?;

        if let Some(other) = variants
            .iter()
            .find(|v| *v.local_name == *variant.local_name)
        {
            return Err(ctx.report()
                .err(Loc::pos(&variant.local_name), "conflicting enum name")
                .info(Loc::pos(&other.local_name), "previous variant here")
                .into());
        }

        Ok(Rc::new(variant))
    }
}

/// enum value with assigned ordinal
impl<'input, 'a> IntoModel for (Item<'input, EnumVariant<'input>>, &'a RpEnumType) {
    type Output = Loc<RpVariant>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (variant, ty) = self;

        variant.map(|comment, attributes, item| {
            let ordinal = if let Some(argument) = item.argument.into_model(scope)? {
                if !ty.is_assignable_from(&argument) {
                    return Err(
                        format!("unexpected value {}, expected type {}", argument, ty).into(),
                    );
                }

                Loc::take(Loc::and_then(argument, |value| value.into_ordinal())?)
            } else {
                RpEnumOrdinal::Generated
            };

            let attributes = attributes.into_model(scope)?;
            check_attributes!(scope, attributes);

            Ok(RpVariant {
                name: scope.as_name().push(item.name.to_string()),
                local_name: Loc::map(item.name.clone(), |s| s.to_string()),
                comment: Comment(&comment).into_model(scope)?,
                ordinal: ordinal,
            })
        })
    }
}

impl<'input> IntoModel for Item<'input, Field<'input>> {
    type Output = Loc<RpField>;

    fn into_model(self, scope: &Scope) -> Result<Loc<RpField>> {
        self.map(|comment, attributes, item| {
            let name = &item.name;

            let field_as = item.field_as
                .into_model(scope)?
                .or_else(|| scope.field_naming().map(|n| n.convert(name)));

            let attributes = attributes.into_model(scope)?;
            check_attributes!(scope, attributes);

            Ok(RpField {
                modifier: item.modifier,
                name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                ty: item.ty.into_model(scope)?,
                field_as: field_as,
            })
        })
    }
}

impl<'input> IntoModel for File<'input> {
    type Output = RpFile;

    fn into_model(self, scope: &Scope) -> Result<RpFile> {
        let options = self.options.into_model(scope)?;
        let decls = self.decls.into_model(scope)?;

        Ok(RpFile {
            comment: Comment(&self.comment).into_model(scope)?,
            options: options,
            decls: decls,
        })
    }
}

impl<'input> IntoModel for Item<'input, InterfaceBody<'input>> {
    type Output = Loc<RpInterfaceBody>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use std::collections::btree_map::Entry::*;

        self.map(|comment, attributes, item| {
            let ctx = scope.ctx();

            let (fields, codes, _options, decls) = item.members.into_model(scope)?;

            let mut sub_types: BTreeMap<String, Rc<Loc<RpSubType>>> = BTreeMap::new();

            for sub_type in item.sub_types {
                let scope = scope.child(Loc::value(&sub_type.name).to_owned());

                let sub_type = sub_type.into_model(&scope)?;

                // key has to be owned by entry
                let key = sub_type.local_name.clone();

                match sub_types.entry(key) {
                    Vacant(entry) => entry.insert(Rc::new(sub_type)),
                    Occupied(entry) => {
                        return Err(ctx.report()
                            .err(Loc::pos(&sub_type), "sub-type already defined")
                            .info(Loc::pos(entry.get()), "already defined here")
                            .into());
                    }
                };
            }

            let mut attributes = attributes.into_model(scope)?;

            let mut sub_type_strategy = RpSubTypeStrategy::default();

            if let Some(mut type_info) = attributes.take_selection("type_info") {
                sub_type_strategy = push_type_info(ctx, &mut type_info)?;
                check_selection!(scope, type_info);
            }

            check_attributes!(scope, attributes);

            return Ok(RpInterfaceBody {
                name: scope.as_name(),
                local_name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                decls: decls,
                fields: fields,
                codes: codes,
                sub_types: sub_types,
                sub_type_strategy: sub_type_strategy,
            });

            /// Extract type_info attribute.
            fn push_type_info(
                ctx: &Context,
                selection: &mut Selection,
            ) -> Result<RpSubTypeStrategy> {
                if let Some(strategy) = selection.take("strategy") {
                    let id = strategy.as_string()?;

                    match id {
                        "tagged" => {
                            if let Some(tag) = selection.take("tag") {
                                let tag = tag.as_string()?;

                                return Ok(RpSubTypeStrategy::Tagged {
                                    tag: tag.to_string(),
                                });
                            }
                        }
                        _ => {
                            return Err(ctx.report()
                                .err(Loc::pos(&strategy), "bad strategy")
                                .into());
                        }
                    }
                }

                Ok(RpSubTypeStrategy::default())
            }
        })
    }
}

impl IntoModel for RpPackage {
    type Output = RpPackage;

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
        Ok(self)
    }
}

impl IntoModel for Name {
    type Output = RpName;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::Name::*;

        let out = match self {
            Relative { parts } => scope.as_name().extend(parts),
            Absolute { prefix, parts } => {
                let package = if let Some(ref prefix) = prefix {
                    if let Some(package) = scope.lookup_prefix(prefix) {
                        package.clone()
                    } else {
                        return Err(Error::new(format!("Missing prefix: {}", prefix.clone())));
                    }
                } else {
                    scope.package()
                };

                RpName {
                    prefix: prefix,
                    package: package,
                    parts: parts,
                }
            }
        };

        Ok(out)
    }
}

impl<'input> IntoModel for (&'input Path, usize, usize) {
    type Output = (PathBuf, usize, usize);

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
        Ok((self.0.to_owned(), self.1, self.2))
    }
}

impl<'input> IntoModel for OptionDecl<'input> {
    type Output = RpOptionDecl;

    fn into_model(self, scope: &Scope) -> Result<RpOptionDecl> {
        let decl = RpOptionDecl {
            name: self.name.to_string(),
            value: self.value.into_model(scope)?,
        };

        Ok(decl)
    }
}

impl<'input> IntoModel for Item<'input, ServiceBody<'input>> {
    type Output = Loc<RpServiceBody>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use linked_hash_map::Entry::*;

        return self.map(|comment, attributes, item| {
            let ctx = scope.ctx();

            let mut endpoint_names: HashMap<String, ErrorPos> = HashMap::new();
            let mut endpoints = LinkedHashMap::new();
            let mut options = Vec::new();
            let mut decls = Vec::new();

            for member in item.members {
                match member {
                    ServiceMember::Endpoint(endpoint) => {
                        handle_endpoint(endpoint, scope, &mut endpoint_names, &mut endpoints)?;
                    }
                    ServiceMember::Option(option) => {
                        options.push(option.into_model(scope)?);
                    }
                    ServiceMember::InnerDecl(decl) => {
                        decls.push(decl.into_model(scope)?);
                    }
                };
            }

            let mut attributes = attributes.into_model(scope)?;

            let mut http = RpServiceBodyHttp::default();

            if let Some(selection) = attributes.take_selection("http") {
                let (mut selection, pos) = Loc::take_pair(selection);
                push_http(ctx, scope, &mut selection, &mut http).with_pos(pos)?;
                check_selection!(scope, selection);
            }

            check_attributes!(scope, attributes);

            Ok(RpServiceBody {
                name: scope.as_name(),
                local_name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                decls: decls,
                http: http,
                endpoints: endpoints,
            })
        });

        fn push_http(
            _ctx: &Context,
            _scope: &Scope,
            selection: &mut Selection,
            http: &mut RpServiceBodyHttp,
        ) -> Result<()> {
            if let Some(url) = selection.take("url") {
                let url = Loc::and_then(url, |url| url.as_string().map(ToOwned::to_owned))?;
                http.url = Some(url);
            }

            Ok(())
        }

        /// Handle a single endpoint.
        fn handle_endpoint<'input>(
            endpoint: Item<'input, Endpoint<'input>>,
            scope: &Scope,
            endpoint_names: &mut HashMap<String, ErrorPos>,
            endpoints: &mut LinkedHashMap<String, Loc<RpEndpoint>>,
        ) -> Result<()> {
            let ctx = scope.ctx();

            let endpoint = endpoint.into_model(scope)?;

            // Check that there are no conflicting endpoint names.
            match endpoint_names.entry(endpoint.name().to_string()) {
                hash_map::Entry::Vacant(entry) => entry.insert(Loc::pos(&endpoint).into()),
                hash_map::Entry::Occupied(entry) => {
                    return Err(ctx.report()
                        .err(Loc::pos(&endpoint), "conflicting name of endpoint")
                        .info(entry.get().clone_error_pos(), "previous name here")
                        .into());
                }
            };

            // Check that there are no conflicting endpoint IDs.
            match endpoints.entry(Loc::value(&endpoint.id).to_string()) {
                Vacant(entry) => entry.insert(endpoint),
                Occupied(entry) => {
                    return Err(ctx.report()
                        .err(Loc::pos(&endpoint), "conflicting id of endpoint")
                        .info(Loc::pos(entry.get()), "previous id here")
                        .into());
                }
            };

            Ok(())
        }
    }
}

impl<'input> IntoModel for Item<'input, Endpoint<'input>> {
    type Output = Loc<RpEndpoint>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        return self.map(|comment, attributes, item| {
            let id = item.id.into_model(scope)?;
            let ctx = scope.ctx();

            let name = item.alias.into_model(scope)?.unwrap_or_else(|| {
                scope
                    .endpoint_naming()
                    .map(|n| n.convert(id.as_str()))
                    .unwrap_or_else(|| id.to_string())
            });

            let mut arguments = LinkedHashMap::new();

            for (name, channel) in item.arguments {
                let name = name.into_model(scope)?;
                let channel = channel.into_model(scope)?;

                match arguments.entry(Loc::value(&name).clone()) {
                    linked_hash_map::Entry::Vacant(entry) => {
                        entry.insert((name, channel));
                    }
                    linked_hash_map::Entry::Occupied(entry) => {
                        return Err(ctx.report()
                            .err(Loc::pos(&name), "argument already present")
                            .info(Loc::pos(&entry.get().0), "argument present here")
                            .into());
                    }
                }
            }

            let response = item.response.into_model(scope)?;

            let mut attributes = attributes.into_model(scope)?;

            let mut http = RpEndpointHttp::default();

            if let Some(selection) = attributes.take_selection("http") {
                let (mut selection, pos) = Loc::take_pair(selection);

                push_http(
                    ctx,
                    scope,
                    response.as_ref(),
                    &arguments,
                    &mut selection,
                    &mut http,
                ).with_pos(&pos)?;

                check_selection!(scope, selection);
            }

            check_attributes!(scope, attributes);

            Ok(RpEndpoint {
                id: id,
                name: name,
                comment: Comment(&comment).into_model(scope)?,
                attributes: attributes,
                arguments: arguments,
                response: response,
                http: http,
            })
        });

        /// Add HTTP options associated with an endpoint.
        fn push_http(
            ctx: &Context,
            scope: &Scope,
            response: Option<&Loc<RpChannel>>,
            arguments: &LinkedHashMap<String, (Loc<String>, Loc<RpChannel>)>,
            selection: &mut Selection,
            http: &mut RpEndpointHttp,
        ) -> Result<()> {
            // Keep track of used variables.
            let mut unused_args = arguments
                .iter()
                .map(|(key, value)| (key.as_str(), &value.0))
                .collect::<HashMap<_, _>>();

            if let Some(path) = selection.take("path") {
                let (path, pos) = Loc::take_pair(path);
                http.path = Some(parse_path(scope, path, &mut unused_args).with_pos(pos)?);
            }

            if let Some(body) = selection.take("body") {
                let (body, pos) = Loc::take_pair(body);
                let body = body.as_identifier().with_pos(&pos)?;

                if unused_args.remove(body).is_none() {
                    return Err(format!("no such argument: {}", body).into()).with_pos(&pos);
                }

                http.body = Some(body.to_string());
            }

            if let Some(method) = selection.take("method") {
                let (method, pos) = Loc::take_pair(method);
                http.method = Some(parse_method(method).with_pos(pos)?);
            }

            if let Some(accept) = selection.take("accept") {
                let accept = Loc::and_then(accept, |a| {
                    a.as_string().and_then(|a| match a {
                        "application/json" => Ok(RpAccept::Json),
                        "text/plain" => Ok(RpAccept::Text),
                        _ => Err("unsupported media type".into()),
                    })
                })?;

                http_verify_accept(ctx, &accept, response)?;
                http.accept = Loc::take(accept);
            }

            // Assert that all arguments are used somehow.
            if !unused_args.is_empty() {
                let mut report = ctx.report();

                for arg in unused_args.values() {
                    report =
                        report.err(Loc::pos(arg), "Argument not used in #[http(...)] attribute");
                }

                return Err(report.into());
            }

            Ok(())
        }

        /// Parse a path specification.
        fn parse_path(
            scope: &Scope,
            path: RpValue,
            unused_args: &mut HashMap<&str, &Loc<String>>,
        ) -> Result<RpPathSpec> {
            let path = path.as_string()?;
            let path = path_parser::parse(path)
                .map_err(|e| format!("Bad path: {}: {}", path, e.display()))?;
            let path = path.into_model(scope)?;

            for var in path.vars() {
                if unused_args.remove(var).is_none() {
                    return Err(format!("no such argument: {}", var).into());
                }
            }

            Ok(path)
        }

        /// Parse a method.
        fn parse_method(method: RpValue) -> Result<RpHttpMethod> {
            use self::RpHttpMethod::*;

            let m = match method.as_string()? {
                "GET" => GET,
                "POST" => POST,
                "PUT" => PUT,
                "UPDATE" => UPDATE,
                "DELETE" => DELETE,
                "PATCH" => PATCH,
                "HEAD" => HEAD,
                method => return Err(format!("no such method: {}", method).into()),
            };

            Ok(m)
        }

        /// Check that accept matches response.
        fn http_verify_accept(
            ctx: &Context,
            accept: &Loc<RpAccept>,
            response: Option<&Loc<RpChannel>>,
        ) -> Result<()> {
            let response = match response {
                Some(response) => response,
                None => return Ok(()),
            };

            let (accept, pos) = Loc::borrow_pair(&accept);

            match *accept {
                // Can handle complex data types.
                ref accept if *accept == RpAccept::Json => return Ok(()),
                _ => {
                    if *response.ty() == RpType::String {
                        return Ok(());
                    }

                    return Err(ctx.report()
                        .err(
                            Loc::pos(response),
                            "Only `string` responses are supported for the given `accept`",
                        )
                        .info(pos, "Specified here")
                        .into());
                }
            }
        }
    }
}

impl<'input> IntoModel for Channel {
    type Output = RpChannel;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::Channel::*;

        let result = match self {
            Unary { ty, .. } => RpChannel::Unary {
                ty: ty.into_model(scope)?,
            },
            Streaming { ty, .. } => RpChannel::Streaming {
                ty: ty.into_model(scope)?,
            },
        };

        Ok(result)
    }
}

impl<'input> IntoModel for Item<'input, SubType<'input>> {
    type Output = Loc<RpSubType>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::TypeMember::*;

        return self.map(|comment, attributes, item| {
            let ctx = scope.ctx();

            let mut fields: Vec<Loc<RpField>> = Vec::new();
            let mut codes = Vec::new();
            let mut options = Vec::new();
            let mut decls = Vec::new();

            for member in item.members {
                match member {
                    Field(field) => {
                        let field = field.into_model(scope)?;

                        if let Some(other) = fields
                            .iter()
                            .find(|f| f.name() == field.name() || f.ident() == field.ident())
                        {
                            return Err(ctx.report()
                                .err(Loc::pos(&field), "conflict in field")
                                .info(Loc::pos(other), "previous declaration here")
                                .into());
                        }

                        fields.push(field);
                    }
                    Code(code) => {
                        codes.push(code.into_model(scope)?);
                    }
                    Option(option) => {
                        options.push(option.into_model(scope)?);
                    }
                    InnerDecl(decl) => {
                        decls.push(decl.into_model(scope)?);
                    }
                }
            }

            let sub_type_name = sub_type_name(item.alias, scope)?;

            let attributes = attributes.into_model(scope)?;
            check_attributes!(scope, attributes);

            Ok(RpSubType {
                name: scope.as_name(),
                local_name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                decls: decls,
                fields: fields,
                codes: codes,
                sub_type_name: sub_type_name,
            })
        });

        /// Extract all names provided.
        fn alias_name<'input>(alias: Loc<Value<'input>>, scope: &Scope) -> Result<Loc<String>> {
            let (alias, pos) = Loc::take_pair(alias.into_model(scope)?);

            match alias {
                RpValue::String(string) => Ok(Loc::new(string, pos)),
                _ => Err("expected string".into()).with_pos(pos),
            }
        }

        fn sub_type_name<'input>(
            alias: option::Option<Loc<Value<'input>>>,
            scope: &Scope,
        ) -> Result<::std::option::Option<Loc<String>>> {
            if let Some(alias) = alias {
                alias_name(alias, scope).map(Some)
            } else {
                Ok(None)
            }
        }
    }
}

impl<'input> IntoModel for Item<'input, TupleBody<'input>> {
    type Output = Loc<RpTupleBody>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        self.map(|comment, attributes, item| {
            let (fields, codes, _options, decls) = item.members.into_model(scope)?;

            let attributes = attributes.into_model(scope)?;
            check_attributes!(scope, attributes);

            Ok(RpTupleBody {
                name: scope.as_name(),
                local_name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                decls: decls,
                fields: fields,
                codes: codes,
            })
        })
    }
}

impl<'input> IntoModel for Item<'input, TypeBody<'input>> {
    type Output = Loc<RpTypeBody>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        self.map(|comment, attributes, item| {
            let (fields, codes, options, decls) = item.members.into_model(scope)?;

            let reserved: HashSet<Loc<String>> = options
                .find_all_identifiers("reserved")?
                .into_iter()
                .collect();

            let attributes = attributes.into_model(scope)?;
            check_attributes!(scope, attributes);

            Ok(RpTypeBody {
                name: scope.as_name(),
                local_name: item.name.to_string(),
                comment: Comment(&comment).into_model(scope)?,
                decls: decls,
                fields: fields,
                codes: codes,
                reserved: reserved,
            })
        })
    }
}

impl<'input> IntoModel for Vec<TypeMember<'input>> {
    type Output = (
        Vec<Loc<RpField>>,
        Vec<Loc<RpCode>>,
        Vec<Loc<RpOptionDecl>>,
        Vec<RpDecl>,
    );

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::TypeMember::*;

        let ctx = scope.ctx();

        let mut fields: Vec<Loc<RpField>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();
        let mut decls = Vec::new();

        for member in self {
            match member {
                Field(field) => {
                    let field = field.into_model(scope)?;

                    if let Some(other) = fields
                        .iter()
                        .find(|f| f.name() == field.name() || f.ident() == field.ident())
                    {
                        return Err(ctx.report()
                            .err(Loc::pos(&field), "conflict in field")
                            .info(Loc::pos(other), "previous declaration here")
                            .into());
                    }

                    fields.push(field);
                }
                Code(code) => codes.push(code.into_model(scope)?),
                Option(option) => options.push(option.into_model(scope)?),
                InnerDecl(decl) => decls.push(decl.into_model(scope)?),
            }
        }

        Ok((fields, codes, options, decls))
    }
}

impl<'input> IntoModel for Code<'input> {
    type Output = RpCode;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        Ok(RpCode {
            context: self.context.into_model(scope)?,
            lines: self.content.into_iter().map(|s| s.to_string()).collect(),
        })
    }
}

impl<'input> IntoModel for Value<'input> {
    type Output = RpValue;

    fn into_model(self, scope: &Scope) -> Result<RpValue> {
        use self::Value::*;

        let out = match self {
            String(string) => RpValue::String(string),
            Number(number) => RpValue::Number(number),
            Boolean(boolean) => RpValue::Boolean(boolean),
            Identifier(identifier) => RpValue::Identifier(identifier.to_string()),
            Array(inner) => RpValue::Array(inner.into_model(scope)?),
            Type(ty) => RpValue::Type(ty.into_model(scope)?),
        };

        Ok(out)
    }
}

impl<'input> IntoModel for Vec<Loc<Attribute<'input>>> {
    type Output = Attributes;

    fn into_model(self, scope: &Scope) -> Result<Attributes> {
        use self::Attribute::*;

        let ctx = scope.ctx();

        let mut words = HashMap::new();
        let mut selections = HashMap::new();

        for attribute in self {
            let (attr, attr_pos) = Loc::take_pair(attribute);

            match attr {
                Word(word) => {
                    let (word, pos) = Loc::take_pair(word.into_model(scope)?);

                    if let Some(old) = words.insert(word, pos.clone()) {
                        return Err(ctx.report()
                            .err(pos, "word already present")
                            .info(old, "old attribute here")
                            .into());
                    }
                }
                List(key, name_values) => {
                    let key = Loc::take(key.into_model(scope)?);

                    match selections.entry(key) {
                        hash_map::Entry::Vacant(entry) => {
                            let mut words = Vec::new();
                            let mut values = HashMap::new();

                            for name_value in name_values {
                                match name_value {
                                    AttributeItem::Word(word) => {
                                        words.push(word.into_model(scope)?);
                                    }
                                    AttributeItem::NameValue { name, value } => {
                                        let name = name.into_model(scope)?;
                                        let value = value.into_model(scope)?;
                                        values.insert(Loc::value(&name).clone(), (name, value));
                                    }
                                }
                            }

                            let selection = Selection::new(words, values);
                            entry.insert(Loc::new(selection, attr_pos));
                        }
                        hash_map::Entry::Occupied(entry) => {
                            return Err(ctx.report()
                                .err(attr_pos, "attribute already present")
                                .info(Loc::pos(entry.get()), "attribute here")
                                .into());
                        }
                    }
                }
            }
        }

        Ok(Attributes::new(words, selections))
    }
}

impl<'input> IntoModel for PathSpec<'input> {
    type Output = RpPathSpec;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        Ok(RpPathSpec {
            steps: self.steps.into_model(scope)?,
        })
    }
}

impl<'input> IntoModel for PathStep<'input> {
    type Output = RpPathStep;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        Ok(RpPathStep {
            parts: self.parts.into_model(scope)?,
        })
    }
}

impl<'input> IntoModel for PathPart<'input> {
    type Output = RpPathPart;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::PathPart::*;

        let out = match self {
            Variable(variable) => RpPathPart::Variable(variable.into_model(scope)?),
            Segment(segment) => RpPathPart::Segment(segment),
        };

        Ok(out)
    }
}
