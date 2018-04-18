use std::borrow::Cow;
use std::env;
use std::ffi;
use std::fmt;
use std::result;
use std::sync::atomic;
use {Span, WithSpan};

const RUST_BACKTRACE: &str = "RUST_BACKTRACE";

#[cfg(all(feature = "backtrace", feature = "std"))]
mod internal {
    pub const HAS_BACKTRACE: bool = true;

    pub use backtrace::Backtrace;
}

#[cfg(not(all(feature = "backtrace", feature = "std")))]
mod internal {
    pub const HAS_BACKTRACE: bool = false;

    use std::fmt;

    /// Fake internal representation.
    pub struct Backtrace(());

    impl Backtrace {
        pub fn new() -> Backtrace {
            Backtrace(())
        }
    }

    impl fmt::Debug for Backtrace {
        fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
}

pub use self::internal::{Backtrace, HAS_BACKTRACE};

pub type Result<T> = result::Result<T, Error>;

/// Extra convenience functions for results based on core errors.
pub trait ResultExt<T>
where
    Self: Sized,
{
    fn chain_err<C, D>(self, chain: C) -> Result<T>
    where
        C: FnOnce() -> D,
        D: Into<Error>;
}

impl<T> ResultExt<T> for result::Result<T, Error> {
    fn chain_err<C, D>(self, chain: C) -> Result<T>
    where
        C: FnOnce() -> D,
        D: Into<Error>,
    {
        match self {
            Err(e) => {
                let mut new = chain().into();
                new.cause = Some(Box::new(e));
                Err(new)
            }
            Ok(value) => Ok(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Causes<'a> {
    current: Option<&'a Error>,
}

impl<'a> Iterator for Causes<'a> {
    type Item = &'a Error;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.current {
            self.current = e.cause();
            return Some(e);
        }

        None
    }
}

/// Error display type.
///
/// Since Error can't implement fmt::Display, this acts as a proxy.
pub struct Display<'a> {
    message: Cow<'a, str>,
}

impl<'a> fmt::Display for Display<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.message.as_ref())
    }
}

fn is_backtrace_enabled<F: Fn(&str) -> Option<ffi::OsString>>(get_var: F) -> bool {
    match get_var(RUST_BACKTRACE) {
        Some(ref val) if val != "0" => true,
        _ => false,
    }
}

/// The kind of error that has been raised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Regular error that must not be ignored.
    Regular,
    /// Error has been reported to context and can be ignored.
    Context,
}

pub struct Error {
    message: Cow<'static, str>,
    kind: ErrorKind,
    span: Option<Span>,
    cause: Option<Box<Error>>,
    suppressed: Vec<Error>,
    backtrace: Option<Backtrace>,
}

impl Error {
    pub fn new<M: Into<Cow<'static, str>>>(message: M) -> Self {
        Self {
            message: message.into(),
            kind: ErrorKind::Regular,
            span: None,
            cause: None,
            suppressed: Vec::new(),
            backtrace: Self::new_backtrace(),
        }
    }

    /// Build a new error that has been constructed from a context.
    /// 
    /// These errors can safely be ignored, and will be removed as soon as we no longer
    /// have spans in errors.
    /// 
    /// All spanned issues should be reported through the context.
    pub fn new_context<M: Into<Cow<'static, str>>>(message: M) -> Self {
        Self {
            message: message.into(),
            kind: ErrorKind::Context,
            span: None,
            cause: None,
            suppressed: Vec::new(),
            backtrace: Self::new_backtrace(),
        }
    }

    /// Check if this is a context error.
    pub fn is_context(&self) -> bool {
        self.kind == ErrorKind::Context
    }

    fn new_backtrace() -> Option<Backtrace> {
        if !HAS_BACKTRACE {
            return None;
        }

        static ENABLED: atomic::AtomicUsize = atomic::ATOMIC_USIZE_INIT;

        match ENABLED.load(atomic::Ordering::SeqCst) {
            0 => {
                let enabled = is_backtrace_enabled(|var| env::var_os(var));

                ENABLED.store(enabled as usize + 1, atomic::Ordering::SeqCst);

                if !enabled {
                    return None;
                }
            }
            1 => return None,
            _ => {}
        }

        return Some(Backtrace::new());
    }

    /// Set the position for this error.
    pub fn with_span<P: Into<Span>>(self, span: P) -> Error {
        Error {
            span: Some(span.into()),
            ..self
        }
    }

    /// Set the position for this error.
    pub fn with_suppressed<S: IntoIterator<Item = Error>>(self, suppressed: S) -> Error {
        Error {
            suppressed: suppressed.into_iter().collect(),
            ..self
        }
    }

    /// Convert errro into a type that is `fmt::Display`.
    ///
    /// WARNING: drops error information. Only use if absolutely necessary!
    pub fn display(&self) -> Display {
        Display {
            message: Cow::from(self.message.as_ref()),
        }
    }

    /// Get backtrace.
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }

    /// Get the message for the error.
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    /// Extract the error position, if available.
    pub fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    /// Get the cause of this error.
    pub fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(AsRef::as_ref)
    }

    /// Get all suppressed errors.
    pub fn suppressed(&self) -> Vec<&Error> {
        self.suppressed.iter().collect()
    }

    /// Iterate over all causes.
    pub fn causes(&self) -> Causes {
        Causes {
            current: Some(self),
        }
    }
}

impl<T> From<T> for Error
where
    T: fmt::Display,
{
    fn from(value: T) -> Error {
        Error::new(value.to_string())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Error")
            .field("message", &self.message)
            .finish()
    }
}

impl WithSpan for Error {
    fn with_span<E: Into<Span>>(self, span: E) -> Self {
        if self.span.is_some() {
            return self;
        }

        Self {
            span: Some(span.into()),
            ..self
        }
    }
}
