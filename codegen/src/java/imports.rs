use super::_type::{Type, ClassType};
use super::element_spec::ElementSpec;
use super::elements::Elements;
use super::statement::Statement;
use super::variable::Variable;

pub trait ImportReceiver {
    fn receive(&mut self, ty: &ClassType);

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
    fn imports<I>(&self, &mut I) where I: ImportReceiver;
}

impl Imports for ElementSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            ElementSpec::Elements(ref elements) => receiver.import_all(elements),
            ElementSpec::Statement(ref statement) => statement.imports(receiver),
            ElementSpec::Nested(ref nested) => {
                (*nested).imports(receiver);
            }
            _ => {}
        }
    }
}

impl Imports for Variable {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Variable::Type(ref ty) => {
                ty.imports(receiver);
            }
            Variable::Statement(ref stmt) => {
                stmt.imports(receiver);
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

impl Imports for Elements {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.elements);
    }
}

impl Imports for ClassType {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.receive(self);
        receiver.import_all(&self.arguments);
    }
}

impl Imports for Type {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Type::Class(ref class) => class.imports(receiver),
            _ => {}
        };
    }
}
