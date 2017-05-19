use super::annotation_spec::AnnotationSpec;
use super::argument_spec::ArgumentSpec;
use super::field_spec::FieldSpec;
use super::variable::{AsVariable, Variable};

fn java_quote_string(input: &str) -> String {
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

    pub fn join<A>(&self, separator: A) -> Statement
        where A: AsVariable + Clone
    {
        Statement::join_with(&self.parts, separator)
    }

    pub fn join_with<S, A>(parts: &Vec<S>, separator: A) -> Statement
        where S: AsVariable + Clone,
              A: AsVariable + Clone
    {
        let mut it = parts.iter().map(AsVariable::as_variable);

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

    pub fn join_statements<S, A>(parts: &Vec<S>, separator: A) -> Statement
        where S: AsStatement + Clone,
              A: AsVariable + Clone
    {
        let mut it = parts.iter().map(AsStatement::as_statement);

        let part = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut stmt = Statement::new();
        stmt.push(part);

        let sep = &separator;

        while let Some(part) = it.next() {
            stmt.push(sep.as_variable());
            stmt.push(part);
        }

        stmt
    }

    pub fn format(&self, level: usize) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut current: Vec<String> = Vec::new();

        for part in &self.parts {
            match *part {
                Variable::Type(ref ty) => {
                    current.push(ty.format(level));
                }
                Variable::String(ref string) => {
                    current.push(java_quote_string(string));
                }
                Variable::Statement(ref stmt) => {
                    current.push(stmt.format(level).join(" "));
                }
                Variable::Literal(ref content) => {
                    current.push(content.to_owned());
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

impl AsStatement for FieldSpec {
    fn as_statement(self) -> Statement {
        let mut s = Statement::new();

        if !self.modifiers.is_empty() {
            s.push(self.modifiers);
            s.push(" ");
        }

        s.push(self.ty);
        s.push(" ");
        s.push(self.name);

        s
    }
}

impl AsStatement for ArgumentSpec {
    fn as_statement(self) -> Statement {
        let mut s = Statement::new();

        for a in &self.annotations {
            s.push(a);
            s.push(" ");
        }

        if !self.modifiers.is_empty() {
            s.push(self.modifiers);
            s.push(" ");
        }

        s.push(self.ty);
        s.push(" ");
        s.push(self.name);

        s
    }
}

impl AsStatement for AnnotationSpec {
    fn as_statement(self) -> Statement {
        let mut stmt = Statement::new();

        let mut annotation = Statement::new();
        annotation.push("@");
        annotation.push(self.ty);

        if !self.arguments.is_empty() {
            stmt.push(annotation);
            stmt.push("(");
            stmt.push(Statement::join_with(&self.arguments, ", "));
            stmt.push(")");
        } else {
            stmt.push(annotation);
        }

        stmt
    }
}
