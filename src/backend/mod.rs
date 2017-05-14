pub mod java;

use parser::ast;
use options::Options;

use errors::*;

pub trait Backend {
    fn add_file(&mut self, file: ast::File) -> Result<()>;

    fn process(&self, options: &Options) -> Result<()>;
}
