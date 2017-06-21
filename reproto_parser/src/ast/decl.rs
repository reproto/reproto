use super::*;
use super::errors::*;

#[derive(Debug)]
pub enum Decl<'a> {
    Type(TypeBody<'a>),
    Tuple(TupleBody<'a>),
    Interface(InterfaceBody<'a>),
    Enum(EnumBody<'a>),
    Service(ServiceBody<'a>),
}

impl<'a> IntoModel for Decl<'a> {
    type Output = RpDecl;

    fn into_model(self, pos: &Path) -> Result<RpDecl> {
        let decl = match self {
            Decl::Type(body) => RpDecl::Type(body.into_model(pos)?),
            Decl::Interface(body) => RpDecl::Interface(body.into_model(pos)?),
            Decl::Enum(body) => RpDecl::Enum(body.into_model(pos)?),
            Decl::Tuple(body) => RpDecl::Tuple(body.into_model(pos)?),
            Decl::Service(body) => RpDecl::Service(body.into_model(pos)?),
        };

        Ok(decl)
    }
}
