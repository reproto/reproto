//! Data model for request or responses for endpoints

use std::fmt;
use std::result;
use {Diagnostics, Flavor, Loc, Translate, Translator};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(bound = "F::Type: ::serde::Serialize")]
pub enum RpChannel<F: 'static>
where
    F: Flavor,
{
    /// Single send.
    Unary { ty: Loc<F::Type> },
    /// Multiple sends.
    Streaming { ty: Loc<F::Type> },
}

impl<F: 'static> RpChannel<F>
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

impl<F: 'static> fmt::Display for RpChannel<F>
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

impl<F: 'static, T> Translate<T> for RpChannel<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = RpChannel<T::Target>;

    /// Translate into different flavor.
    fn translate(
        self,
        diag: &mut Diagnostics,
        translator: &T,
    ) -> result::Result<RpChannel<T::Target>, ()> {
        use self::RpChannel::*;

        let out = match self {
            Unary { ty } => {
                let (ty, span) = Loc::take_pair(ty);
                let ty = try_diag!(diag, span, translator.translate_type(diag, ty));
                let ty = Loc::new(ty, span);
                Unary { ty }
            }
            Streaming { ty } => {
                let (ty, span) = Loc::take_pair(ty);
                let ty = try_diag!(diag, span, translator.translate_type(diag, ty));
                let ty = Loc::new(ty, span);
                Streaming { ty }
            }
        };

        Ok(out)
    }
}
