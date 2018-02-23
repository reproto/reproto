pub trait SourceOperator {
    fn first(&self, c: char) -> Box<Iterator<Item = char>>;

    fn new_section(&self, c: char) -> Box<Iterator<Item = char>> {
        self.rest(c)
    }

    fn rest(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(Some(c).into_iter())
    }

    fn join(&self, input: Vec<String>) -> String;
}

pub trait Source {
    fn operate<O>(&self, input: &str, operator: &O) -> String
    where
        O: SourceOperator;
}

pub trait FromNaming {
    fn to_lower_camel(&self) -> Box<Naming>;

    fn to_upper_camel(&self) -> Box<Naming>;

    fn to_lower_snake(&self) -> Box<Naming>;

    fn to_upper_snake(&self) -> Box<Naming>;
}

pub trait Naming {
    fn convert(&self, input: &str) -> String;
}

pub struct LowerCamelNaming<S>
where
    S: Source,
{
    source: S,
}

impl<S> LowerCamelNaming<S>
where
    S: Source,
{
}

impl<S> Naming for LowerCamelNaming<S>
where
    S: Source,
{
    fn convert(&self, input: &str) -> String {
        self.source.operate(input, self)
    }
}

impl<S> SourceOperator for LowerCamelNaming<S>
where
    S: Source,
{
    fn first(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_lowercase())
    }

    fn new_section(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_uppercase())
    }

    fn rest(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_lowercase())
    }

    fn join(&self, input: Vec<String>) -> String {
        input.join("")
    }
}

pub struct UpperCamelNaming<S>
where
    S: Source,
{
    source: S,
}

impl<S> UpperCamelNaming<S>
where
    S: Source,
{
}

impl<S> Naming for UpperCamelNaming<S>
where
    S: Source,
{
    fn convert(&self, input: &str) -> String {
        self.source.operate(input, self)
    }
}

impl<S> SourceOperator for UpperCamelNaming<S>
where
    S: Source,
{
    fn first(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_uppercase())
    }

    fn new_section(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_uppercase())
    }

    fn rest(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_lowercase())
    }

    fn join(&self, input: Vec<String>) -> String {
        input.join("")
    }
}

pub struct LowerSnakeNaming<S>
where
    S: Source,
{
    source: S,
}

impl<S> LowerSnakeNaming<S>
where
    S: Source,
{
}

impl<S> Naming for LowerSnakeNaming<S>
where
    S: Source,
{
    fn convert(&self, input: &str) -> String {
        self.source.operate(input, self)
    }
}

impl<S> SourceOperator for LowerSnakeNaming<S>
where
    S: Source,
{
    fn first(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_lowercase())
    }

    fn new_section(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_lowercase())
    }

    fn rest(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_lowercase())
    }

    fn join(&self, input: Vec<String>) -> String {
        input.join("_")
    }
}

pub struct UpperSnakeNaming<S>
where
    S: Source,
{
    source: S,
}

impl<S> UpperSnakeNaming<S>
where
    S: Source,
{
}

impl<S> Naming for UpperSnakeNaming<S>
where
    S: Source,
{
    fn convert(&self, input: &str) -> String {
        self.source.operate(input, self)
    }
}

impl<S> SourceOperator for UpperSnakeNaming<S>
where
    S: Source,
{
    fn first(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_uppercase())
    }

    fn new_section(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_uppercase())
    }

    fn rest(&self, c: char) -> Box<Iterator<Item = char>> {
        Box::new(c.to_uppercase())
    }

    fn join(&self, input: Vec<String>) -> String {
        input.join("_")
    }
}

#[derive(Clone)]
pub struct CamelCase {}

impl CamelCase {
    pub fn new() -> CamelCase {
        CamelCase {}
    }
}

/// A source for camel-cased strings.
impl Source for CamelCase {
    fn operate<O>(&self, input: &str, operator: &O) -> String
    where
        O: SourceOperator,
    {
        let mut out = Vec::new();
        let mut buffer = String::new();

        let mut it = input.chars().peekable();

        let mut first = true;
        let mut first_section = true;

        while let Some(c) = it.next() {
            if first {
                if first_section {
                    buffer.extend(operator.first(c));
                    first_section = false;
                } else {
                    buffer.extend(operator.new_section(c));
                }

                first = false;
            } else {
                for c in operator.rest(c) {
                    buffer.push(c);
                }
            }

            if let Some(n) = it.peek() {
                if n.is_uppercase() && buffer.len() > 0 {
                    out.push(buffer.clone());
                    buffer.clear();
                    first = true;
                    continue;
                }
            }
        }

        if !buffer.is_empty() {
            out.push(buffer.clone());
            buffer.clear();
        }

        operator.join(out)
    }
}

impl FromNaming for CamelCase {
    fn to_lower_camel(&self) -> Box<Naming> {
        Box::new(LowerCamelNaming {
            source: self.clone(),
        })
    }

    fn to_upper_camel(&self) -> Box<Naming> {
        Box::new(UpperCamelNaming {
            source: self.clone(),
        })
    }

    fn to_lower_snake(&self) -> Box<Naming> {
        Box::new(LowerSnakeNaming {
            source: self.clone(),
        })
    }

    fn to_upper_snake(&self) -> Box<Naming> {
        Box::new(UpperSnakeNaming {
            source: self.clone(),
        })
    }
}

#[derive(Clone)]
pub struct SnakeCase {}

impl SnakeCase {
    pub fn new() -> SnakeCase {
        SnakeCase {}
    }
}

/// A source for snake-cased strings.
impl Source for SnakeCase {
    fn operate<O>(&self, input: &str, operator: &O) -> String
    where
        O: SourceOperator,
    {
        let mut out = Vec::new();
        let mut buffer = String::new();

        let mut it = input.chars();

        let mut first = true;
        let mut first_section = true;

        while let Some(c) = it.next() {
            if c == '_' {
                out.push(buffer.clone());
                buffer.clear();
                first = true;
                continue;
            }

            if first {
                if first_section {
                    buffer.extend(operator.first(c));
                    first_section = false;
                } else {
                    buffer.extend(operator.new_section(c));
                }

                first = false;
            } else {
                for c in operator.rest(c) {
                    buffer.push(c);
                }
            }
        }

        if !buffer.is_empty() {
            out.push(buffer.clone());
            buffer.clear();
        }

        operator.join(out)
    }
}

impl FromNaming for SnakeCase {
    fn to_lower_camel(&self) -> Box<Naming> {
        Box::new(LowerCamelNaming {
            source: self.clone(),
        })
    }

    fn to_upper_camel(&self) -> Box<Naming> {
        Box::new(UpperCamelNaming {
            source: self.clone(),
        })
    }

    fn to_lower_snake(&self) -> Box<Naming> {
        Box::new(LowerSnakeNaming {
            source: self.clone(),
        })
    }

    fn to_upper_snake(&self) -> Box<Naming> {
        Box::new(UpperSnakeNaming {
            source: self.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn lower_camel_test() {
        let camel = CamelCase::new().to_lower_camel();

        assert_eq!("fooBarBaz", camel.convert("FooBarBaz"));
        assert_eq!("fooBarBaz", camel.convert("fooBarBaz"));

        let snake = SnakeCase::new().to_lower_camel();

        assert_eq!("fooBarBaz", snake.convert("foo_bar_baz"));
        assert_eq!("fooBarBaz", snake.convert("Foo_Bar_baz"));
    }

    #[test]
    pub fn upper_camel_test() {
        let camel = CamelCase::new().to_upper_camel();

        assert_eq!("FooBarBaz", camel.convert("FooBarBaz"));
        assert_eq!("FooBarBaz", camel.convert("fooBarBaz"));

        let snake = SnakeCase::new().to_upper_camel();

        assert_eq!("FooBarBaz", snake.convert("foo_bar_baz"));
        assert_eq!("FooBarBaz", snake.convert("foo_Bar_baz"));
    }

    #[test]
    pub fn lower_snake_test() {
        let camel = CamelCase::new().to_lower_snake();

        assert_eq!("foo_bar_baz", camel.convert("FooBarBaz"));
        assert_eq!("foo_bar_baz", camel.convert("fooBarBaz"));

        let snake = SnakeCase::new().to_lower_snake();

        assert_eq!("foo_bar_baz", snake.convert("FOO_BAR_BAZ"));
        assert_eq!("foo_bar_baz", snake.convert("foo_BAR_baz"));
    }

    #[test]
    pub fn upper_snake_test() {
        let camel = CamelCase::new().to_upper_snake();

        assert_eq!("FOO_BAR_BAZ", camel.convert("FooBarBaz"));
        assert_eq!("FOO_BAR_BAZ", camel.convert("fooBarBaz"));

        let snake = SnakeCase::new().to_upper_snake();

        assert_eq!("FOO_BAR_BAZ", snake.convert("FOO_BAR_BAZ"));
        assert_eq!("FOO_BAR_BAZ", snake.convert("foo_bar_baz"));
        assert_eq!("FOO_BAR_BAZ", snake.convert("foo_BAR_baz"));
    }
}
