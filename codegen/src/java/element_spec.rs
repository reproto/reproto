use super::block::{AsBlock, Block};
use super::class_spec::ClassSpec;
use super::interface_spec::InterfaceSpec;
use super::section::Sections;
use super::statement::Statement;

#[derive(Debug, Clone)]
pub enum ElementSpec {
    Class(ClassSpec),
    Interface(InterfaceSpec),
    Statement(Statement),
    Literal(Vec<String>),
}

impl ElementSpec {
    pub fn add_to_block(&self, target: &mut Block) {
        match *self {
            ElementSpec::Class(ref class) => {
                target.push(class.as_block());
            }
            ElementSpec::Interface(ref interface) => {
                target.push(interface.as_block());
            }
            ElementSpec::Statement(ref statement) => {
                target.push(statement);
            }
            ElementSpec::Literal(ref content) => {
                target.push(content);
            }
        };
    }

    pub fn add_to_sections(&self, target: &mut Sections) {
        match *self {
            ElementSpec::Class(ref class) => {
                target.push(class.as_block());
            }
            ElementSpec::Interface(ref interface) => {
                target.push(interface.as_block());
            }
            ElementSpec::Statement(ref statement) => {
                target.push(statement);
            }
            ElementSpec::Literal(ref content) => {
                target.push(content);
            }
        };
    }
}
