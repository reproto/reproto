use std::path::{Path, PathBuf};
use super::*;

pub struct FileObjects {
    path: PathBuf,
}

impl FileObjects {
    pub fn new<P: AsRef<Path> + ?Sized>(path: &P) -> FileObjects {
        FileObjects { path: path.as_ref().to_owned() }
    }
}

impl Objects for FileObjects {}
