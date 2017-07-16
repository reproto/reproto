use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum RpDecl {
    Type(Rc<RpTypeBody>),
    Interface(Rc<RpInterfaceBody>),
    Enum(Rc<RpEnumBody>),
    Tuple(Rc<RpTupleBody>),
    Service(Rc<RpServiceBody>),
}

impl RpDecl {
    pub fn name(&self) -> &str {
        match *self {
            RpDecl::Type(ref body) => &body.name,
            RpDecl::Interface(ref body) => &body.name,
            RpDecl::Enum(ref body) => &body.name,
            RpDecl::Tuple(ref body) => &body.name,
            RpDecl::Service(ref body) => &body.name,
        }
    }

    pub fn comment(&self) -> &[String] {
        match *self {
            RpDecl::Type(ref body) => &body.comment,
            RpDecl::Interface(ref body) => &body.comment,
            RpDecl::Enum(ref body) => &body.comment,
            RpDecl::Tuple(ref body) => &body.comment,
            RpDecl::Service(ref body) => &body.comment,
        }
    }

    /// Convert a declaration into its registered types.
    pub fn into_registered_type(&self,
                                package: &RpVersionedPackage,
                                pos: &Pos)
                                -> Vec<(RpTypeId, Loc<RpRegistered>)> {
        match *self {
            RpDecl::Type(ref ty) => {
                let type_id = package.into_type_id(RpName::with_parts(vec![ty.name.clone()]));
                let token = Loc::new(RpRegistered::Type(ty.clone()), pos.clone());
                vec![(type_id, token)]
            }
            RpDecl::Interface(ref interface) => {
                let mut out = Vec::new();

                let current = vec![interface.name.clone()];
                let type_id = RpTypeId::new(package.clone(), RpName::with_parts(current.clone()));
                let token = Loc::new(RpRegistered::Interface(interface.clone()), pos.clone());

                for (name, sub_type) in &interface.sub_types {
                    let sub_type = RpRegistered::SubType {
                        parent: interface.clone(),
                        sub_type: sub_type.as_ref().clone(),
                    };

                    let token = Loc::new(sub_type, pos.clone());

                    let mut current = current.clone();
                    current.push(name.to_owned());
                    out.push((type_id.with_name(RpName::with_parts(current)), token));
                }

                out.push((type_id, token));
                out
            }
            RpDecl::Enum(ref en) => {
                let mut out = Vec::new();

                let current = vec![en.name.clone()];
                let type_id = RpTypeId::new(package.clone(), RpName::with_parts(current.clone()));
                let token = Loc::new(RpRegistered::Enum(en.clone()), pos.clone());

                for variant in &en.variants {
                    let enum_constant = RpRegistered::EnumConstant {
                        parent: en.clone(),
                        variant: variant.as_ref().clone(),
                    };
                    let token = Loc::new(enum_constant, pos.clone());

                    let mut current = current.clone();
                    current.push((*variant.name).to_owned());
                    out.push((type_id.with_name(RpName::with_parts(current)), token));
                }

                out.push((type_id, token));
                out
            }
            RpDecl::Tuple(ref tuple) => {
                let type_id = RpTypeId::new(package.clone(),
                                            RpName::with_parts(vec![tuple.name.clone()]));
                let token = Loc::new(RpRegistered::Tuple(tuple.clone()), pos.clone());
                vec![(type_id, token)]
            }
            RpDecl::Service(ref service) => {
                let type_id = RpTypeId::new(package.clone(),
                                            RpName::with_parts(vec![service.name.clone()]));
                let token = Loc::new(RpRegistered::Service(service.clone()), pos.clone());
                vec![(type_id, token)]
            }
        }
    }
}

impl ::std::fmt::Display for RpDecl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RpDecl::Type(ref body) => write!(f, "type {}", body.name),
            RpDecl::Interface(ref body) => write!(f, "interface {}", body.name),
            RpDecl::Enum(ref body) => write!(f, "enum {}", body.name),
            RpDecl::Tuple(ref body) => write!(f, "tuple {}", body.name),
            RpDecl::Service(ref body) => write!(f, "service {}", body.name),
        }
    }
}

impl Merge for Loc<RpDecl> {
    fn merge(&mut self, source: Loc<RpDecl>) -> Result<()> {
        let dest_pos = self.pos().clone();
        let m = self.as_mut();

        match *m {
            RpDecl::Type(ref mut body) => {
                if let RpDecl::Type(ref other) = *source {
                    return body.merge(other.clone());
                }
            }
            RpDecl::Enum(ref mut body) => {
                if let RpDecl::Enum(ref other) = *source {
                    if let Some(variant) = other.variants.iter().next() {
                        return Err(ErrorKind::ExtendEnum("cannot extend enum with additional \
                                                       variants"
                                                             .to_owned(),
                                                         variant.pos().into(),
                                                         dest_pos.into())
                            .into());
                    }

                    if let Some(field) = other.fields.iter().next() {
                        return Err(ErrorKind::ExtendEnum("cannot extend enum with additional \
                                                          fields"
                                                             .to_owned(),
                                                         field.pos().into(),
                                                         dest_pos.into())
                            .into());
                    }


                    return body.merge(other.clone());
                }
            }
            RpDecl::Interface(ref mut body) => {
                if let RpDecl::Interface(ref other) = *source {
                    return body.merge(other.clone());
                }
            }
            RpDecl::Tuple(ref mut body) => {
                if let RpDecl::Tuple(ref other) = *source {
                    return body.merge(other.clone());
                }
            }
            RpDecl::Service(ref mut body) => {
                if let RpDecl::Service(ref other) = *source {
                    return body.merge(other.clone());
                }
            }
        }

        return Err(ErrorKind::DeclMerge(format!("cannot merge with {}", source),
                                        source.pos().into(),
                                        dest_pos.into())
            .into());
    }
}
