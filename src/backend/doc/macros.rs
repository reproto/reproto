pub trait FormatAttribute {
    fn format_attribute(&self) -> String;
}

impl<T> FormatAttribute for Vec<T>
    where T: FormatAttribute
{
    fn format_attribute(&self) -> String {
        let mut out = String::new();

        let mut it = self.iter().peekable();

        while let Some(next) = it.next() {
            out.push_str(&next.format_attribute());

            if !it.peek().is_none() {
                out.push_str(" ");
            }
        }

        out
    }
}

impl<'a> FormatAttribute for &'a str {
    fn format_attribute(&self) -> String {
        (*self).to_owned()
    }
}

impl FormatAttribute for String {
    fn format_attribute(&self) -> String {
        self.clone()
    }
}

#[macro_export]
macro_rules! html {
    (@open $out:ident, $element:ident {$($key:ident => $value:expr),*}) => {{
        write!($out, "<{}", stringify!($element))?;
        $(
            write!($out, " {}=\"", stringify!($key))?;
            $out.write_str(&$value.format_attribute())?;
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
