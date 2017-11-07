extern crate reproto_core;

use self::Violation::*;
use reproto_core::{Loc, RpDecl, RpField, RpFile, RpName, RpRegistered, RpType, Version};
use reproto_core::errors::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Violation {
    /// An entire declaration has been removed.
    MinorDeclRemoved(RpRegistered),
    /// Field was removed.
    MinorRemoveField(Loc<RpField>),
    /// Field type was changed from one to another.
    MinorFieldTypeChange(Loc<RpType>, Loc<RpType>),
    /// Required field added.
    MinorAddRequiredField(Loc<RpField>),
    /// An entire declaration has been removed.
    PatchDeclRemoved(RpRegistered),
    /// Field was removed.
    PatchRemoveField(Loc<RpField>),
    /// Field type was changed from one to another.
    PatchFieldTypeChange(Loc<RpType>, Loc<RpType>),
    /// Field added.
    PatchAddField(Loc<RpField>),
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
        storage.insert(field.name.clone(), field);
    }

    storage
}

/// Performs checks for minor version violations.
fn check_minor(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let to_storage = decls_to_map(&from.decls);
    let from_storage = decls_to_map(&to.decls);

    // Minot chenot permitted to remove declarations

    for (name, to_reg) in to_storage {
        if let Some(from_reg) = from_storage.get(&name) {
            let from_fields = fields_to_map(fields(&from_reg));
            let mut to_fields = fields_to_map(fields(&to_reg));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    if to_field.ty.clone().without_version() !=
                        from_field.ty.clone().without_version()
                    {
                        let from_ty = from_field.clone().map(|f| f.ty);
                        let to_ty = to_field.clone().map(|f| f.ty);
                        violations.push(MinorFieldTypeChange(from_ty, to_ty));
                    }
                } else {
                    violations.push(MinorRemoveField(from_field.clone()));
                }
            }

            // check that added fields are optional
            for (_, to_field) in to_fields.into_iter() {
                if to_field.is_required() {
                    violations.push(MinorAddRequiredField(to_field.clone()));
                }
            }
        } else {
            violations.push(MinorDeclRemoved(to_reg));
        }
    }

    Ok(violations)
}

fn check_patch(from: &RpFile, to: &RpFile) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    let to_storage = decls_to_map(&from.decls);
    let from_storage = decls_to_map(&to.decls);

    // Minot chenot permitted to remove declarations

    for (name, to_reg) in to_storage {
        if let Some(from_reg) = from_storage.get(&name) {
            let from_fields = fields_to_map(fields(&from_reg));
            let mut to_fields = fields_to_map(fields(&to_reg));

            for (name, from_field) in from_fields.into_iter() {
                if let Some(to_field) = to_fields.remove(&name) {
                    if to_field.ty.clone().without_version() !=
                        from_field.ty.clone().without_version()
                    {
                        let from_ty = from_field.clone().map(|f| f.ty);
                        let to_ty = to_field.clone().map(|f| f.ty);
                        violations.push(PatchFieldTypeChange(from_ty, to_ty));
                    }
                } else {
                    violations.push(PatchRemoveField(from_field.clone()));
                }
            }

            // check that added fields are optional
            for (_, to_field) in to_fields.into_iter() {
                violations.push(PatchAddField(to_field.clone()));
            }
        } else {
            violations.push(PatchDeclRemoved(to_reg));
        }
    }

    Ok(violations)
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
