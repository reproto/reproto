use genco::lang::Python;
use genco::tokens::{FormatInto, ItemStr};
use genco::Tokens;
use std::fmt;

pub struct BlockComment<I>(pub I);

impl<I> FormatInto<Python> for BlockComment<I>
where
    I: IntoIterator,
    I::Item: Into<ItemStr>,
{
    fn format_into(self, out: &mut Tokens<Python>) {
        let mut it = self.0.into_iter().peekable();

        if !it.peek().is_some() {
            return;
        }

        out.append(ItemStr::Static("\"\"\""));

        while let Some(line) = it.next() {
            out.push();
            out.append(line.into());
        }

        out.push();
        out.append(ItemStr::Static("\"\"\""));
    }
}

/// Utilities for handling differences between python versions.
pub trait VersionHelper: fmt::Debug {
    /// Check if the given variable is a string.
    ///
    /// In Python 3, strings are `str` objects.
    /// In Python 2, strings would be `unicode` objects.
    fn is_string(&self, var: &ItemStr) -> Tokens<Python>;
}
