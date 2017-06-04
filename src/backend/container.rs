/// A container is a data structure that can push elements of itself.
pub trait Container {
    fn push(&mut self, other: &Self);
}
