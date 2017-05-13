use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Modifier {
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Modifiers {
    modifiers: HashSet<Modifier>,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers { modifiers: HashSet::new() }
    }

    pub fn add(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

error_chain! {
    errors {
        InvalidEscape {
        }

        VariableUnderflow {
        }
    }
}

pub enum Section {
    Block(Block),
    Statement(Statement),
    Spacing,
}

pub enum Variable {
    Literal(String),
    Type(Type),
}

pub struct Statement {
    format: String,
    variables: Vec<Variable>,
}

impl Statement {
    pub fn new(format: &str) -> Statement {
        Statement {
            format: format.to_owned(),
            variables: Vec::new(),
        }
    }

    pub fn push_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    pub fn format(&self) -> Result<String> {
        let mut out = String::new();

        let mut it = self.format.chars();

        let mut var_it = self.variables.iter();

        while let Some(c) = it.next() {
            match c {
                '$' => {
                    let kind: char = it.next().ok_or(ErrorKind::InvalidEscape)?;
                    let var = var_it.next().ok_or(ErrorKind::VariableUnderflow)?;

                    match kind {
                        'S' => {
                            if let Variable::Literal(ref literal) = *var {
                                out.push_str(literal);
                            }
                        }
                        '$' => out.push('$'),
                        _ => return Err(ErrorKind::InvalidEscape.into()),
                    }
                }
                c => out.push(c),
            }
        }

        Ok(out)
    }
}

pub struct Block {
    open: Option<Statement>,
    close: Option<Statement>,
    statements: Vec<Section>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            open: None,
            close: None,
            statements: Vec::new(),
        }
    }

    pub fn open(&mut self, open: Statement) {
        self.open = Some(open)
    }

    pub fn close(&mut self, close: Statement) {
        self.close = Some(close)
    }

    pub fn statement(&mut self, statement: Statement) {
        self.statements.push(Section::Statement(statement));
    }

    pub fn spacing(&mut self) {
        self.statements.push(Section::Spacing);
    }

    pub fn block(&mut self, block: Block) {
        self.statements.push(Section::Block(block));
    }

    pub fn format_statements(&self, indent: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();

        for var in &self.statements {
            match *var {
                Section::Statement(ref statement) => {
                    out.push(format!("{}{};", indent, statement.format()?));
                }
                Section::Block(ref block) => {
                    out.extend(block.format(indent)?);
                }
                Section::Spacing => {
                    out.push("".to_owned());
                }
                _ => {}
            }
        }

        Ok(out)
    }

    pub fn format(&self, indent: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();

        if let Some(ref open) = self.open {
            out.push(format!("{}{} {{", indent, open.format()?).to_owned());
        } else {
            out.push(format!("{}{{", indent).to_owned());
        }

        out.extend(self.format_statements(indent)?);

        if let Some(ref close) = self.close {
            out.push(format!("{}{} {{", indent, close.format()?).to_owned());
        } else {
            out.push(format!("{}}}", indent).to_owned());
        }

        Ok(out)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Type {
    package: String,
    name: String,
}

impl Type {
    pub fn fully_qualified_name(&self) -> String {
        format!("{}.{}", self.package, self.name).to_owned()
    }
}

#[derive(Debug)]
pub struct MethodArgument {
    modifiers: Modifiers,
}

#[derive(Debug)]
pub struct MethodDecl {
    modifiers: Modifiers,
}

impl MethodDecl {
    pub fn new() -> MethodDecl {
        MethodDecl { modifiers: Modifiers::new() }
    }
}

#[derive(Debug)]
pub struct ClassDecl {
    name: String,
    methods: Vec<MethodDecl>,
}

impl ClassDecl {
    pub fn new(name: &str) -> ClassDecl {
        ClassDecl {
            name: name.to_owned(),
            types: HashSet::new(),
        }
    }

    fn resolve_imports(&self) -> &HashSet<Type> {
        &self.types
    }

    pub fn as_block(&self) -> Block {
        let mut open = Statement::new("public class $S");
        open.push_variable(Variable::Literal(self.name.clone()));

        let mut block = Block::new();
        block.open(open);

        block
    }
}

#[derive(Debug)]
pub struct FileDecl {
    package: String,
    class: ClassDecl,
}

impl FileDecl {
    pub fn new(package: &str, class: ClassDecl) -> FileDecl {
        FileDecl {
            package: package.to_owned(),
            class: class,
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let mut block = Block::new();

        let mut package = Statement::new("package $S");
        package.push_variable(Variable::Literal(self.package.clone()));
        block.statement(package);
        block.spacing();

        let mut out = String::new();

        for t in self.class.resolve_imports() {
            let mut import = Statement::new("import $S");
            import.push_variable(Variable::Type(t.clone()));
            block.statement(import);
        }

        block.block(self.class.as_block());

        let mut out = String::new();

        for line in block.format_statements("")? {
            out.push_str(&line);
            out.push('\n');
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        let mut class = ClassDecl::new("Foo");
        let mut file = FileDecl::new("se.tedro", class);

        println!("{}", file.to_string().unwrap());
    }
}
