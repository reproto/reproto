//! Naming conversion utilities
//!
//! These have been carefully designed to be object-safe, so that you can
//! dynamically configure which utility to use.

use std::char;
use std::fmt;
use std::fmt::Write as _;

type FormatFn = fn(&mut fmt::Formatter<'_>, input: &str) -> fmt::Result;

/// The display implementation for the given naming.
///
/// Creating using [display()][Naming::display].
#[derive(Clone, Copy)]
pub struct Display<'a> {
    input: &'a str,
    internal_format: fn(&mut fmt::Formatter<'_>, &str) -> fmt::Result,
}

impl<'a> fmt::Display for Display<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.internal_format)(fmt, self.input)
    }
}

impl<'a> fmt::Debug for Display<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Display")
            .field("input", &self.input)
            .finish()
    }
}

pub trait Naming {
    /// Build the internal formatting function.
    fn internal_format(&self) -> FormatFn;

    /// Clone the given naming policy.
    fn clone_box(&self) -> Box<dyn Naming>;

    /// Convert the given string.
    fn convert(&self, input: &str) -> String {
        self.display(input).to_string()
    }

    /// Display the given string in-place.
    fn display<'a>(&self, input: &'a str) -> Display<'a> {
        Display {
            input,
            internal_format: self.internal_format(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ToLowerCamel(());

impl Naming for ToLowerCamel {
    fn internal_format(&self) -> FormatFn {
        internal_format::<Self>
    }

    fn clone_box(&self) -> Box<dyn Naming> {
        Box::new(*self)
    }
}

impl InternalFormat for ToLowerCamel {
    type FirstIter = char::ToLowercase;
    type OpenSectionIter = char::ToUppercase;
    type RestIter = char::ToLowercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_lowercase()
    }

    fn open_section_iter(c: char) -> Self::OpenSectionIter {
        c.to_uppercase()
    }

    fn rest_iter(c: char) -> Self::RestIter {
        c.to_lowercase()
    }

    fn sep() -> &'static str {
        ""
    }
}

#[derive(Clone, Copy)]
pub struct ToUpperCamel(());

impl Naming for ToUpperCamel {
    fn internal_format(&self) -> FormatFn {
        internal_format::<Self>
    }

    fn clone_box(&self) -> Box<dyn Naming> {
        Box::new(*self)
    }
}

impl InternalFormat for ToUpperCamel {
    type FirstIter = char::ToUppercase;
    type OpenSectionIter = char::ToUppercase;
    type RestIter = char::ToLowercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_uppercase()
    }

    fn open_section_iter(c: char) -> Self::OpenSectionIter {
        c.to_uppercase()
    }

    fn rest_iter(c: char) -> Self::RestIter {
        c.to_lowercase()
    }

    fn sep() -> &'static str {
        ""
    }
}

#[derive(Clone, Copy)]
pub struct ToLowerSnake(());

impl Naming for ToLowerSnake {
    fn internal_format(&self) -> FormatFn {
        internal_format::<Self>
    }

    fn clone_box(&self) -> Box<dyn Naming> {
        Box::new(*self)
    }
}

impl InternalFormat for ToLowerSnake {
    type FirstIter = char::ToLowercase;
    type OpenSectionIter = char::ToLowercase;
    type RestIter = char::ToLowercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_lowercase()
    }

    fn open_section_iter(c: char) -> Self::OpenSectionIter {
        c.to_lowercase()
    }

    fn rest_iter(c: char) -> Self::RestIter {
        c.to_lowercase()
    }

    fn sep() -> &'static str {
        "_"
    }
}

#[derive(Clone, Copy)]
pub struct ToUpperSnake(());

impl Naming for ToUpperSnake {
    fn internal_format(&self) -> FormatFn {
        internal_format::<Self>
    }

    fn clone_box(&self) -> Box<dyn Naming> {
        Box::new(*self)
    }
}

impl InternalFormat for ToUpperSnake {
    type FirstIter = char::ToUppercase;
    type OpenSectionIter = char::ToUppercase;
    type RestIter = char::ToUppercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_uppercase()
    }

    fn open_section_iter(c: char) -> Self::OpenSectionIter {
        c.to_uppercase()
    }

    fn rest_iter(c: char) -> Self::RestIter {
        c.to_uppercase()
    }

    fn sep() -> &'static str {
        "_"
    }
}

/// Trait to implement for type which which can be used with [internal_format()].
trait InternalFormat {
    type FirstIter: Iterator<Item = char>;
    type OpenSectionIter: Iterator<Item = char>;
    type RestIter: Iterator<Item = char>;

    /// Iterate over the first character.
    fn first_iter(c: char) -> Self::FirstIter;

    /// Iterate over the open section.
    fn open_section_iter(c: char) -> Self::OpenSectionIter;

    fn rest_iter(c: char) -> Self::RestIter;

    /// Separator to put between each block.
    fn sep() -> &'static str;
}

/// A source for camel-cased strings.
fn internal_format<O>(fmt: &mut fmt::Formatter<'_>, input: &str) -> fmt::Result
where
    O: InternalFormat,
{
    let mut open_section = true;
    let mut first = true;

    for c in input.chars() {
        match c {
            '_' if first => {
                fmt.write_char('_')?;
            }
            '_' => {
                open_section = true;
            }
            c if first => {
                for c in O::first_iter(c) {
                    fmt.write_char(c)?;
                }

                first = false;
                open_section = false;
            }
            c if c.is_uppercase() || open_section => {
                fmt.write_str(O::sep())?;

                for c in O::open_section_iter(c) {
                    fmt.write_char(c)?;
                }

                open_section = false;
            }
            c => {
                for c in O::rest_iter(c) {
                    fmt.write_char(c)?;
                }
            }
        }
    }

    Ok(())
}

/// Construct a naming policy that convert values into lower camel case.
///
/// # Examples
///
/// ```rust
/// use reproto_naming::Naming as _;
///
/// let naming = reproto_naming::to_lower_camel();
///
/// assert_eq!("fooBarBaz", naming.convert("FooBarBaz"));
/// assert_eq!("fooBarBaz", naming.convert("fooBarBaz"));
/// assert_eq!("fooBarBaz", naming.convert("foo_bar_baz"));
/// assert_eq!("fooBarBaz", naming.convert("Foo_Bar_baz"));
///
/// assert_eq!("_fooBarBaz", naming.convert("_FooBarBaz"));
/// assert_eq!("_fooBarBaz", naming.convert("_fooBarBaz"));
/// assert_eq!("_fooBarBaz", naming.convert("_foo_bar_baz"));
/// assert_eq!("_fooBarBaz", naming.convert("_Foo_Bar_baz"));
/// ```
pub fn to_lower_camel() -> ToLowerCamel {
    ToLowerCamel(())
}

/// Construct a naming policy that convert values into upper camel case.
///
/// # Examples
///
/// ```rust
/// use reproto_naming::Naming as _;
///
/// let naming = reproto_naming::to_upper_camel();
///
/// assert_eq!("FooBarBaz", naming.convert("FooBarBaz"));
/// assert_eq!("FooBarBaz", naming.convert("fooBarBaz"));
/// assert_eq!("FooBBarBaz", naming.convert("fooBBarBaz"));
/// assert_eq!("FooBarBaz", naming.convert("foo_bar_baz"));
/// assert_eq!("FooBarBaz", naming.convert("foo_Bar_baz"));
/// ```
pub fn to_upper_camel() -> ToUpperCamel {
    ToUpperCamel(())
}

/// Construct a naming policy that convert values into lower snake case.
///
/// # Examples
///
/// ```rust
/// use reproto_naming::Naming as _;
///
/// let naming = reproto_naming::to_lower_snake();
///
/// assert_eq!("foo_bar_baz", naming.convert("FooBarBaz"));
/// assert_eq!("foo_bar_baz", naming.convert("fooBarBaz"));
/// ```
pub fn to_lower_snake() -> ToLowerSnake {
    ToLowerSnake(())
}

/// Construct a naming policy that convert values into upper snake case.
///
/// # Examples
///
/// ```rust
/// use reproto_naming::Naming as _;
///
/// let naming = reproto_naming::to_upper_snake();
///
/// assert_eq!("FOO_BAR_BAZ", naming.convert("FooBarBaz"));
/// assert_eq!("FOO_BAR_BAZ", naming.convert("fooBarBaz"));
/// assert_eq!("FOO_BAR_BAZ", naming.convert("foo_bar_baz"));
/// ```
pub fn to_upper_snake() -> ToUpperSnake {
    ToUpperSnake(())
}
