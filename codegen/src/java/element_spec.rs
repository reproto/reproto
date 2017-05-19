use super::annotation_spec::AnnotationSpec;
use super::class_spec::ClassSpec;
use super::elements::Elements;
use super::interface_spec::InterfaceSpec;
use super::method_spec::MethodSpec;
use super::statement::{AsStatement, Statement};

#[derive(Debug, Clone)]
pub enum ElementSpec {
    Statement(Statement),
    Literal(String),
    Elements(Vec<ElementSpec>),
    Nested(Box<ElementSpec>),
    Spacing,
}

impl ElementSpec {
    pub fn format(&self, current: &str, indent: &str) -> Vec<String> {
        let mut out = Vec::new();

        match *self {
            ElementSpec::Statement(ref statement) => {
                for line in statement.format(0usize) {
                    out.push(format!("{}{}", current, line));
                }
            }
            ElementSpec::Literal(ref line) => {
                out.push(format!("{}{}", current, line));
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
                out.push("".to_owned());
            }
        };

        out
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
        ElementSpec::Literal(self.to_owned())
    }
}

impl AsElementSpec for ElementSpec {
    fn as_element_spec(self) -> ElementSpec {
        self
    }
}

impl AsElementSpec for Statement {
    fn as_element_spec(self) -> ElementSpec {
        ElementSpec::Statement(self)
    }
}

impl AsElementSpec for Elements {
    fn as_element_spec(self) -> ElementSpec {
        ElementSpec::Elements(self.elements)
    }
}

impl AsElementSpec for Vec<String> {
    fn as_element_spec(self) -> ElementSpec {
        ElementSpec::Elements(self.into_iter().map(ElementSpec::Literal).collect())
    }
}

impl AsElementSpec for ClassSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut elements = Elements::new();

        for a in &self.annotations {
            elements.push(a);
        }

        let mut open = Statement::new();

        if !self.modifiers.is_empty() {
            open.push(self.modifiers);
            open.push(" ");
        }

        open.push("class ");
        open.push(&self.name);
        open.push(" {");

        elements.push(open);

        let mut class_body = Elements::new();

        let mut fields = Elements::new();

        for field in &self.fields {
            let mut field = field.as_statement();
            field.push(";");
            fields.push(field);
        }

        class_body.push(fields);

        for constructor in &self.constructors {
            class_body.push(constructor.as_element_spec(&self.name));
        }

        for element in &self.elements.elements {
            class_body.push(element);
        }

        elements.push_nested(class_body.join(ElementSpec::Spacing));
        elements.push("}");

        elements.as_element_spec()
    }
}

impl AsElementSpec for MethodSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut elements = Elements::new();

        for a in &self.annotations {
            elements.push(a);
        }

        let mut open = Statement::new();

        if !self.modifiers.is_empty() {
            open.push(self.modifiers);
            open.push(" ");
        }

        match self.returns {
            None => open.push("void "),
            Some(ref returns) => {
                open.push(returns);
                open.push(" ");
            }
        }

        open.push(self.name);
        open.push("(");

        if !self.arguments.is_empty() {
            open.push(Statement::join_statements(&self.arguments, ", "));
        }

        open.push(") {");

        elements.push(open);
        elements.push_nested(self.elements.join(ElementSpec::Spacing));
        elements.push("}");

        elements.as_element_spec()
    }
}

impl AsElementSpec for InterfaceSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut elements = Elements::new();

        let mut open = Statement::new();

        for a in &self.annotations {
            elements.push(a);
        }

        if !self.modifiers.is_empty() {
            open.push(self.modifiers);
            open.push(" ");
        }

        open.push("interface ");
        open.push(self.name);
        open.push(" {");

        elements.push(open);
        elements.push_nested(self.elements.join(ElementSpec::Spacing));
        elements.push("}");

        elements.as_element_spec()
    }
}

impl AsElementSpec for AnnotationSpec {
    fn as_element_spec(self) -> ElementSpec {
        let mut elements = Elements::new();

        let mut annotation = Statement::new();
        annotation.push("@");
        annotation.push(self.ty);

        if !self.arguments.is_empty() {
            let mut open = Statement::new();

            open.push(annotation);
            open.push("(");
            open.push(Statement::join_with(&self.arguments, ", "));
            open.push(")");

            elements.push(open);
        } else {
            elements.push(annotation);
        }

        elements.as_element_spec()
    }
}
