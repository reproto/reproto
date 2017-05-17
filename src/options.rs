use std::path::PathBuf;
use naming;

pub struct Options {
    pub out_path: PathBuf,
    pub package_prefix: Option<String>,
    pub id_converter: Option<Box<naming::Naming>>,
}
