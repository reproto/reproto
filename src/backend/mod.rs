pub mod java;

use environment::Environment;
use options::Options;
use parser::ast;

use errors::*;

pub type TypeId = (ast::Package, String);

pub trait Backend {
    fn process(&self, options: &Options, env: &Environment) -> Result<()>;
}
