pub trait Operator {
    type FirstIter: Iterator<Item = char>;
    type RestIter: Iterator<Item = char>;
    type OpenSection: Iterator<Item = char>;

    fn first_iter(c: char) -> Self::FirstIter;

    fn open_section(c: char) -> Self::OpenSection;

    fn rest_iter(c: char) -> Self::RestIter;

    fn sep() -> &'static str;
}

pub trait Source {
    fn operate<O>(&self, input: &str) -> String
    where
        O: Operator;
}

pub trait Naming {
    fn convert(&self, input: &str) -> String;

    /// Copy the given naming policy.
    fn copy(&self) -> Box<Naming>;
}

#[derive(Clone, Copy)]
pub struct ToLowerCamel(());

impl Naming for ToLowerCamel {
    fn convert(&self, input: &str) -> String {
        operate::<Self>(input)
    }

    fn copy(&self) -> Box<Naming> {
        Box::new(*self)
    }
}

impl Operator for ToLowerCamel {
    type FirstIter = ::std::char::ToLowercase;
    type OpenSection = ::std::char::ToUppercase;
    type RestIter = ::std::char::ToLowercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_lowercase()
    }

    fn open_section(c: char) -> Self::OpenSection {
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
    fn convert(&self, input: &str) -> String {
        operate::<Self>(input)
    }

    fn copy(&self) -> Box<Naming> {
        Box::new(*self)
    }
}

impl Operator for ToUpperCamel {
    type FirstIter = ::std::char::ToUppercase;
    type OpenSection = ::std::char::ToUppercase;
    type RestIter = ::std::char::ToLowercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_uppercase()
    }

    fn open_section(c: char) -> Self::OpenSection {
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
    fn convert(&self, input: &str) -> String {
        operate::<Self>(input)
    }

    fn copy(&self) -> Box<Naming> {
        Box::new(*self)
    }
}

impl Operator for ToLowerSnake {
    type FirstIter = ::std::char::ToLowercase;
    type OpenSection = ::std::char::ToLowercase;
    type RestIter = ::std::char::ToLowercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_lowercase()
    }

    fn open_section(c: char) -> Self::OpenSection {
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
    fn convert(&self, input: &str) -> String {
        operate::<Self>(input)
    }

    fn copy(&self) -> Box<Naming> {
        Box::new(*self)
    }
}

impl Operator for ToUpperSnake {
    type FirstIter = ::std::char::ToUppercase;
    type OpenSection = ::std::char::ToUppercase;
    type RestIter = ::std::char::ToUppercase;

    fn first_iter(c: char) -> Self::FirstIter {
        c.to_uppercase()
    }

    fn open_section(c: char) -> Self::OpenSection {
        c.to_uppercase()
    }

    fn rest_iter(c: char) -> Self::RestIter {
        c.to_uppercase()
    }

    fn sep() -> &'static str {
        "_"
    }
}

/// A source for camel-cased strings.
fn operate<O>(input: &str) -> String
where
    O: Operator,
{
    let mut buf = String::new();

    let mut open_section = true;
    let mut first = true;

    for c in input.chars() {
        match c {
            '_' if first => buf.push('_'),
            '_' => open_section = true,
            c if first => {
                buf.extend(O::first_iter(c));
                first = false;
                open_section = false;
            }
            c if c.is_uppercase() || open_section => {
                buf.push_str(O::sep());
                buf.extend(O::open_section(c));
                open_section = false;
            }
            c => {
                buf.extend(O::rest_iter(c));
            }
        }
    }

    buf
}

pub fn to_lower_camel() -> ToLowerCamel {
    ToLowerCamel(())
}

pub fn to_upper_camel() -> ToUpperCamel {
    ToUpperCamel(())
}

pub fn to_lower_snake() -> ToLowerSnake {
    ToLowerSnake(())
}

pub fn to_upper_snake() -> ToUpperSnake {
    ToUpperSnake(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn preserve_prefix() {
        let naming = to_lower_camel();

        assert_eq!("_fooBarBaz", naming.convert("_FooBarBaz"));
        assert_eq!("_fooBarBaz", naming.convert("_fooBarBaz"));
        assert_eq!("_fooBarBaz", naming.convert("_foo_bar_baz"));
        assert_eq!("_fooBarBaz", naming.convert("_Foo_Bar_baz"));
    }

    #[test]
    pub fn lower_camel_test() {
        let naming = to_lower_camel();

        assert_eq!("fooBarBaz", naming.convert("FooBarBaz"));
        assert_eq!("fooBarBaz", naming.convert("fooBarBaz"));
        assert_eq!("fooBarBaz", naming.convert("foo_bar_baz"));
        assert_eq!("fooBarBaz", naming.convert("Foo_Bar_baz"));
    }

    #[test]
    pub fn upper_camel_test() {
        let naming = to_upper_camel();

        assert_eq!("FooBarBaz", naming.convert("FooBarBaz"));
        assert_eq!("FooBarBaz", naming.convert("fooBarBaz"));
        assert_eq!("FooBBarBaz", naming.convert("fooBBarBaz"));
        assert_eq!("FooBarBaz", naming.convert("foo_bar_baz"));
        assert_eq!("FooBarBaz", naming.convert("foo_Bar_baz"));
    }

    #[test]
    pub fn lower_snake_test() {
        let naming = to_lower_snake();

        assert_eq!("foo_bar_baz", naming.convert("FooBarBaz"));
        assert_eq!("foo_bar_baz", naming.convert("fooBarBaz"));
    }

    #[test]
    pub fn upper_snake_test() {
        let naming = to_upper_snake();

        assert_eq!("FOO_BAR_BAZ", naming.convert("FooBarBaz"));
        assert_eq!("FOO_BAR_BAZ", naming.convert("fooBarBaz"));
        assert_eq!("FOO_BAR_BAZ", naming.convert("foo_bar_baz"));
    }
}
