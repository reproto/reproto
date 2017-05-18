use super::class_spec::ClassSpec;
use super::element_spec::ElementSpec;
use super::statement::Statement;
use super::variable::Variable;
use super::name::{Name, ImportedName};

pub trait ImportReceiver {
    fn receive(&mut self, name: &ImportedName);

    fn import_all<T>(&mut self, sources: &Vec<T>)
        where T: Imports,
              Self: Sized
    {
        for source in sources {
            source.imports(self);
        }
    }
}

pub trait Imports {
    fn imports<I>(&self, receiver: &mut I) where I: ImportReceiver;
}

impl Imports for Name {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Name::Imported(ref imported) => receiver.receive(imported),
            _ => {}
        };
    }
}

impl Imports for Variable {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Variable::Statement(ref stmt) => {
                stmt.imports(receiver);
            }
            Variable::Name(ref name) => {
                name.imports(receiver);
            }
            _ => {}
        }
    }
}

impl Imports for Statement {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.parts);
    }
}

impl Imports for ElementSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            ElementSpec::Statement(ref statement) => {
                statement.imports(receiver);
            }
            ElementSpec::Elements(ref elements) => {
                receiver.import_all(elements);
            }
            ElementSpec::Nested(ref elements) => {
                receiver.import_all(elements);
            }
            _ => {}
        };
    }
}

impl Imports for ClassSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.elements);
    }
}
