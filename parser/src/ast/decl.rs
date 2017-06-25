use super::*;
use super::errors::*;

#[derive(Debug)]
pub enum Decl<'input> {
    Type(TypeBody<'input>),
    Tuple(TupleBody<'input>),
    Interface(InterfaceBody<'input>),
    Enum(EnumBody<'input>),
    Service(ServiceBody<'input>),
}

impl<'input> IntoModel for Decl<'input> {
    type Output = RpDecl;

    fn into_model(self) -> Result<RpDecl> {
        let decl = match self {
            Decl::Type(body) => RpDecl::Type(body.into_model()?),
            Decl::Interface(body) => RpDecl::Interface(body.into_model()?),
            Decl::Enum(body) => RpDecl::Enum(body.into_model()?),
            Decl::Tuple(body) => RpDecl::Tuple(body.into_model()?),
            Decl::Service(body) => RpDecl::Service(body.into_model()?),
        };

        Ok(decl)
    }
}
