pub trait FormatAttribute {
    fn format_attribute(&self) -> String;
}

impl<T> FormatAttribute for Vec<T>
where
    T: FormatAttribute,
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
macro_rules! define_processor {
    ($name:ident, $body:ty, $slf:ident, $($tail:tt)*) => (
        pub struct $name<'env> {
            pub out: ::std::cell::RefCell<DocBuilder<'env>>,
            pub env: &'env Environment,
            pub syntax: (&'env ::syntect::highlighting::Theme, &'env ::syntect::parsing::SyntaxSet),
            pub root: &'env str,
            pub body: &'env $body,
        }

        impl<'env> Processor<'env> for $name<'env> {
            fn env(&self) -> &'env Environment {
                self.env
            }

            fn out(&self) -> ::std::cell::RefMut<DocBuilder<'env>> {
                self.out.borrow_mut()
            }

            fn root(&self) -> &'env str {
                self.root
            }

            fn syntax(&self) -> (&'env ::syntect::highlighting::Theme, &'env ::syntect::parsing::SyntaxSet) {
                self.syntax
            }

            define_processor!(@tail $slf $($tail)*);
        }
    );

    (@tail $slf:ident process => $body:block; $($tail:tt)*) => (
        fn process($slf) -> Result<()> $body

        define_processor!(@tail $slf $($tail)*);
    );

    (@tail $slf:ident current_package => $body:block; $($tail:tt)*) => (
        fn current_package(&$slf) -> Option<&'env ::core::RpVersionedPackage> $body

        define_processor!(@tail $slf $($tail)*);
    );

    (@tail $slf:ident current_package => $expr:expr; $($tail:tt)*) => (
        fn current_package(&$slf) -> Option<&'env ::core::RpVersionedPackage> { Some($expr) }

        define_processor!(@tail $slf $($tail)*);
    );

    (@tail $slf:ident) => ();
}

#[macro_export]
macro_rules! html {
    (@open $slf:ident, $element:ident {$($key:ident => $value:expr),*}) => {{
        write!($slf.out(), "<{}", stringify!($element))?;
        $(
            write!($slf.out(), " {}=\"", stringify!($key))?;
            $slf.out().write_str(&$value.format_attribute())?;
            write!($slf.out(), "\"")?;
        )*
        write!($slf.out(), ">")?;
    }};

    (@close $slf:ident, $element:ident) => {{
        write!($slf.out(), "</{}>", stringify!($element))?;
    }};

    ($slf:ident, $element:ident {$($key:ident => $value:expr),*} => $body:block) => {{
        html!(@open $slf, $element {$($key=> $value),*});
        $slf.out().new_line()?;
        $slf.out().indent();
        $body;
        $slf.out().new_line_unless_empty()?;
        $slf.out().unindent();
        html!(@close $slf, $element);
        $slf.out().new_line()?;
    }};

    ($slf:ident, $element:ident {$($key:ident => $value:expr),*} ~ $body:expr) => {{
        html!(@open $slf, $element {$($key=> $value),*});
        write!($slf.out(), "{}", $body)?;
        html!(@close $slf, $element);
        $slf.out().new_line()?;
    }};

    ($slf:ident, $element:ident {$($key:ident => $value:expr),*}) => {
        html!($element {$($key=> $value),*}, $slf => {})
    };

    ($slf:ident, $element:ident $body:expr) => {
        html!($element {} $body)
    };
}
