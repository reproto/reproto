/// Complete types, including generic arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ClassType {
    pub package: String,
    pub name: String,
    pub arguments: Vec<Type>,
}

impl ClassType {
    pub fn new(package: &str, name: &str, arguments: Vec<Type>) -> ClassType {
        ClassType {
            package: package.to_owned(),
            name: name.to_owned(),
            arguments: arguments,
        }
    }

    pub fn with_arguments<A>(&self, arguments: Vec<A>) -> ClassType
        where A: AsType
    {
        let arguments = arguments.into_iter().map(AsType::as_type).collect();
        ClassType::new(&self.package, &self.name, arguments)
    }

    pub fn extend(&self, part: &str) -> ClassType {
        ClassType::new(&self.package,
                       &format!("{}.{}", self.name, part),
                       self.arguments.clone())
    }

    pub fn to_raw(&self) -> ClassType {
        ClassType::new(&self.package, &self.name, vec![])
    }

    pub fn format(&self, level: usize) -> String {
        let mut out = String::new();

        out.push_str(&self.name);

        if !self.arguments.is_empty() {
            let mut arguments = Vec::new();

            let level = level + 1;

            for g in &self.arguments {
                arguments.push(g.format(level));
            }

            let joined = arguments.join(", ");

            out.push('<');
            out.push_str(&joined);
            out.push('>');
        }

        out
    }
}

pub trait AsClassType {
    fn as_class_type(self) -> ClassType;
}

impl<'a, A> AsClassType for &'a A
    where A: AsClassType + Clone
{
    fn as_class_type(self) -> ClassType {
        self.clone().as_class_type()
    }
}

impl AsClassType for ClassType {
    fn as_class_type(self) -> ClassType {
        self
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Local {
    pub name: String,
}

impl Local {
    pub fn new(name: &str) -> Local {
        Local { name: name.to_owned() }
    }

    pub fn format(&self, _level: usize) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct PrimitiveType {
    pub primitive: String,
    pub boxed: String,
}

impl PrimitiveType {
    pub fn new(primitive: &str, boxed: &str) -> PrimitiveType {
        PrimitiveType {
            primitive: primitive.to_owned(),
            boxed: boxed.to_owned(),
        }
    }

    pub fn format(&self, level: usize) -> String {
        if level <= 0 {
            self.primitive.clone()
        } else {
            self.boxed.clone()
        }
    }

    pub fn as_boxed(&self) -> ClassType {
        ClassType::new("java.lang", &self.boxed, vec![])
    }
}

/// Raw (importable) types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    Class(ClassType),
    Local(Local),
}

impl Type {
    pub fn primitive(primitive: &str, boxed: &str) -> PrimitiveType {
        PrimitiveType::new(primitive, boxed)
    }

    pub fn class(package: &str, name: &str) -> ClassType {
        ClassType::new(package, name, vec![])
    }

    pub fn local(name: &str) -> Local {
        Local::new(name)
    }

    pub fn format(&self, level: usize) -> String {
        match *self {
            Type::Primitive(ref primitive) => primitive.format(level),
            Type::Class(ref class) => class.format(level),
            Type::Local(ref local) => local.format(level),
        }
    }
}

pub trait AsType {
    fn as_type(self) -> Type;
}

impl<'a, A> AsType for &'a A
    where A: AsType + Clone
{
    fn as_type(self) -> Type {
        self.clone().as_type()
    }
}

impl AsType for Type {
    fn as_type(self) -> Type {
        self
    }
}

/// Implementation for ClassType to Type conversion.
impl AsType for ClassType {
    fn as_type(self) -> Type {
        Type::Class(self)
    }
}

/// Implementation for PrimitiveType to Type conversion.
impl AsType for PrimitiveType {
    fn as_type(self) -> Type {
        Type::Primitive(self)
    }
}

impl AsType for Local {
    fn as_type(self) -> Type {
        Type::Local(self)
    }
}
