use super::*;
use super::errors::*;

#[derive(Debug)]
pub enum Decl {
    Type(TypeBody),
    Tuple(TupleBody),
    Interface(InterfaceBody),
    Enum(EnumBody),
    Service(ServiceBody),
}

impl IntoModel for Decl {
    type Output = RpDecl;

    fn into_model(self, pos: &RpPos) -> Result<RpDecl> {
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
