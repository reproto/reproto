use super::class_spec::ClassSpec;
use super::decorator_spec::DecoratorSpec;
use super::method_spec::MethodSpec;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub enum ElementSpec {
    Statement(Statement),
    Literal(Vec<String>),
    Elements(Vec<ElementSpec>),
    Nested(Box<ElementSpec>),
    Spacing,
}

impl ElementSpec {
    pub fn format(&self, current: &str, indent: &str) -> Vec<String> {
        let mut out = Vec::new();

        match *self {
            ElementSpec::Statement(ref statement) => {
                for line in statement.format() {
                    out.push(format!("{}{}", current, line));
                }
            }
            ElementSpec::Literal(ref literal) => {
                for line in literal {
                    out.push(format!("{}{}", current, line));
                }
            }
            ElementSpec::Elements(ref elements) => {
                for element in elements {
                    out.extend(element.format(current, indent));
                }
            }
            ElementSpec::Nested(ref element) => {
                let next_current = format!("{}{}", current, indent);
                out.extend(element.format(&next_current, indent));
            }
            ElementSpec::Spacing => {
                out.push(current.to_owned());
            }
        };

        out
    }

    pub fn join<S>(self, separator: S) -> ElementSpec
        where S: AsElementSpec + Clone
    {
        if let ElementSpec::Elements(elements) = self {
            let mut it = elements.into_iter();

            let part = match it.next() {
                Some(part) => part,
                None => return ElementSpec::Elements(vec![]),
            };

            let mut parts: Vec<ElementSpec> = Vec::new();
            parts.push(part);

            let sep = &separator;

            while let Some(part) = it.next() {
                parts.push(sep.as_element_spec());
                parts.push(part);
            }

            return ElementSpec::Elements(parts);
        }

        return self;
    }
}

pub trait AsElementSpec {
    fn as_element_spec(self) -> ElementSpec;
}

impl<'a, A> AsElementSpec for &'a A
    where A: AsElementSpec + Clone
{
    fn as_element_spec(self) -> ElementSpec {
        self.clone().as_element_spec()
    }
}

impl<'a> AsElementSpec for &'a str {
    fn as_element_spec(self) -> ElementSpec {
        ElementSpec::Literal(vec![self.to_owned()])
    }
}

impl AsElementSpec for ElementSpec {
    fn as_element_spec(self) -> ElementSpec {
        self
    }
}

impl AsElementSpec for MethodSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut out = Vec::new();

        for decorator in self.decorators {
            out.push(decorator.as_element_spec());
        }

        let mut decl = Statement::new();
        decl.push("def ");
        decl.push(self.name);
        decl.push("(");

        let mut arguments = Statement::new();

        for argument in self.arguments {
            arguments.push(argument);
        }

        decl.push(arguments.join(", "));
        decl.push("):");

        out.push(decl.as_element_spec());

        if self.elements.is_empty() {
            out.push(ElementSpec::Nested(Box::new("pass".as_element_spec())));
        } else {
            out.push(ElementSpec::Nested(Box::new(self.elements
                .as_element_spec()
                .join(ElementSpec::Spacing))));
        }

        ElementSpec::Elements(out)
    }
}

impl AsElementSpec for ClassSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut out = Vec::new();

        for decorator in self.decorators {
            out.push(decorator.as_element_spec());
        }

        let mut decl = Statement::new();
        decl.push("class ");
        decl.push(self.name);
        decl.push(":");

        out.push(decl.as_element_spec());

        if self.elements.is_empty() {
            out.push(ElementSpec::Nested(Box::new("pass".as_element_spec())));
        } else {
            out.push(ElementSpec::Nested(Box::new(self.elements
                .as_element_spec()
                .join(ElementSpec::Spacing))));
        }

        ElementSpec::Elements(out)
    }
}

impl AsElementSpec for DecoratorSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut decl = Statement::new();
        decl.push("@");
        decl.push(self.name);

        decl.as_element_spec()
    }
}

impl AsElementSpec for Statement {
    fn as_element_spec(self) -> ElementSpec {
        ElementSpec::Statement(self)
    }
}

impl AsElementSpec for Vec<ElementSpec> {
    fn as_element_spec(self) -> ElementSpec {
        ElementSpec::Elements(self)
    }
}