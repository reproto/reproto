//! Data model for request or responses for endpoints

use crate::errors::Result;
use crate::{Diagnostics, Flavor, Translate, Translator};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(bound = "F::Type: serde::Serialize")]
pub enum RpChannel<F>
where
    F: Flavor,
{
    /// Single send.
    Unary { ty: F::Type },
    /// Multiple sends.
    Streaming { ty: F::Type },
}

impl<F> RpChannel<F>
where
    F: Flavor,
{
    /// Get the type of the channel.
    pub fn ty(&self) -> &F::Type {
        use self::RpChannel::*;

        match *self {
            Unary { ref ty, .. } | Streaming { ref ty, .. } => ty,
        }
    }

    /// Check if channel is streaming.
    pub fn is_streaming(&self) -> bool {
        use self::RpChannel::*;

        match *self {
            Unary { .. } => false,
            Streaming { .. } => true,
        }
    }
}

impl<F> fmt::Display for RpChannel<F>
where
    F: Flavor,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.is_streaming() {
            write!(fmt, "stream {:?}", self.ty())
        } else {
            write!(fmt, "{:?}", self.ty())
        }
    }
}

impl<T> Translate<T> for RpChannel<T::Source>
where
    T: Translator,
{
    type Out = RpChannel<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpChannel<T::Target>> {
        use self::RpChannel::*;

        let out = match self {
            Unary { ty } => Unary {
                ty: translator.translate_type(diag, ty)?,
            },
            Streaming { ty } => Streaming {
                ty: translator.translate_type(diag, ty)?,
            },
        };

        Ok(out)
    }
}
