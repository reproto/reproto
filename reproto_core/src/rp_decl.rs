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

    pub fn comment(&self) -> &Vec<String> {
        match *self {
            RpDecl::Type(ref body) => &body.comment,
            RpDecl::Interface(ref body) => &body.comment,
            RpDecl::Enum(ref body) => &body.comment,
            RpDecl::Tuple(ref body) => &body.comment,
            RpDecl::Service(ref body) => &body.comment,
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

impl Merge for RpLoc<RpDecl> {
    fn merge(&mut self, source: RpLoc<RpDecl>) -> Result<()> {
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
