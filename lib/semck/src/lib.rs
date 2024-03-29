use self::Component::*;
use self::Violation::*;
use reproto_core::errors::Result;
use reproto_core::flavored::*;
use reproto_core::{Span, Spanned, Version};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Component {
    Minor,
    Patch,
}

impl Component {
    /// Describe the component that was violated.
    pub fn describe(&self) -> &str {
        match *self {
            Minor => "minor change violation",
            Patch => "patch change violation",
        }
    }
}

#[derive(Debug)]
pub enum Violation {
    /// An entire declaration has been removed.
    DeclRemoved(Component, Span),
    /// An entire declaration has been added.
    DeclAdded(Component, Span),
    /// Field was removed.
    RemoveField(Component, Span),
    /// Variant was removed.
    RemoveVariant(Component, Span),
    /// Field added.
    AddField(Component, Span),
    /// Variant added.
    AddVariant(Component, Span),
    /// Field type was changed from one to another.
    FieldTypeChange(Component, RpType, Span, RpType, Span),
    /// Field name was changed from one to another.
    FieldNameChange(Component, String, Span, String, Span),
    /// Variant identifier was changed from one to another.
    VariantOrdinalChange(Component, String, Span, String, Span),
    /// Field made required.
    FieldRequiredChange(Component, Span, Span),
    /// Required field added.
    AddRequiredField(Component, Span),
    /// Field modifier changed.
    FieldModifierChange(Component, Span, Span),
    /// Endpoint added.
    AddEndpoint(Component, Span),
    /// Endpoint removed.
    RemoveEndpoint(Component, Span),
    /// Endpoint request type changed.
    EndpointRequestChange(Component, Option<RpChannel>, Span, Option<RpChannel>, Span),
    /// Endpoint response type changed.
    EndpointResponseChange(Component, Option<RpChannel>, Span, Option<RpChannel>, Span),
}

fn fields<'a>(named: &RpNamed<'a>) -> Vec<&'a Spanned<RpField>> {
    match *named {
        RpNamed::Type(target) => target.fields.iter().collect(),
        RpNamed::Tuple(target) => target.fields.iter().collect(),
        RpNamed::Interface(target) => target.fields.iter().collect(),
        RpNamed::SubType(target) => target.fields.iter().collect(),
        _ => vec![],
    }
}

fn enum_variants<'a>(named: &'a RpNamed) -> Vec<RpVariantRef<'a>> {
    match *named {
        RpNamed::Enum(target) => target.variants.iter().collect(),
        _ => vec![],
    }
}

fn endpoints_to_map<'a>(named: &RpNamed<'a>) -> HashMap<&'a str, &'a Spanned<RpEndpoint>> {
    match *named {
        RpNamed::Service(target) => target.endpoints.iter().map(|e| (e.ident(), e)).collect(),
        _ => HashMap::new(),
    }
}

fn decls_to_map<'a, I>(decls: I) -> HashMap<RpName, RpNamed<'a>>
where
    I: IntoIterator<Item = &'a RpDecl>,
{
    let mut storage = HashMap::new();

    for decl in decls {
        for named in decl.to_named() {
            // Checked separately for each Enum.
            if let RpNamed::EnumVariant(_) = named {
                continue;
            }

            storage.insert(Spanned::borrow(named.name()).clone().localize(), named);
        }
    }

    storage
}

fn variants_to_map<'a, I>(variants: I) -> HashMap<RpName, RpVariantRef<'a>>
where
    I: IntoIterator<Item = RpVariantRef<'a>>,
{
    let mut storage = HashMap::new();

    for variant in variants {
        storage.insert(Spanned::borrow(&variant.name).clone().localize(), variant);
    }

    storage
}

fn fields_to_map<'a, I>(fields: I) -> HashMap<String, &'a Spanned<RpField>>
where
    I: IntoIterator<Item = &'a Spanned<RpField>>,
{
    let mut storage = HashMap::new();

    for field in fields {
        storage.insert(field.ident.clone(), field);
    }

    storage
}

/// Perform checks on an endpoint channel.
fn check_endpoint_channel<F, E>(
    component: Component,
    violations: &mut Vec<Violation>,
    from_endpoint: &Spanned<RpEndpoint>,
    to_endpoint: &Spanned<RpEndpoint>,
    accessor: F,
    error: E,
) -> Result<()>
where
    F: Fn(&RpEndpoint) -> &Option<Spanned<RpChannel>>,
    E: Fn(Component, Option<RpChannel>, Span, Option<RpChannel>, Span) -> Violation,
{
    let from_ty = accessor(from_endpoint)
        .as_ref()
        .map(|r| (r.is_streaming(), r.ty().clone().localize()));

    let to_ty = accessor(to_endpoint)
        .as_ref()
        .map(|r| (r.is_streaming(), r.ty().clone().localize()));

    if from_ty != to_ty {
        let from_pos = accessor(from_endpoint)
            .as_ref()
            .map(|r| r.span())
            .unwrap_or(from_endpoint.span());

        let to_pos = accessor(to_endpoint)
            .as_ref()
            .map(|r| r.span())
            .unwrap_or(to_endpoint.span());

        violations.push(error(
            component,
            accessor(from_endpoint)
                .as_ref()
                .map(Spanned::borrow)
                .map(Clone::clone),
            from_pos.into(),
            accessor(to_endpoint)
                .as_ref()
                .map(Spanned::borrow)
                .map(Clone::clone),
            to_pos.into(),
        ));
    }

    Ok(())
}

fn check_endpoint_type(
    component: Component,
    violations: &mut Vec<Violation>,
    from_endpoint: &Spanned<RpEndpoint>,
    to_endpoint: &Spanned<RpEndpoint>,
) -> Result<()> {
    // TODO: check arguments.
    /*check_endpoint_channel(
        component.clone(),
        violations,
        from_endpoint,
        to_endpoint,
        |e| &e.request,
        EndpointRequestChange,
    )?;*/

    check_endpoint_channel(
        component.clone(),
        violations,
        from_endpoint,
        to_endpoint,
        |e| &e.response,
        EndpointResponseChange,
    )?;

    Ok(())
}

fn common_check_variant(
    component: Component,
    violations: &mut Vec<Violation>,
    from_variant: RpVariantRef,
    to_variant: RpVariantRef,
) -> Result<()> {
    if from_variant.value != to_variant.value {
        violations.push(VariantOrdinalChange(
            component.clone(),
            from_variant.to_string(),
            from_variant.span.into(),
            to_variant.to_string(),
            to_variant.span.into(),
        ));
    }

    Ok(())
}

fn common_check_field(
    component: Component,
    violations: &mut Vec<Violation>,
    from_field: &Spanned<RpField>,
    to_field: &Spanned<RpField>,
) -> Result<()> {
    if to_field.ty.clone().localize() != from_field.ty.clone().localize() {
        violations.push(FieldTypeChange(
            component.clone(),
            from_field.ty.clone(),
            from_field.span().into(),
            to_field.ty.clone(),
            to_field.span().into(),
        ));
    }

    // not permitted to rename fields.
    if to_field.name() != from_field.name() {
        violations.push(FieldNameChange(
            component.clone(),
            from_field.name().to_string(),
            from_field.span().into(),
            to_field.name().to_string(),
            to_field.span().into(),
        ));
    }

    Ok(())
}

/// Performs checks for minor version violations.
fn check_minor(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let from_storage = decls_to_map(&from.decls);
    let mut to_storage = decls_to_map(&to.decls);

    for (name, from_named) in from_storage {
        if let Some(to_named) = to_storage.remove(&name) {
            let from_fields = fields_to_map(fields(&from_named));
            let mut to_fields = fields_to_map(fields(&to_named));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    check_field(&mut violations, from_field, to_field)?;
                } else {
                    violations.push(RemoveField(Minor, from_field.span().into()));
                }
            }

            // check that added fields are not required.
            for (_, to_field) in to_fields.into_iter() {
                if to_field.is_required() {
                    violations.push(AddRequiredField(Minor, to_field.span().into()));
                }
            }

            let from_variants = variants_to_map(enum_variants(&from_named));
            let mut to_variants = variants_to_map(enum_variants(&to_named));

            for (name, from_variant) in from_variants.into_iter() {
                if let Some(to_variant) = to_variants.remove(&name) {
                    check_variant(&mut violations, from_variant, to_variant)?;
                } else {
                    violations.push(RemoveVariant(Minor, from_variant.span.into()));
                }
            }

            let from_endpoints = endpoints_to_map(&from_named);
            let mut to_endpoints = endpoints_to_map(&to_named);

            for (name, from_endpoint) in from_endpoints.into_iter() {
                if let Some(to_endpoint) = to_endpoints.remove(&name) {
                    check_endpoint(&mut violations, from_endpoint, to_endpoint)?;
                } else {
                    violations.push(RemoveEndpoint(Minor, from_endpoint.span().into()));
                }
            }
        } else {
            violations.push(DeclRemoved(Minor, from_named.span().into()));
        }
    }

    return Ok(violations);

    fn check_field(
        violations: &mut Vec<Violation>,
        from_field: &Spanned<RpField>,
        to_field: &Spanned<RpField>,
    ) -> Result<()> {
        common_check_field(Minor, violations, from_field, to_field)?;

        // Minor patch may make fields optional, but not required.
        if from_field.is_optional() && to_field.is_required() {
            violations.push(FieldRequiredChange(
                Minor,
                from_field.span().into(),
                to_field.span().into(),
            ));
        }

        Ok(())
    }

    fn check_variant(
        violations: &mut Vec<Violation>,
        from_variant: RpVariantRef,
        to_variant: RpVariantRef,
    ) -> Result<()> {
        common_check_variant(Minor, violations, from_variant, to_variant)?;
        Ok(())
    }

    fn check_endpoint(
        violations: &mut Vec<Violation>,
        from_endpoint: &Spanned<RpEndpoint>,
        to_endpoint: &Spanned<RpEndpoint>,
    ) -> Result<()> {
        check_endpoint_type(Minor, violations, from_endpoint, to_endpoint)?;
        Ok(())
    }
}

fn check_patch(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let from_storage = decls_to_map(&from.decls);
    let mut to_storage = decls_to_map(&to.decls);

    for (name, from_named) in from_storage {
        if let Some(to_named) = to_storage.remove(&name) {
            let from_fields = fields_to_map(fields(&from_named));
            let mut to_fields = fields_to_map(fields(&to_named));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    check_field(&mut violations, from_field, to_field)?;
                } else {
                    violations.push(RemoveField(Patch, from_field.span().into()));
                }
            }

            // added fields are not permitted
            for (_, to_field) in to_fields.into_iter() {
                violations.push(AddField(Patch, to_field.span().into()));
            }

            let from_variants = variants_to_map(enum_variants(&from_named));
            let mut to_variants = variants_to_map(enum_variants(&to_named));

            for (name, from_variant) in from_variants.into_iter() {
                if let Some(to_variant) = to_variants.remove(&name) {
                    check_variant(&mut violations, from_variant, to_variant)?;
                } else {
                    violations.push(RemoveVariant(Patch, from_variant.span.into()));
                }
            }

            // added variants are not permitted
            for (_, to_variant) in to_variants.into_iter() {
                violations.push(AddVariant(Patch, to_variant.span.into()));
            }

            let from_endpoints = endpoints_to_map(&from_named);
            let mut to_endpoints = endpoints_to_map(&to_named);

            for (name, from_endpoint) in from_endpoints.into_iter() {
                if let Some(to_endpoint) = to_endpoints.remove(&name) {
                    check_endpoint(&mut violations, from_endpoint, to_endpoint)?;
                } else {
                    violations.push(RemoveEndpoint(Patch, from_endpoint.span().into()));
                }
            }

            // added endpoints are not permitted
            for (_, to_endpoint) in to_endpoints.into_iter() {
                violations.push(AddEndpoint(Patch, to_endpoint.span().into()));
            }
        } else {
            violations.push(DeclRemoved(Patch, from_named.span().into()));
        }
    }

    for (_, to_named) in to_storage.into_iter() {
        violations.push(DeclAdded(Patch, to_named.span().into()));
    }

    return Ok(violations);

    fn check_field(
        violations: &mut Vec<Violation>,
        from_field: &Spanned<RpField>,
        to_field: &Spanned<RpField>,
    ) -> Result<()> {
        common_check_field(Patch, violations, from_field, to_field)?;

        if to_field.required != from_field.required {
            violations.push(FieldModifierChange(
                Patch,
                from_field.span().into(),
                to_field.span().into(),
            ));
        }

        Ok(())
    }

    fn check_variant(
        violations: &mut Vec<Violation>,
        from_variant: RpVariantRef,
        to_variant: RpVariantRef,
    ) -> Result<()> {
        common_check_variant(Patch, violations, from_variant, to_variant)?;
        Ok(())
    }

    fn check_endpoint(
        violations: &mut Vec<Violation>,
        from_endpoint: &Spanned<RpEndpoint>,
        to_endpoint: &Spanned<RpEndpoint>,
    ) -> Result<()> {
        check_endpoint_type(Patch, violations, from_endpoint, to_endpoint)?;
        Ok(())
    }
}

pub fn check(from: (&Version, &RpFile), to: (&Version, &RpFile)) -> Result<Vec<Violation>> {
    let (from_version, from_file) = from;
    let (to_version, to_file) = to;

    if from_version.major == to_version.major {
        if from_version.minor < to_version.minor {
            return check_minor(from_file, to_file);
        }

        if from_version.patch < to_version.patch {
            return check_patch(from_file, to_file);
        }
    }

    Ok(vec![])
}
