use super::*;

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

impl Imports for ClassSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.annotations);
        receiver.import_all(&self.constructors);
        receiver.import_all(&self.methods);
    }
}

impl Imports for InterfaceSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.annotations);
        receiver.import_all(&self.elements);
    }
}

impl Imports for ElementSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            ElementSpec::Class(ref class) => class.imports(receiver),
            ElementSpec::Interface(ref interface) => interface.imports(receiver),
            ElementSpec::Statement(ref statement) => statement.imports(receiver),
            _ => {}
        }
    }
}

impl Imports for Section {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        match *self {
            Section::Block(ref block) => block.imports(receiver),
            Section::Statement(ref statement) => statement.imports(receiver),
            _ => {}
        };
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

impl Imports for Sections {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        receiver.import_all(&self.sections);
    }
}

impl Imports for Block {
    fn imports<I>(&self, receiver: &mut I)
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
            Type::Primitive(_) => {}
            Type::Class(ref class) => class.imports(receiver),
        };
    }
}

impl Imports for FieldSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.ty.imports(receiver);
    }
}

impl Imports for ConstructorSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.sections.imports(receiver);
        receiver.import_all(&self.annotations);
        receiver.import_all(&self.arguments);
    }
}

impl Imports for AnnotationSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.ty.imports(receiver);

        for a in &self.arguments {
            a.imports(receiver);
        }
    }
}

impl Imports for ArgumentSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        self.ty.imports(receiver);

        for a in &self.annotations {
            a.imports(receiver);
        }
    }
}

impl Imports for MethodSpec {
    fn imports<I>(&self, receiver: &mut I)
        where I: ImportReceiver
    {
        if let Some(ref ty) = self.returns {
            ty.imports(receiver);
        }

        receiver.import_all(&self.arguments);
        self.sections.imports(receiver);
    }
}
