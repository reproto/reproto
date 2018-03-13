//! Specialization for Swift code generation.

use genco::{Custom, Formatter, Tokens};
use std::borrow::Cow;
use std::fmt::{self, Write};

static SEP: &'static str = ".";

/// Swift token specialization.
#[derive(Debug, Clone)]
pub enum Swift<'el> {
    /// An imported name.
    Imported {
        /// Module of the imported name.
        module: Cow<'el, str>,
        /// Name imported.
        name: Cow<'el, str>,
    },
}

impl<'el> Swift<'el> {}

impl<'el> Custom for Swift<'el> {
    type Extra = ();

    fn format(&self, out: &mut Formatter, _extra: &mut Self::Extra, _level: usize) -> fmt::Result {
        use self::Swift::*;

        match *self {
            Imported {
                ref module,
                ref name,
                ..
            } => {
                out.write_str(module)?;
                out.write_str(SEP)?;
                out.write_str(name)?;
            }
        }

        Ok(())
    }

    fn quote_string(out: &mut Formatter, input: &str) -> fmt::Result {
        out.write_char('"')?;

        for c in input.chars() {
            match c {
                '\t' => out.write_str("\\t")?,
                '\n' => out.write_str("\\n")?,
                '\r' => out.write_str("\\r")?,
                '\'' => out.write_str("\\'")?,
                '"' => out.write_str("\\\"")?,
                '\\' => out.write_str("\\\\")?,
                c => out.write_char(c)?,
            };
        }

        out.write_char('"')?;
        Ok(())
    }

    fn write_file<'a>(
        tokens: Tokens<'a, Self>,
        out: &mut Formatter,
        extra: &mut Self::Extra,
        level: usize,
    ) -> fmt::Result {
        let mut toks: Tokens<Self> = Tokens::new();
        toks.push_ref(&tokens);
        toks.join_line_spacing().format(out, extra, level)
    }
}

/// Setup an imported element.
pub fn imported<'a, M, N>(module: M, name: N) -> Swift<'a>
where
    M: Into<Cow<'a, str>>,
    N: Into<Cow<'a, str>>,
{
    Swift::Imported {
        module: module.into(),
        name: name.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{imported, Swift};
    use genco::{Quoted, Tokens};

    #[test]
    fn test_string() {
        let mut toks: Tokens<Swift> = Tokens::new();
        toks.append("hello \n world".quoted());
        let res = toks.to_string();

        assert_eq!(Ok("\"hello \\n world\""), res.as_ref().map(|s| s.as_str()));
    }

    #[test]
    fn test_imported() {
        let dbg = imported("Foo", "Debug");
        let mut toks: Tokens<Swift> = Tokens::new();
        toks.push(toks!(&dbg));

        assert_eq!(
            Ok("Foo.Debug\n"),
            toks.to_file().as_ref().map(|s| s.as_str())
        );
    }
}
