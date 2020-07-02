mod python2;
mod requests;

pub(crate) use self::python2::{Config as Python2Config, Module as Python2};
pub(crate) use self::requests::{Config as RequestsConfig, Module as Requests};
