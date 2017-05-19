use super::variable::{AsVariable, Variable};

fn python_quote_string(input: &str) -> String {
    let mut out = String::new();
    let mut it = input.chars();

    out.push('"');

    while let Some(c) = it.next() {
        match c {
            '\t' => out.push_str("\\t"),
            '\u{0007}' => out.push_str("\\b"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{0014}' => out.push_str("\\f"),
            '\'' => out.push_str("\\'"),
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }

    out.push('"');
    out
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub parts: Vec<Variable>,
}

impl Statement {
    pub fn new() -> Statement {
        Statement { parts: Vec::new() }
    }

    pub fn push<V>(&mut self, variable: V)
        where V: AsVariable
    {
        self.parts.push(variable.as_variable());
    }

    pub fn push_arguments<S, A>(&mut self, arguments: &Vec<S>, separator: A)
        where S: AsStatement + Clone,
              A: AsVariable + Clone
    {
        if arguments.is_empty() {
            return;
        }

        let mut out: Statement = Statement::new();

        for a in arguments {
            out.push(a.as_statement());
        }

        self.push(out.join(separator));
    }

    pub fn join<A>(self, separator: A) -> Statement
        where A: AsVariable + Clone
    {
        let mut it = self.parts.into_iter();

        let part = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut parts: Vec<Variable> = Vec::new();
        parts.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            parts.push(sep.as_variable());
            parts.push(part);
        }

        Statement { parts: parts }
    }

    pub fn format(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut current: Vec<String> = Vec::new();

        for part in &self.parts {
            match *part {
                Variable::String(ref string) => {
                    current.push(python_quote_string(string));
                }
                Variable::Statement(ref stmt) => {
                    current.push(stmt.format().join(" "));
                }
                Variable::Literal(ref content) => {
                    current.push(content.to_owned());
                }
                Variable::Name(ref name) => {
                    current.push(name.format());
                }
                Variable::Spacing => {
                    out.push(current.join(""));
                    current.clear();
                }
            }
        }

        if !current.is_empty() {
            out.push(current.join(""));
            current.clear();
        }

        out
    }
}

pub trait AsStatement {
    fn as_statement(self) -> Statement;
}

impl<'a, A> AsStatement for &'a A
    where A: AsStatement + Clone
{
    fn as_statement(self) -> Statement {
        self.clone().as_statement()
    }
}

impl AsStatement for Statement {
    fn as_statement(self) -> Statement {
        self
    }
}

impl AsStatement for Variable {
    fn as_statement(self) -> Statement {
        Statement { parts: vec![self] }
    }
}

impl AsStatement for String {
    fn as_statement(self) -> Statement {
        Statement { parts: vec![Variable::Literal(self)] }
    }
}
