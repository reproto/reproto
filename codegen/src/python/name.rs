#[derive(Debug, Clone)]
pub enum Name {
    Imported(ImportedName),
    BuiltIn(BuiltInName),
    Local(LocalName),
}

impl Name {
    pub fn imported(module: &str, name: &str) -> ImportedName {
        ImportedName {
            module: module.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn built_in(name: &str) -> BuiltInName {
        BuiltInName { name: name.to_owned() }
    }

    pub fn local(name: &str) -> LocalName {
        LocalName { name: name.to_owned() }
    }

    pub fn format(&self) -> String {
        match *self {
            Name::Imported(ref imported) => {
                format!("{}.{}", imported.module, imported.name.clone())
            }
            Name::BuiltIn(ref built_in) => built_in.name.clone(),
            Name::Local(ref local) => local.name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImportedName {
    pub module: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct BuiltInName {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct LocalName {
    pub name: String,
}

pub trait AsName {
    fn as_name(self) -> Name;
}

impl<'a, A> AsName for &'a A
    where A: AsName + Clone
{
    fn as_name(self) -> Name {
        self.clone().as_name()
    }
}

impl AsName for Name {
    fn as_name(self) -> Name {
        self
    }
}

impl AsName for ImportedName {
    fn as_name(self) -> Name {
        Name::Imported(self)
    }
}

impl AsName for BuiltInName {
    fn as_name(self) -> Name {
        Name::BuiltIn(self)
    }
}

impl AsName for LocalName {
    fn as_name(self) -> Name {
        Name::Local(self)
    }
}
