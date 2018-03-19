extern crate reproto_core;

use self::Component::*;
use self::Violation::*;
use reproto_core::{ErrorPos, Loc, RpChannel, RpDecl, RpEndpoint, RpField, RpFile, RpName, RpReg,
                   RpType, RpVariant, Version};
use reproto_core::errors::*;
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
    DeclRemoved(Component, ErrorPos),
    /// An entire declaration has been added.
    DeclAdded(Component, ErrorPos),
    /// Field was removed.
    RemoveField(Component, ErrorPos),
    /// Variant was removed.
    RemoveVariant(Component, ErrorPos),
    /// Field added.
    AddField(Component, ErrorPos),
    /// Variant added.
    AddVariant(Component, ErrorPos),
    /// Field type was changed from one to another.
    FieldTypeChange(Component, RpType, ErrorPos, RpType, ErrorPos),
    /// Field name was changed from one to another.
    FieldNameChange(Component, String, ErrorPos, String, ErrorPos),
    /// Variant identifier was changed from one to another.
    VariantOrdinalChange(Component, String, ErrorPos, String, ErrorPos),
    /// Field made required.
    FieldRequiredChange(Component, ErrorPos, ErrorPos),
    /// Required field added.
    AddRequiredField(Component, ErrorPos),
    /// Field modifier changed.
    FieldModifierChange(Component, ErrorPos, ErrorPos),
    /// Endpoint added.
    AddEndpoint(Component, ErrorPos),
    /// Endpoint removed.
    RemoveEndpoint(Component, ErrorPos),
    /// Endpoint request type changed.
    EndpointRequestChange(Component, Option<RpChannel>, ErrorPos, Option<RpChannel>, ErrorPos),
    /// Endpoint response type changed.
    EndpointResponseChange(Component, Option<RpChannel>, ErrorPos, Option<RpChannel>, ErrorPos),
}

fn fields(reg: &RpReg) -> Vec<&Loc<RpField>> {
    use self::RpReg::*;

    match *reg {
        Type(ref target) => target.fields.iter().collect(),
        Tuple(ref target) => target.fields.iter().collect(),
        Interface(ref target) => target.fields.iter().collect(),
        SubType(_, ref target) => target.fields.iter().collect(),
        _ => vec![],
    }
}

fn enum_variants(reg: &RpReg) -> Vec<&Loc<RpVariant>> {
    use self::RpReg::*;

    match *reg {
        Enum(ref target) => target.variants.iter().map(|v| &**v).collect(),
        _ => vec![],
    }
}

fn endpoints_to_map(reg: &RpReg) -> HashMap<&str, &Loc<RpEndpoint>> {
    use self::RpReg::*;

    match *reg {
        Service(ref target) => target.endpoints.iter().map(|e| (e.ident(), e)).collect(),
        _ => HashMap::new(),
    }
}

fn decls_to_map<'a, I: 'a>(decls: I) -> HashMap<RpName, RpReg>
where
    I: IntoIterator<Item = &'a RpDecl>,
{
    let mut storage = HashMap::new();

    for decl in decls {
        for reg in decl.to_reg() {
            // Checked separately for each Enum.
            if let RpReg::EnumVariant(_, _) = reg {
                continue;
            }

            storage.insert(reg.name().clone().localize(), reg);
        }
    }

    storage
}

fn variants_to_map<'a, I: 'a>(variants: I) -> HashMap<RpName, &'a Loc<RpVariant>>
where
    I: IntoIterator<Item = &'a Loc<RpVariant>>,
{
    let mut storage = HashMap::new();

    for variant in variants {
        storage.insert(variant.name.clone().localize(), variant);
    }

    storage
}

fn fields_to_map<'a, I: 'a>(fields: I) -> HashMap<String, &'a Loc<RpField>>
where
    I: IntoIterator<Item = &'a Loc<RpField>>,
{
    let mut storage = HashMap::new();

    for field in fields {
        storage.insert(field.ident().to_string(), field);
    }

    storage
}

/// Perform checks on an endpoint channel.
fn check_endpoint_channel<F, E>(
    component: Component,
    violations: &mut Vec<Violation>,
    from_endpoint: &Loc<RpEndpoint>,
    to_endpoint: &Loc<RpEndpoint>,
    accessor: F,
    error: E,
) -> Result<()>
where
    F: Fn(&RpEndpoint) -> &Option<Loc<RpChannel>>,
    E: Fn(Component,
       Option<RpChannel>,
       ErrorPos,
       Option<RpChannel>,
       ErrorPos)
       -> Violation,
{
    let from_ty = accessor(from_endpoint).as_ref().map(|r| {
        (r.is_streaming(), r.ty().clone().localize())
    });

    let to_ty = accessor(to_endpoint).as_ref().map(|r| {
        (r.is_streaming(), r.ty().clone().localize())
    });

    if from_ty != to_ty {
        let from_pos = accessor(from_endpoint)
            .as_ref()
            .map(|r| Loc::pos(r))
            .unwrap_or(Loc::pos(from_endpoint));

        let to_pos = accessor(to_endpoint)
            .as_ref()
            .map(|r| Loc::pos(r))
            .unwrap_or(Loc::pos(to_endpoint));

        violations.push(error(
            component,
            accessor(from_endpoint).as_ref().map(Loc::value).map(
                Clone::clone,
            ),
            from_pos.into(),
            accessor(to_endpoint).as_ref().map(Loc::value).map(
                Clone::clone,
            ),
            to_pos.into(),
        ));
    }

    Ok(())
}

fn check_endpoint_type(
    component: Component,
    violations: &mut Vec<Violation>,
    from_endpoint: &Loc<RpEndpoint>,
    to_endpoint: &Loc<RpEndpoint>,
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
    from_variant: &Loc<RpVariant>,
    to_variant: &Loc<RpVariant>,
) -> Result<()> {
    if from_variant.ordinal() != to_variant.ordinal() {
        violations.push(VariantOrdinalChange(
            component.clone(),
            from_variant.ordinal().to_string(),
            Loc::pos(from_variant).into(),
            to_variant.ordinal().to_string(),
            Loc::pos(to_variant).into(),
        ));
    }

    Ok(())
}

fn common_check_field(
    component: Component,
    violations: &mut Vec<Violation>,
    from_field: &Loc<RpField>,
    to_field: &Loc<RpField>,
) -> Result<()> {
    if to_field.ty.clone().localize() != from_field.ty.clone().localize() {
        violations.push(FieldTypeChange(
            component.clone(),
            from_field.ty.clone(),
            Loc::pos(from_field).into(),
            to_field.ty.clone(),
            Loc::pos(to_field).into(),
        ));
    }

    // not permitted to rename fields.
    if to_field.name() != from_field.name() {
        violations.push(FieldNameChange(
            component.clone(),
            from_field.name().to_string(),
            Loc::pos(from_field).into(),
            to_field.name().to_string(),
            Loc::pos(to_field).into(),
        ));
    }

    Ok(())
}

/// Performs checks for minor version violations.
fn check_minor(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let from_storage = decls_to_map(&from.decls);
    let mut to_storage = decls_to_map(&to.decls);

    for (name, from_reg) in from_storage {
        if let Some(to_reg) = to_storage.remove(&name) {
            let from_fields = fields_to_map(fields(&from_reg));
            let mut to_fields = fields_to_map(fields(&to_reg));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    check_field(&mut violations, from_field, to_field)?;
                } else {
                    violations.push(RemoveField(Minor, Loc::pos(from_field).into()));
                }
            }

            // check that added fields are not required.
            for (_, to_field) in to_fields.into_iter() {
                if to_field.is_required() {
                    violations.push(AddRequiredField(Minor, Loc::pos(to_field).into()));
                }
            }

            let from_variants = variants_to_map(enum_variants(&from_reg));
            let mut to_variants = variants_to_map(enum_variants(&to_reg));

            for (name, from_variant) in from_variants.into_iter() {
                if let Some(to_variant) = to_variants.remove(&name) {
                    check_variant(&mut violations, from_variant, to_variant)?;
                } else {
                    violations.push(RemoveVariant(Minor, Loc::pos(from_variant).into()));
                }
            }

            let from_endpoints = endpoints_to_map(&from_reg);
            let mut to_endpoints = endpoints_to_map(&to_reg);

            for (name, from_endpoint) in from_endpoints.into_iter() {
                if let Some(to_endpoint) = to_endpoints.remove(&name) {
                    check_endpoint(&mut violations, from_endpoint, to_endpoint)?;
                } else {
                    violations.push(RemoveEndpoint(Minor, Loc::pos(from_endpoint).into()));
                }
            }
        } else {
            violations.push(DeclRemoved(Minor, from_reg.pos().into()));
        }
    }

    return Ok(violations);

    fn check_field(
        violations: &mut Vec<Violation>,
        from_field: &Loc<RpField>,
        to_field: &Loc<RpField>,
    ) -> Result<()> {
        common_check_field(Minor, violations, from_field, to_field)?;

        // Minor patch may make fields optional, but not required.
        if from_field.is_optional() && to_field.is_required() {
            violations.push(FieldRequiredChange(
                Minor,
                Loc::pos(from_field).into(),
                Loc::pos(to_field).into(),
            ));
        }

        Ok(())
    }

    fn check_variant(
        violations: &mut Vec<Violation>,
        from_variant: &Loc<RpVariant>,
        to_variant: &Loc<RpVariant>,
    ) -> Result<()> {
        common_check_variant(Minor, violations, from_variant, to_variant)?;
        Ok(())
    }

    fn check_endpoint(
        violations: &mut Vec<Violation>,
        from_endpoint: &Loc<RpEndpoint>,
        to_endpoint: &Loc<RpEndpoint>,
    ) -> Result<()> {
        check_endpoint_type(Minor, violations, from_endpoint, to_endpoint)?;
        Ok(())
    }
}

fn check_patch(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let from_storage = decls_to_map(&from.decls);
    let mut to_storage = decls_to_map(&to.decls);

    for (name, from_reg) in from_storage {
        if let Some(to_reg) = to_storage.remove(&name) {
            let from_fields = fields_to_map(fields(&from_reg));
            let mut to_fields = fields_to_map(fields(&to_reg));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    check_field(&mut violations, from_field, to_field)?;
                } else {
                    violations.push(RemoveField(Patch, Loc::pos(from_field).into()));
                }
            }

            // added fields are not permitted
            for (_, to_field) in to_fields.into_iter() {
                violations.push(AddField(Patch, Loc::pos(to_field).into()));
            }

            let from_variants = variants_to_map(enum_variants(&from_reg));
            let mut to_variants = variants_to_map(enum_variants(&to_reg));

            for (name, from_variant) in from_variants.into_iter() {
                if let Some(to_variant) = to_variants.remove(&name) {
                    check_variant(&mut violations, from_variant, to_variant)?;
                } else {
                    violations.push(RemoveVariant(Patch, Loc::pos(from_variant).into()));
                }
            }

            // added variants are not permitted
            for (_, to_variant) in to_variants.into_iter() {
                violations.push(AddVariant(Patch, Loc::pos(to_variant).into()));
            }

            let from_endpoints = endpoints_to_map(&from_reg);
            let mut to_endpoints = endpoints_to_map(&to_reg);

            for (name, from_endpoint) in from_endpoints.into_iter() {
                if let Some(to_endpoint) = to_endpoints.remove(&name) {
                    check_endpoint(&mut violations, from_endpoint, to_endpoint)?;
                } else {
                    violations.push(RemoveEndpoint(Patch, Loc::pos(from_endpoint).into()));
                }
            }

            // added endpoints are not permitted
            for (_, to_endpoint) in to_endpoints.into_iter() {
                violations.push(AddEndpoint(Patch, Loc::pos(to_endpoint).into()));
            }
        } else {
            violations.push(DeclRemoved(Patch, from_reg.pos().into()));
        }
    }

    for (_, to_reg) in to_storage.into_iter() {
        violations.push(DeclAdded(Patch, to_reg.pos().into()));
    }

    return Ok(violations);

    fn check_field(
        violations: &mut Vec<Violation>,
        from_field: &Loc<RpField>,
        to_field: &Loc<RpField>,
    ) -> Result<()> {
        common_check_field(Patch, violations, from_field, to_field)?;

        if to_field.required != from_field.required {
            violations.push(FieldModifierChange(
                Patch,
                Loc::pos(from_field).into(),
                Loc::pos(to_field).into(),
            ));
        }

        Ok(())
    }

    fn check_variant(
        violations: &mut Vec<Violation>,
        from_variant: &Loc<RpVariant>,
        to_variant: &Loc<RpVariant>,
    ) -> Result<()> {
        common_check_variant(Patch, violations, from_variant, to_variant)?;
        Ok(())
    }

    fn check_endpoint(
        violations: &mut Vec<Violation>,
        from_endpoint: &Loc<RpEndpoint>,
        to_endpoint: &Loc<RpEndpoint>,
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
