use super::class_spec::ClassSpec;
use super::interface_spec::InterfaceSpec;
use super::method_spec::MethodSpec;
use super::section::{AsSection, Sections, Section};
use super::statement::{AsStatement, Statement};
use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct Block {
    pub open: Option<Statement>,
    pub close: Option<Statement>,
    pub sections: Sections,
}

impl Block {
    pub fn new() -> Block {
        Block {
            open: None,
            close: None,
            sections: Sections::new(),
        }
    }

    pub fn open<S>(&mut self, open: S)
        where S: AsStatement
    {
        self.open = Some(open.as_statement())
    }

    pub fn close<S>(&mut self, close: S)
        where S: AsStatement
    {
        self.close = Some(close.as_statement())
    }

    pub fn push<S>(&mut self, section: S)
        where S: AsSection
    {
        self.sections.push(section);
    }

    pub fn extend(&mut self, sections: &Sections) {
        self.sections.extend(sections);
    }

    pub fn format(&self, level: usize, current: &str, indent: &str) -> Vec<String> {
        let mut out = Vec::new();

        if let Some(ref open) = self.open {
            let mut it = open.format(level).into_iter().peekable();

            while let Some(line) = it.next() {
                if it.peek().is_none() {
                    out.push(format!("{}{} {{", current, line).to_owned());
                } else {
                    out.push(format!("{}{}", current, line).to_owned());
                }
            }
        } else {
            out.push(format!("{}{{", current).to_owned());
        }

        out.extend(self.sections.format(level, &format!("{}{}", current, indent), indent));

        if let Some(ref close) = self.close {
            let close = close.format(level).join(" ");
            out.push(format!("{}}} {};", current, close).to_owned());
        } else {
            out.push(format!("{}}}", current).to_owned());
        }

        out
    }
}

pub trait AsBlock {
    fn as_block(self) -> Block;
}

impl<'a, A> AsBlock for &'a A
    where A: AsBlock + Clone
{
    fn as_block(self) -> Block {
        self.clone().as_block()
    }
}

impl AsBlock for Block {
    fn as_block(self) -> Block {
        self
    }
}

impl AsBlock for ClassSpec {
    fn as_block(self) -> Block {
        let mut open = Statement::new();

        for a in &self.annotations {
            open.push(a.as_statement());
            open.push(Variable::Spacing);
        }

        if !self.modifiers.is_empty() {
            open.push(self.modifiers);
            open.push(" ");
        }

        open.push("class ");
        open.push(&self.name);

        let mut block = Block::new();
        block.open(open);

        for field in &self.fields {
            block.push(field.as_statement());
        }

        /// TODO: figure out a better way...
        let mut first = self.fields.is_empty();

        for constructor in &self.constructors {
            if first {
                first = false;
            } else {
                block.push(Section::Spacing);
            }

            block.push(constructor.as_block(&self.name));
        }

        for method in &self.methods {
            if first {
                first = false;
            } else {
                block.push(Section::Spacing);
            }

            block.push(method.as_block());
        }

        for element in &self.elements {
            if first {
                first = false;
            } else {
                block.push(Section::Spacing);
            }

            block.push(element);
        }

        block
    }
}

impl AsBlock for MethodSpec {
    fn as_block(self) -> Block {
        let mut open = Statement::new();

        for a in &self.annotations {
            open.push(a.as_statement());
            open.push(Variable::Spacing);
        }

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
            let mut arguments: Statement = Statement::new();

            for a in &self.arguments {
                arguments.push(a.as_statement());
            }

            open.push(arguments.join(", "));
        }

        open.push(")");

        let mut block = Block::new();
        block.open(open);
        block.extend(&self.sections);

        block
    }
}

impl AsBlock for InterfaceSpec {
    fn as_block(self) -> Block {
        let mut open = Statement::new();

        for a in &self.annotations {
            open.push(a.as_statement());
            open.push(Variable::Spacing);
        }

        if !self.modifiers.is_empty() {
            open.push(self.modifiers);
            open.push(" ");
        }

        open.push("interface ");
        open.push(self.name);

        let mut block = Block::new();
        block.open(open);

        let mut first: bool = true;

        for element in &self.elements {
            if first {
                first = false;
            } else {
                block.push(Section::Spacing);
            }

            block.push(element);
        }

        block
    }
}
