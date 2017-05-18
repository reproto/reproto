use super::class_spec::ClassSpec;
use super::interface_spec::InterfaceSpec;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub enum ElementSpec {
    Class(ClassSpec),
    Interface(InterfaceSpec),
    Statement(Statement),
    Literal(Vec<String>),
}
