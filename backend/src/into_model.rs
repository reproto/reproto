use super::errors::*;
use super::scope::Scope;
pub use core::*;
use linked_hash_map::LinkedHashMap;
pub use parser::ast::*;
use std::collections::{BTreeMap, HashMap, HashSet, hash_map};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Adds a method for all types that supports conversion into core types.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, scope: &Scope) -> Result<Self::Output>;
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
            Name { name } => RpType::Name { name: name.into_model(scope)? },
            Array { inner } => RpType::Array { inner: inner.into_model(scope)? },
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

impl<'input> IntoModel for EnumBody<'input> {
    type Output = RpEnumBody;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let mut variants: Vec<Rc<Loc<RpVariant>>> = Vec::new();

        let (fields, codes, _options, decls) = members_into_model(scope, self.members)?;

        if fields.len() > 0 {
            return Err("enums can't have fields".into());
        }

        let ty = self.ty.into_model(scope)?;

        let variant_type = if let Some(ty) = ty {
            ty.and_then(|ty| {
                ty.as_enum_type().ok_or_else(
                    || "expected string or absent".into(),
                ) as Result<RpEnumType>
            })?
        } else {
            RpEnumType::Generated
        };

        for variant in self.variants {
            let (variant, pos) = variant.take_pair();

            let variant = (variant, &variant_type).into_model(scope).with_pos(&pos)?;

            if let Some(other) = variants.iter().find(
                |v| *v.local_name == *variant.local_name,
            )
            {
                return Err(
                    ErrorKind::EnumVariantConflict(
                        other.local_name.pos().into(),
                        variant.local_name.pos().into(),
                    ).into(),
                );
            }

            variants.push(Rc::new(Loc::new(variant, pos)));
        }

        Ok(RpEnumBody {
            name: scope.as_name(),
            local_name: self.name.to_string(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            decls: decls,
            variant_type: variant_type,
            variants: variants,
            codes: codes,
        })
    }
}

/// enum value with assigned ordinal
impl<'input, 'a> IntoModel for (EnumVariant<'input>, &'a RpEnumType) {
    type Output = RpVariant;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (variant, ty) = self;

        let ordinal = if let Some(argument) = variant.argument.into_model(scope)? {
            if !ty.is_assignable_from(&argument) {
                return Err(
                    format!("unexpected value {}, expected type {}", argument, ty).into(),
                );
            }

            argument.and_then(|value| value.to_ordinal())?
        } else {
            RpEnumOrdinal::Generated
        };

        Ok(RpVariant {
            name: scope.as_name().push(variant.name.to_string()),
            local_name: variant.name.clone().map(str::to_string),
            comment: variant.comment.into_iter().map(ToOwned::to_owned).collect(),
            ordinal: ordinal,
        })
    }
}

impl<'input> IntoModel for Field<'input> {
    type Output = RpField;

    fn into_model(self, scope: &Scope) -> Result<RpField> {
        let name = &self.name;

        let field_as = self.field_as.into_model(scope)?.or_else(|| {
            scope.field_naming().map(|n| n.convert(name))
        });

        Ok(RpField {
            modifier: self.modifier,
            name: self.name.to_string(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            ty: self.ty.into_model(scope)?,
            field_as: field_as,
        })
    }
}

impl<'input> IntoModel for File<'input> {
    type Output = RpFile;

    fn into_model(self, scope: &Scope) -> Result<RpFile> {
        let options = self.options.into_model(scope)?;

        let mut decls = Vec::new();

        for decl in self.decls {
            decls.push(Rc::new(decl.into_model(scope)?));
        }

        Ok(RpFile {
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            options: options,
            decls: decls,
        })
    }
}

impl<'input> IntoModel for InterfaceBody<'input> {
    type Output = RpInterfaceBody;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use std::collections::btree_map::Entry::*;

        let (fields, codes, _options, decls) = members_into_model(scope, self.members)?;

        let mut sub_types: BTreeMap<String, Rc<Loc<RpSubType>>> = BTreeMap::new();

        for sub_type in self.sub_types {
            let (sub_type, pos) = sub_type.take_pair();
            let sub_type = Rc::new(Loc::new(sub_type.into_model(scope)?, pos));

            // key has to be owned by entry
            let key = sub_type.local_name.clone();

            match sub_types.entry(key) {
                Vacant(entry) => entry.insert(sub_type),
                Occupied(entry) => {
                    return Err(
                        ErrorKind::Pos(
                            format!("sub-type `{}` already defined", sub_type.local_name),
                            entry.get().pos().into(),
                        ).into(),
                    );
                }
            };
        }

        Ok(RpInterfaceBody {
            name: scope.as_name(),
            local_name: self.name.to_string(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            decls: decls,
            fields: fields,
            codes: codes,
            sub_types: sub_types,
        })
    }
}

/// Generic implementation for vectors.
impl<T> IntoModel for Loc<T>
where
    T: IntoModel,
{
    type Output = Loc<T::Output>;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (value, pos) = self.take_pair();
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

impl<'a> IntoModel for &'a str {
    type Output = String;

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
        Ok(self.to_owned())
    }
}

impl IntoModel for String {
    type Output = String;

    fn into_model(self, _scope: &Scope) -> Result<Self::Output> {
        Ok(self)
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
                        return Err(ErrorKind::MissingPrefix(prefix.clone()).into());
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
            name: self.name.to_owned(),
            value: self.value.into_model(scope)?,
        };

        Ok(decl)
    }
}

impl<'input> IntoModel for PathSegment<'input> {
    type Output = RpPathSegment;

    fn into_model(self, scope: &Scope) -> Result<RpPathSegment> {
        let out = match self {
            PathSegment::Literal { value } => RpPathSegment::Literal {
                value: value.into_model(scope)?,
            },
            PathSegment::Variable { name, ty } => {
                RpPathSegment::Variable {
                    name: name.into_model(scope)?,
                    ty: ty.into_model(scope)?,
                }
            }
        };

        Ok(out)
    }
}

impl<'input> IntoModel for PathSpec<'input> {
    type Output = RpPathSpec;

    fn into_model(self, scope: &Scope) -> Result<RpPathSpec> {
        Ok(RpPathSpec { segments: self.segments.into_model(scope)? })
    }
}

impl<'input> IntoModel for ServiceBody<'input> {
    type Output = RpServiceBody;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use linked_hash_map::Entry::*;

        let mut endpoint_names: HashMap<String, ErrorPos> = HashMap::new();
        let mut endpoints = LinkedHashMap::new();

        for endpoint in self.endpoints {
            let endpoint = endpoint.into_model(scope)?;

            // Check that there are no conflicting endpoint names.
            match endpoint_names.entry(endpoint.name().to_string()) {
                hash_map::Entry::Vacant(entry) => entry.insert(endpoint.pos().into()),
                hash_map::Entry::Occupied(entry) => {
                    return Err(
                        ErrorKind::EndpointNameConflict(
                            endpoint.pos().into(),
                            entry.get().clone_error_pos(),
                        ).into(),
                    );
                }
            };

            // Check that there are no conflicting endpoint IDs.
            match endpoints.entry(endpoint.id.value().to_string()) {
                Vacant(entry) => entry.insert(endpoint),
                Occupied(entry) => {
                    return Err(
                        ErrorKind::EndpointConflict(
                            endpoint.pos().into(),
                            entry.get().pos().into(),
                        ).into(),
                    );
                }
            };
        }

        // TODO: check for duplicate endpoints.
        return Ok(RpServiceBody {
            name: scope.as_name(),
            local_name: self.name.to_string(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            endpoints: endpoints,
            decls: vec![],
        });
    }
}

impl<'input> IntoModel for Endpoint<'input> {
    type Output = RpEndpoint;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let id = self.id.into_model(scope)?;

        let name = self.alias.into_model(scope)?.unwrap_or_else(|| {
            scope
                .endpoint_naming()
                .map(|n| n.convert(id.as_str()))
                .unwrap_or_else(|| id.to_string())
        });

        return Ok(RpEndpoint {
            id: id,
            name: name,
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            request: self.request.into_model(scope)?,
            response: self.response.into_model(scope)?,
        });
    }
}

impl<'input> IntoModel for Channel {
    type Output = RpChannel;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::Channel::*;

        let result = match self {
            Unary { ty, .. } => RpChannel::Unary { ty: ty.into_model(scope)? },
            Streaming { ty, .. } => RpChannel::Streaming { ty: ty.into_model(scope)? },
        };

        Ok(result)
    }
}

impl<'input> IntoModel for SubType<'input> {
    type Output = RpSubType;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        use self::Member::*;

        let mut fields: Vec<Loc<RpField>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();
        let mut decls = Vec::new();

        for member in self.members {
            let (member, pos) = member.take_pair();

            match member {
                Field(field) => {
                    let field = field.into_model(scope)?;

                    if let Some(other) = fields.iter().find(|f| {
                        f.name() == field.name() || f.ident() == field.ident()
                    })
                    {
                        return Err(
                            ErrorKind::FieldConflict(
                                field.ident().to_owned(),
                                pos.into(),
                                other.pos().into(),
                            ).into(),
                        );
                    }

                    fields.push(Loc::new(field, pos));
                }
                Code(context, lines) => {
                    codes.push(code(pos, context.to_owned(), lines));
                }
                Option(option) => {
                    options.push(Loc::new(option.into_model(scope)?, pos));
                }
                InnerDecl(decl) => {
                    decls.push(Rc::new(Loc::new(decl.into_model(scope)?, pos)));
                }
            }
        }

        let names = options.find_all_strings("name")?;

        let comment = self.comment.into_iter().map(ToOwned::to_owned).collect();

        Ok(RpSubType {
            name: scope.as_name().push(self.name.to_string()),
            local_name: self.name.to_string(),
            comment: comment,
            decls: decls,
            fields: fields,
            codes: codes,
            names: names,
        })
    }
}

impl<'input> IntoModel for TupleBody<'input> {
    type Output = RpTupleBody;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (fields, codes, _options, decls) = members_into_model(scope, self.members)?;

        Ok(RpTupleBody {
            name: scope.as_name(),
            local_name: self.name.to_string(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            decls: decls,
            fields: fields,
            codes: codes,
        })
    }
}

impl<'input> IntoModel for TypeBody<'input> {
    type Output = RpTypeBody;

    fn into_model(self, scope: &Scope) -> Result<Self::Output> {
        let (fields, codes, options, decls) = members_into_model(scope, self.members)?;

        let reserved: HashSet<Loc<String>> = options
            .find_all_identifiers("reserved")?
            .into_iter()
            .collect();

        Ok(RpTypeBody {
            name: scope.as_name(),
            local_name: self.name.to_string(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            decls: decls,
            fields: fields,
            codes: codes,
            reserved: reserved,
        })
    }
}

type Fields = Vec<Loc<RpField>>;
type Codes = Vec<Loc<RpCode>>;
type OptionVec = Vec<Loc<RpOptionDecl>>;

pub fn code<'input>(pos: Pos, context: String, lines: Vec<&'input str>) -> Loc<RpCode> {
    let code = RpCode {
        context: context,
        lines: lines.into_iter().map(ToString::to_string).collect(),
    };

    Loc::new(code, pos)
}

pub fn members_into_model(
    scope: &Scope,
    members: Vec<Loc<Member>>,
) -> Result<(Fields, Codes, OptionVec, Vec<Rc<Loc<RpDecl>>>)> {
    use self::Member::*;

    let mut fields: Vec<Loc<RpField>> = Vec::new();
    let mut codes = Vec::new();
    let mut options: Vec<Loc<RpOptionDecl>> = Vec::new();
    let mut decls = Vec::new();

    for member in members {
        let (value, pos) = member.take_pair();

        match value {
            Field(field) => {
                let field = field.into_model(scope)?;

                if let Some(other) = fields.iter().find(|f| {
                    f.name() == field.name() || f.ident() == field.ident()
                })
                {
                    return Err(
                        ErrorKind::FieldConflict(
                            field.ident().to_owned(),
                            pos.into(),
                            other.pos().into(),
                        ).into(),
                    );
                }

                fields.push(Loc::new(field, pos));
            }
            Code(context, lines) => {
                codes.push(code(pos.into(), context.to_owned(), lines));
            }
            Option(option) => {
                options.push(Loc::new(option.into_model(scope)?, pos));
            }
            InnerDecl(decl) => {
                decls.push(Rc::new(Loc::new(decl.into_model(scope)?, pos)));
            }
        }
    }

    Ok((fields, codes, options, decls))
}

impl<'input> IntoModel for Value<'input> {
    type Output = RpValue;

    fn into_model(self, scope: &Scope) -> Result<RpValue> {
        use self::Value::*;

        let out = match self {
            String(string) => RpValue::String(string),
            Number(number) => RpValue::Number(number),
            Boolean(boolean) => RpValue::Boolean(boolean),
            Identifier(identifier) => RpValue::Identifier(identifier.to_owned()),
            Array(inner) => RpValue::Array(inner.into_model(scope)?),
        };

        Ok(out)
    }
}
