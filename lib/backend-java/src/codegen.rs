/// Code generator for the given path.

use backend::errors::Result;
use std::path::Path;

pub trait Codegen {
    /// Build the given piece of code in the given path.
    fn generate(&self, out_path: &Path) -> Result<()>;
}
