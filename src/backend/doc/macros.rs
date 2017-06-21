use std::fmt;

pub trait FormatAttribute {
    fn format_attribute(&self, f: &mut fmt::Write) -> fmt::Result;
}

impl<T> FormatAttribute for Vec<T>
    where T: FormatAttribute
{
    fn format_attribute(&self, f: &mut fmt::Write) -> fmt::Result {
        let mut it = self.iter().peekable();

        while let Some(next) = it.next() {
            next.format_attribute(f)?;

            if !it.peek().is_none() {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

impl<'a> FormatAttribute for &'a str {
    fn format_attribute(&self, f: &mut fmt::Write) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FormatAttribute for String {
    fn format_attribute(&self, f: &mut fmt::Write) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[macro_export]
macro_rules! html {
    (@open $out:ident, $element:ident {$($key:ident => $value:expr),*}) => {{
        write!($out, "<{}", stringify!($element))?;
        $(
            write!($out, " {}=\"", stringify!($key))?;
            $value.format_attribute($out)?;
            write!($out, "\"")?;
        )*
        write!($out, ">")?;
    }};

    ($out:ident, $element:ident {$($key:ident => $value:expr),*} => $body:block) => {{
        html!(@open $out, $element {$($key=> $value),*});
        $body;
        write!($out, "</{}>", stringify!($element))?;
    }};

    ($out:ident, $element:ident {$($key:ident => $value:expr),*} ~ $body:expr) => {
        html!($out, $element {$($key=> $value),*} => {
            write!($out, "{}", $body)?
        })
    };

    ($out:ident, $element:ident {$($key:ident => $value:expr),*}) => {
        html!($element {$($key=> $value),*}, $out => {})
    };

    ($out:ident, $element:ident $body:expr) => {
        html!($element {} $body)
    };
}
