use std::path::PathBuf;

pub struct Options {
    pub out_path: PathBuf,
    pub package_prefix: Option<String>,
}
