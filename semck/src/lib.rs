extern crate reproto_core;

use self::Component::*;
use self::Violation::*;
use reproto_core::{ErrorPos, Loc, RpDecl, RpField, RpFile, RpName, RpRegistered, RpType, Version};
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
    /// Field was removed.
    RemoveField(Component, ErrorPos),
    /// Field added.
    AddField(Component, ErrorPos),
    /// Field type was changed from one to another.
    FieldTypeChange(Component, RpType, ErrorPos, RpType, ErrorPos),
    /// Field name was changed from one to another.
    FieldNameChange(Component, String, ErrorPos, String, ErrorPos),
    /// Field identifier was changed from one to another.
    FieldIdentifierChange(Component, String, ErrorPos, String, ErrorPos),
    /// Field made required.
    FieldRequiredChange(Component, ErrorPos, ErrorPos),
    /// Required field added.
    AddRequiredField(Component, ErrorPos),
    /// Field modifier changed.
    FieldModifierChange(Component, ErrorPos, ErrorPos),
}

fn fields(reg: &RpRegistered) -> Vec<&Loc<RpField>> {
    use self::RpRegistered::*;

    match *reg {
        Type(ref target) => target.fields.iter().collect(),
        Tuple(ref target) => target.fields.iter().collect(),
        Interface(ref target) => target.fields.iter().collect(),
        SubType(_, ref target) => target.fields.iter().collect(),
        _ => vec![],
    }
}

fn decls_to_map<'a, I: 'a>(decls: I) -> HashMap<RpName, RpRegistered>
where
    I: IntoIterator<Item = &'a Loc<RpDecl>>,
{
    let mut storage = HashMap::new();

    for decl in decls {
        for reg in decl.into_registered_type() {
            storage.insert(reg.name().clone().without_version(), reg);
        }
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

/// Performs checks for minor version violations.
fn check_minor(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let from_storage = decls_to_map(&from.decls);
    let to_storage = decls_to_map(&to.decls);

    // Minot chenot permitted to remove declarations

    for (name, to_reg) in to_storage {
        if let Some(from_reg) = from_storage.get(&name) {
            let from_fields = fields_to_map(fields(&from_reg));
            let mut to_fields = fields_to_map(fields(&to_reg));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    check_field(&mut violations, from_field, to_field)?;
                } else {
                    violations.push(RemoveField(Minor, from_field.pos().into()));
                }
            }

            // check that added fields are optional
            for (_, to_field) in to_fields.into_iter() {
                if to_field.is_required() {
                    violations.push(AddRequiredField(Minor, to_field.pos().into()));
                }
            }
        } else {
            violations.push(DeclRemoved(Minor, to_reg.pos().into()));
        }
    }

    return Ok(violations);

    fn check_field(
        violations: &mut Vec<Violation>,
        from_field: &Loc<RpField>,
        to_field: &Loc<RpField>,
    ) -> Result<()> {
        if to_field.ty.clone().without_version() != from_field.ty.clone().without_version() {
            violations.push(FieldTypeChange(
                Minor,
                from_field.ty.clone(),
                from_field.pos().into(),
                to_field.ty.clone(),
                to_field.pos().into(),
            ));
        }

        // not permitted to rename fields.
        if to_field.name() != from_field.name() {
            violations.push(FieldNameChange(
                Minor,
                from_field.name().to_string(),
                from_field.pos().into(),
                to_field.name().to_string(),
                to_field.pos().into(),
            ));
        }

        if to_field.ident() != from_field.ident() {
            violations.push(FieldIdentifierChange(
                Minor,
                from_field.ident().to_string(),
                from_field.pos().into(),
                to_field.ident().to_string(),
                to_field.pos().into(),
            ));
        }

        // not permitted to make fields required.
        if from_field.is_optional() && to_field.is_required() {
            violations.push(FieldRequiredChange(
                Minor,
                from_field.pos().into(),
                to_field.pos().into(),
            ));
        }

        Ok(())
    }
}

fn check_patch(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let from_storage = decls_to_map(&from.decls);
    let to_storage = decls_to_map(&to.decls);

    // Minot chenot permitted to remove declarations

    for (name, to_reg) in to_storage {
        if let Some(from_reg) = from_storage.get(&name) {
            let from_fields = fields_to_map(fields(&from_reg));
            let mut to_fields = fields_to_map(fields(&to_reg));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    check_field(&mut violations, from_field, to_field)?;
                } else {
                    violations.push(RemoveField(Patch, from_field.pos().into()));
                }
            }

            // added fields are not permitted
            for (_, to_field) in to_fields.into_iter() {
                violations.push(AddField(Patch, to_field.pos().into()));
            }
        } else {
            violations.push(DeclRemoved(Patch, to_reg.pos().into()));
        }
    }

    return Ok(violations);

    fn check_field(
        violations: &mut Vec<Violation>,
        from_field: &Loc<RpField>,
        to_field: &Loc<RpField>,
    ) -> Result<()> {
        if to_field.ty.clone().without_version() != from_field.ty.clone().without_version() {
            violations.push(FieldTypeChange(
                Patch,
                from_field.ty.clone(),
                from_field.pos().into(),
                to_field.ty.clone(),
                to_field.pos().into(),
            ));
        }

        if to_field.name() != from_field.name() {
            violations.push(FieldNameChange(
                Patch,
                from_field.name().to_string(),
                from_field.pos().into(),
                to_field.name().to_string(),
                to_field.pos().into(),
            ));
        }

        if to_field.ident() != from_field.ident() {
            violations.push(FieldIdentifierChange(
                Minor,
                from_field.ident().to_string(),
                from_field.pos().into(),
                to_field.ident().to_string(),
                to_field.pos().into(),
            ));
        }

        if to_field.modifier != from_field.modifier {
            violations.push(FieldModifierChange(
                Patch,
                from_field.pos().into(),
                to_field.pos().into(),
            ));
        }

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
