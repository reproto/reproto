/// A code generator inspired by JavaPoet (https://github.com/square/javapoet)

use std::collections::BTreeSet;

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

fn format_statement_part_format(format: &String, variables: &Vec<Variable>) -> Result<String> {
    let mut out = String::new();

    let mut it = format.chars();
    let mut var_it = variables.iter();

    while let Some(c) = it.next() {
        match c {
            '$' => {
                let kind: char = it.next().ok_or(ErrorKind::InvalidEscape)?;
                let var = var_it.next().ok_or(ErrorKind::VariableUnderflow)?;

                match kind {
                    'L' => {
                        if let Variable::Literal(ref literal) = *var {
                            out.push_str(literal);
                        } else {
                            return Err(ErrorKind::InvalidVariable.into());
                        }
                    }
                    'T' => {
                        if let Variable::TypeSpec(ref type_) = *var {
                            out.push_str(&type_.format()?);
                        } else {
                            return Err(ErrorKind::InvalidVariable.into());
                        }
                    }
                    'S' => {
                        if let Variable::String(ref string) = *var {
                            out.push_str(&java_quote_string(string));
                        } else {
                            return Err(ErrorKind::InvalidVariable.into());
                        }
                    }
                    'N' => {
                        if let Variable::Name(ref name) = *var {
                            out.push_str(name);
                        } else {
                            return Err(ErrorKind::InvalidVariable.into());
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

fn add_annotations(annotations: &Vec<AnnotationSpec>, target: &mut Statement) -> Result<()> {
    if annotations.is_empty() {
        return Ok(());
    }

    let mut out = Statement::new();

    for a in annotations {
        out.push_statement(a.as_statement()?);
    }

    target.push_statement(out.join(" "));
    target.push_literal(" ");

    Ok(())
}

fn add_arguments<S>(arguments: &Vec<S>, target: &mut Statement) -> Result<()>
    where S: AsStatement
{
    if arguments.is_empty() {
        return Ok(());
    }

    let mut out: Statement = Statement::new();

    for a in arguments {
        out.push_statement(a.as_statement()?);
    }

    target.push_statement(out.join(", "));

    Ok(())
}

error_chain! {
    errors {
        InvalidEscape {
        }

        InvalidVariable {
        }

        VariableUnderflow {
        }
    }
}

pub trait Imports {
    fn imports<I>(&self, &mut I) where I: ImportReceiver;
}

pub trait ImportReceiver {
    fn receive(&mut self, type_: &Type);

    fn import_all<T>(&mut self, sources: &Vec<T>)
        where T: Imports,
              Self: Sized
    {
        for source in sources {
            source.imports(self);
        }
    }
}

/// Trait allowing a type to be converted to a statement.
pub trait AsStatement {
    fn as_statement(&self) -> Result<Statement>;
}

impl AsStatement for Statement {
    fn as_statement(&self) -> Result<Statement> {
        Ok(self.clone())
    }
}

impl ImportReceiver for BTreeSet<Type> {
    fn receive(&mut self, type_: &Type) {
        self.insert(type_.clone());
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Modifier {
    Public,
    Protected,
    Private,
    Static,
    Final,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Modifiers {
    modifiers: BTreeSet<Modifier>,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers { modifiers: BTreeSet::new() }
    }

    pub fn insert(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }

    pub fn format(&self) -> Result<String> {
        let mut out: Vec<String> = Vec::new();

        for m in &self.modifiers {
            out.push(match *m {
                Modifier::Public => "public".to_owned(),
                Modifier::Protected => "protected".to_owned(),
                Modifier::Private => "private".to_owned(),
                Modifier::Static => "static".to_owned(),
                Modifier::Final => "final".to_owned(),
            });
        }

        Ok(out.join(" "))
    }

    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum Section {
    Block(Block),
    Statement(Statement),
    Spacing,
}

impl Section {
    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Section::Block(ref block) => block.imports(receiver),
            Section::Statement(ref statement) => statement.imports(receiver),
            _ => {}
        };
    }
}

#[derive(Debug, Clone)]
pub enum Variable {
    Literal(String),
    TypeSpec(TypeSpec),
    String(String),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum StatementPart {
    // literal part
    Literal(String),
    // formatted part
    Format(String, Vec<Variable>),
    // nested statement
    Statement(Statement),
}

#[derive(Debug, Clone)]
pub struct Statement {
    parts: Vec<StatementPart>,
}

impl Statement {
    pub fn new() -> Statement {
        Statement { parts: Vec::new() }
    }

    pub fn push_literal(&mut self, literal: &str) {
        self.parts.push(StatementPart::Literal(literal.to_owned()));
    }

    pub fn push_statement(&mut self, statement: Statement) {
        self.parts.push(StatementPart::Statement(statement));
    }

    pub fn push(&mut self, format: &str, variables: Vec<Variable>) {
        self.parts.push(StatementPart::Format(format.to_owned(), variables));
    }

    pub fn join(self, literal: &str) -> Statement {
        let mut it = self.parts.into_iter();

        let part = match it.next() {
            Some(part) => part,
            None => return Statement::new(),
        };

        let mut parts: Vec<StatementPart> = Vec::new();
        parts.push(part);

        while let Some(part) = it.next() {
            parts.push(StatementPart::Literal(literal.to_owned()));
            parts.push(part);
        }

        Statement { parts: parts }
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        for part in &self.parts {
            match *part {
                StatementPart::Format(_, ref variables) => {
                    for var in variables {
                        if let Variable::TypeSpec(ref type_) = *var {
                            type_.imports(receiver);
                        }
                    }
                }
                StatementPart::Statement(ref stmt) => {
                    stmt.imports(receiver);
                }
                _ => {}
            }
        }
    }

    pub fn format(&self) -> Result<String> {
        let mut result: Vec<String> = Vec::new();

        for part in &self.parts {
            match *part {
                StatementPart::Format(ref format, ref variables) => {
                    result.push(format_statement_part_format(format, variables)?);
                }
                StatementPart::Statement(ref stmt) => {
                    result.push(stmt.format()?);
                }
                StatementPart::Literal(ref string) => {
                    result.push(string.clone());
                }
            }
        }

        Ok(result.join(""))
    }
}

macro_rules! stmt {
    ($($fmt:expr, $vars:expr),*) => {{
        let mut s = Statement::new();
        $(s.push($fmt, $vars);)*
        s
    }};

    ($fmt:expr, $($tail:tt)*) => {{
        let mut s = Statement::new();
        let mut vars = Vec::new();
        vars.extend(stmt!($($tail)*));
        s.push($fmt, vars);
        s
    }};

    (type_spec $var:expr) => {{
        vec![Variable::TypeSpec($var.as_type_spec())]
    }};

    (type_spec $var:expr, $($tail:tt)*) => {{
        let mut vars = vec![Variable::TypeSpec($var.as_type_spec())];
        vars.extend(stmt!($($tail)*));
        vars
    }};

    (name $var:expr) => {{
        vec![Variable::Name($var.as_name())]
    }};

    (name $var:expr, $($tail:tt)*) => {{
        let mut vars = vec![Variable::Name($var.as_name())];
        vars.extend(stmt!($($tail)*));
        vars
    }};

    (literal $var:expr) => {{
        vec![Variable::Literal($var)]
    }};

    (literal $var:expr, $($tail:tt)*) => {{
        let mut vars = vec![Variable::Literal($var)];
        vars.extend(stmt!($($tail)*));
        vars
    }};

    (string $var:expr) => {{
        vec![Variable::String($var)]
    }};

    (string $var:expr, $($tail:tt)*) => {{
        let mut vars = vec![Variable::String($var)];
        vars.extend(stmt!($($tail)*));
        vars
    }};
}

macro_rules! mods {
    ($($modifier:expr),*) => {
        {
            let mut tmp_modifiers = Modifiers::new();

            $(
                tmp_modifiers.insert($modifier);
            )*

            tmp_modifiers
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sections {
    sections: Vec<Section>,
}

impl Sections {
    pub fn new() -> Sections {
        Sections { sections: Vec::new() }
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        for section in &self.sections {
            section.imports(receiver);
        }
    }

    pub fn push_statement(&mut self, statement: &Statement) {
        self.sections.push(Section::Statement(statement.clone()));
    }

    pub fn push_spacing(&mut self) {
        self.sections.push(Section::Spacing);
    }

    pub fn push_block(&mut self, block: &Block) {
        self.sections.push(Section::Block(block.clone()));
    }

    pub fn extend(&mut self, sections: &Sections) {
        self.sections.extend(sections.sections.iter().map(Clone::clone));
    }

    pub fn format(&self, current: &str, indent: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();

        for section in &self.sections {
            match *section {
                Section::Statement(ref statement) => {
                    out.push(format!("{}{};", current, statement.format()?));
                }
                Section::Block(ref block) => {
                    out.extend(block.format(current, indent)?);
                }
                Section::Spacing => {
                    out.push("".to_owned());
                }
            }
        }

        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    open: Option<Statement>,
    close: Option<Statement>,
    sections: Sections,
}

impl Block {
    pub fn new() -> Block {
        Block {
            open: None,
            close: None,
            sections: Sections::new(),
        }
    }

    pub fn open(&mut self, open: Statement) {
        self.open = Some(open)
    }

    pub fn close(&mut self, close: Statement) {
        self.close = Some(close)
    }

    pub fn push_statement(&mut self, statement: &Statement) {
        self.sections.push_statement(statement);
    }

    pub fn push_spacing(&mut self) {
        self.sections.push_spacing();
    }

    pub fn push_block(&mut self, block: &Block) {
        self.sections.push_block(block);
    }

    pub fn extend(&mut self, sections: &Sections) {
        self.sections.extend(sections);
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        if let Some(ref open) = self.open {
            open.imports(receiver);
        }

        if let Some(ref close) = self.close {
            close.imports(receiver);
        }

        self.sections.imports(receiver);
    }

    pub fn format(&self, current: &str, indent: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();

        if let Some(ref open) = self.open {
            out.push(format!("{}{} {{", current, open.format()?).to_owned());
        } else {
            out.push(format!("{}{{", current).to_owned());
        }

        out.extend(self.sections.format(&format!("{}{}", current, indent), indent)?);

        if let Some(ref close) = self.close {
            out.push(format!("{}{} {{", current, close.format()?).to_owned());
        } else {
            out.push(format!("{}}}", current).to_owned());
        }

        Ok(out)
    }
}


/// Raw (importable) types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Type {
    package: String,
    name: String,
}

impl Type {
    pub fn new(package: &str, name: &str) -> Type {
        Type {
            package: package.to_owned(),
            name: name.to_owned(),
        }
    }

    fn with_arguments<I>(&self, arguments: Vec<I>) -> TypeSpec
        where I: AsTypeSpec
    {
        let arguments: Vec<TypeSpec> = arguments.iter().map(AsTypeSpec::as_type_spec).collect();
        TypeSpec::new(self.clone(), arguments)
    }
}

/// Implementation for TypeSpec reference (&TypeSpec) to TypeSpec conversion.
impl<'a> AsTypeSpec for &'a TypeSpec {
    fn as_type_spec(&self) -> TypeSpec {
        (*self).clone()
    }
}

/// Implementation for Type reference (&Type) to TypeSpec conversion.
impl<'a> AsTypeSpec for &'a Type {
    fn as_type_spec(&self) -> TypeSpec {
        TypeSpec::new((*self).clone(), vec![])
    }
}

/// Implementation for Type to TypeSpec conversion.
impl AsTypeSpec for Type {
    fn as_type_spec(&self) -> TypeSpec {
        TypeSpec::new(self.clone(), vec![])
    }
}

/// Trait for types that can be converted into TypeSpec's
pub trait AsTypeSpec {
    fn as_type_spec(&self) -> TypeSpec;
}

pub trait AsName {
    fn as_name(&self) -> String;
}

/// Complete types, including generic arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TypeSpec {
    raw: Type,
    arguments: Vec<TypeSpec>,
}

impl TypeSpec {
    pub fn new(raw: Type, arguments: Vec<TypeSpec>) -> TypeSpec {
        TypeSpec {
            raw: raw,
            arguments: arguments,
        }
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.receive(&self.raw);

        for t in &self.arguments {
            t.imports(receiver);
        }
    }

    pub fn format(&self) -> Result<String> {
        let mut out = String::new();

        out.push_str(&self.raw.name);

        if !self.arguments.is_empty() {
            let mut arguments = Vec::new();

            for g in &self.arguments {
                arguments.push(g.format()?);
            }

            let joined = arguments.join(", ");

            out.push('<');
            out.push_str(&joined);
            out.push('>');
        }

        Ok(out)
    }
}

impl AsTypeSpec for TypeSpec {
    fn as_type_spec(&self) -> TypeSpec {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MethodArgument {
    modifiers: Modifiers,
}

#[derive(Debug, Clone)]
pub struct FieldSpec {
    modifiers: Modifiers,
    type_: TypeSpec,
    name: String,
}

impl FieldSpec {
    pub fn new<I>(modifiers: Modifiers, type_: I, name: &str) -> FieldSpec
        where I: AsTypeSpec
    {
        FieldSpec {
            modifiers: modifiers,
            type_: type_.as_type_spec(),
            name: name.to_owned(),
        }
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.type_.imports(receiver);
    }
}

impl AsStatement for FieldSpec {
    fn as_statement(&self) -> Result<Statement> {
        let mut s = Statement::new();

        if !self.modifiers.is_empty() {
            s.push("$L ", stmt!(literal self.modifiers.format()?));
        }

        s.push("$T ", stmt![type_spec self.type_]);
        s.push("$L", stmt!(literal self.name.clone()));

        Ok(s)
    }
}

impl AsName for FieldSpec {
    fn as_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ConstructorSpec {
    modifiers: Modifiers,
    annotations: Vec<AnnotationSpec>,
    arguments: Vec<ArgumentSpec>,
    sections: Sections,
}

impl ConstructorSpec {
    pub fn new(modifiers: Modifiers) -> ConstructorSpec {
        ConstructorSpec {
            modifiers: modifiers,
            annotations: Vec::new(),
            arguments: Vec::new(),
            sections: Sections::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push_argument(&mut self, argument: &ArgumentSpec) {
        self.arguments.push(argument.clone());
    }

    pub fn push_statement(&mut self, statement: &Statement) {
        self.sections.push_statement(statement);
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.sections.imports(receiver);
        receiver.import_all(&self.annotations);
        receiver.import_all(&self.arguments);
    }

    pub fn as_block(&self, enclosing: &str) -> Result<Block> {
        let mut open = Statement::new();

        add_annotations(&self.annotations, &mut open)?;

        if !self.modifiers.is_empty() {
            open.push("$L ", stmt!(literal self.modifiers.format()?));
        }

        open.push("$L(", stmt![literal enclosing.to_owned()]);
        add_arguments(&self.arguments, &mut open)?;
        open.push(")", vec![]);

        let mut block = Block::new();
        block.open(open);
        block.extend(&self.sections);

        Ok(block)
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationSpec {
    type_: TypeSpec,
    arguments: Vec<Statement>,
}

impl AnnotationSpec {
    pub fn new<I>(type_: I) -> AnnotationSpec
        where I: AsTypeSpec
    {
        AnnotationSpec {
            type_: type_.as_type_spec(),
            arguments: Vec::new(),
        }
    }

    pub fn push_argument(&mut self, statement: &Statement) {
        self.arguments.push(statement.clone());
    }
}

impl AsStatement for AnnotationSpec {
    fn as_statement(&self) -> Result<Statement> {
        let mut s = Statement::new();
        s.push("@$T", stmt![type_spec self.type_]);

        if !self.arguments.is_empty() {
            s.push_literal("(");
            add_arguments(&self.arguments, &mut s)?;
            s.push_literal(")");
        }

        Ok(s)
    }
}

impl Imports for AnnotationSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.type_.imports(receiver);

        for a in &self.arguments {
            a.imports(receiver);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentSpec {
    modifiers: Modifiers,
    type_: TypeSpec,
    name: String,
    annotations: Vec<AnnotationSpec>,
}

impl ArgumentSpec {
    pub fn new<I>(modifiers: Modifiers, type_: I, name: &str) -> ArgumentSpec
        where I: AsTypeSpec
    {
        ArgumentSpec {
            modifiers: modifiers,
            type_: type_.as_type_spec(),
            name: name.to_owned(),
            annotations: Vec::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }
}

impl AsStatement for ArgumentSpec {
    fn as_statement(&self) -> Result<Statement> {
        let mut s = Statement::new();

        add_annotations(&self.annotations, &mut s)?;

        if !self.modifiers.is_empty() {
            s.push("$L ", stmt!(literal self.modifiers.format()?));
        }

        s.push("$T ", stmt![type_spec self.type_.clone()]);
        s.push("$L", stmt!(literal self.name.clone()));

        Ok(s)
    }
}

impl Imports for ArgumentSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.type_.imports(receiver);

        for a in &self.annotations {
            a.imports(receiver);
        }
    }
}

impl AsName for ArgumentSpec {
    fn as_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MethodSpec {
    modifiers: Modifiers,
    name: String,
    annotations: Vec<AnnotationSpec>,
    arguments: Vec<ArgumentSpec>,
    returns: Option<TypeSpec>,
    sections: Sections,
}

impl MethodSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> MethodSpec {
        MethodSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            annotations: Vec::new(),
            arguments: Vec::new(),
            returns: None,
            sections: Sections::new(),
        }
    }

    pub fn push_annotation(&mut self, annotation: &AnnotationSpec) {
        self.annotations.push(annotation.clone());
    }

    pub fn push_argument(&mut self, argument: &ArgumentSpec) {
        self.arguments.push(argument.clone());
    }

    pub fn returns<I>(&mut self, returns: I)
        where I: AsTypeSpec
    {
        self.returns = Some(returns.as_type_spec())
    }

    pub fn push_statement(&mut self, statement: &Statement) {
        self.sections.push_statement(statement);
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        if let Some(ref type_) = self.returns {
            type_.imports(receiver);
        }

        for a in &self.arguments {
            a.imports(receiver);
        }

        self.sections.imports(receiver);
    }

    pub fn as_block(&self) -> Result<Block> {
        let mut open = Statement::new();

        if !self.annotations.is_empty() {
            let mut annotations = Statement::new();

            for a in &self.annotations {
                annotations.push_statement(a.as_statement()?);
            }

            open.push_statement(annotations.join(" "));
            open.push_literal(" ");
        }

        if !self.modifiers.is_empty() {
            open.push("$L ", stmt!(literal self.modifiers.format()?));
        }

        match self.returns {
            None => open.push("void ", vec![]),
            Some(ref returns) => open.push("$T ", stmt![type_spec returns]),
        }

        open.push("$L(", stmt!(literal self.name.clone()));

        if !self.arguments.is_empty() {
            let mut arguments: Statement = Statement::new();

            for a in &self.arguments {
                arguments.push_statement(a.as_statement()?);
            }

            let arguments: Statement = arguments.join(", ");
            open.push_statement(arguments);
        }

        open.push(")", vec![]);

        let mut block = Block::new();
        block.open(open);
        block.extend(&self.sections);

        Ok(block)
    }
}

#[derive(Debug)]
pub struct ClassSpec {
    modifiers: Modifiers,
    name: String,
    fields: Vec<FieldSpec>,
    constructors: Vec<ConstructorSpec>,
    methods: Vec<MethodSpec>,
}

impl ClassSpec {
    pub fn new(modifiers: Modifiers, name: &str) -> ClassSpec {
        ClassSpec {
            modifiers: modifiers,
            name: name.to_owned(),
            fields: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
        }
    }

    pub fn push_field(&mut self, field: &FieldSpec) {
        self.fields.push(field.clone());
    }

    pub fn push_constructor(&mut self, constructor: &ConstructorSpec) {
        self.constructors.push(constructor.clone());
    }

    pub fn push_method(&mut self, method: &MethodSpec) {
        self.methods.push(method.clone());
    }

    pub fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        for constructor in &self.constructors {
            constructor.imports(receiver);
        }

        for method in &self.methods {
            method.imports(receiver);
        }
    }

    pub fn as_block(&self) -> Result<Block> {
        let mut open = Statement::new();

        if !self.modifiers.is_empty() {
            open.push("$L ", stmt!(literal self.modifiers.format()?));
        }

        open.push("class $L", stmt!(literal self.name.clone()));

        let mut block = Block::new();
        block.open(open);

        /// TODO: figure out a better way...
        let mut first = true;

        for field in &self.fields {
            if first {
                first = false;
            } else {
                block.push_spacing();
            }

            block.push_statement(&field.as_statement()?);
        }

        for constructor in &self.constructors {
            if first {
                first = false;
            } else {
                block.push_spacing();
            }

            block.push_block(&constructor.as_block(&self.name)?);
        }

        for method in &self.methods {
            if first {
                first = false;
            } else {
                block.push_spacing();
            }

            block.push_block(&method.as_block()?);
        }

        Ok(block)
    }
}

#[derive(Debug)]
pub struct FileSpec {
    package: String,
    class: ClassSpec,
}

impl FileSpec {
    pub fn new(package: &str, class: ClassSpec) -> FileSpec {
        FileSpec {
            package: package.to_owned(),
            class: class,
        }
    }

    pub fn format(&self) -> Result<String> {
        let mut sections = Sections::new();

        sections.push_statement(&stmt!("package $L", literal self.package.clone()));
        sections.push_spacing();

        let mut receiver: BTreeSet<Type> = BTreeSet::new();

        self.class.imports(&mut receiver);

        let imports: Vec<Type> =
            receiver.into_iter().filter(|t| t.package != "java.lang").collect();

        if !imports.is_empty() {
            for t in imports {
                sections.push_statement(&stmt!("import $L.$L", literal t.package.clone(), literal t.name.clone()));
            }

            sections.push_spacing();
        }

        sections.push_block(&self.class.as_block()?);

        let mut out = String::new();

        for line in sections.format("", "  ")? {
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
        let reference = ::std::str::from_utf8(include_bytes!("tests/Test.java")).unwrap();

        let string_type = Type::new("java.lang", "String");
        let list_type = Type::new("java.util", "List");
        let json_creator_type = Type::new("com.fasterxml.jackson.annotation", "JsonCreator");
        let list_of_strings = list_type.with_arguments(vec![&string_type]);

        let values_field = FieldSpec::new(mods![Modifier::Private, Modifier::Final],
                                          &list_of_strings,
                                          "values");

        let mut values_argument =
            ArgumentSpec::new(mods![Modifier::Final], &list_of_strings, "values");

        let mut constructor = ConstructorSpec::new(mods![Modifier::Public]);
        constructor.push_annotation(&AnnotationSpec::new(json_creator_type));
        constructor.push_argument(&values_argument);
        constructor.push_statement(&stmt!("this.values = $N", name values_argument));

        let mut values_getter = MethodSpec::new(mods![Modifier::Public], "getValues");
        values_getter.returns(&list_of_strings);
        values_getter.push_statement(&stmt!("return this.$N", name values_field));

        let mut class = ClassSpec::new(mods![Modifier::Public], "Test");
        class.push_field(&values_field);
        class.push_constructor(&constructor);
        class.push_method(&values_getter);

        let mut file = FileSpec::new("se.tedro", class);

        let result = file.format().unwrap();

        println!("{}", result);

        assert_eq!(reference, result);
    }
}
