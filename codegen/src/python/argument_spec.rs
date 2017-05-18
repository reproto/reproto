#[derive(Debug, Clone)]
pub struct ArgumentSpec {
    pub name: String,
}

impl ArgumentSpec {
    pub fn new(name: &str) -> ArgumentSpec {
        ArgumentSpec { name: name.to_owned() }
    }
}

pub trait AsArgumentSpec {
    fn as_argument_spec(self) -> ArgumentSpec;
}

impl<'a, A> AsArgumentSpec for &'a A
    where A: AsArgumentSpec + Clone
{
    fn as_argument_spec(self) -> ArgumentSpec {
        self.clone().as_argument_spec()
    }
}

impl AsArgumentSpec for ArgumentSpec {
    fn as_argument_spec(self) -> ArgumentSpec {
        self
    }
}
