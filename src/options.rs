use naming;
use std::path::PathBuf;

pub struct Options {
    pub out_path: PathBuf,
    pub package_prefix: Option<String>,
    pub id_converter: Option<Box<naming::Naming>>,
    pub modules: Vec<String>,
}
