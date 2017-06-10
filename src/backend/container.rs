use codeviz::common::Element;
use codeviz::common::Elements;
use codeviz::common::VariableFormat;

/// A container is a data structure that can push elements of itself.
pub trait Container<Var>
    where Var: Clone,
          Var: VariableFormat
{
    fn new() -> Self;

    fn push<E>(&mut self, other: E) where E: Clone + Into<Element<Var>>;

    fn join<E>(self, other: E) -> Self where E: Clone + Into<Element<Var>>;
}

impl<Var> Container<Var> for Elements<Var>
    where Var: Clone,
          Var: VariableFormat
{
    fn new() -> Elements<Var> {
        Elements::new()
    }

    fn push<E>(&mut self, other: E)
        where E: Clone + Into<Element<Var>>
    {
        Elements::push(self, other)
    }

    fn join<E>(self, other: E) -> Self
        where E: Clone + Into<Element<Var>>
    {
        Elements::join(self, other)
    }
}
