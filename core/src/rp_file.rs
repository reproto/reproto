//! File declarations

use super::{Loc, RpDecl, RpOptionDecl};
use std::collections::LinkedList;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize)]
pub struct RpFile {
    pub comment: Vec<String>,
    pub options: Vec<Loc<RpOptionDecl>>,
    pub decls: Vec<Rc<Loc<RpDecl>>>,
}

/// Iterator over all declarations in a file.
pub struct ForEachDecl<'a> {
    queue: LinkedList<&'a Rc<Loc<RpDecl>>>,
}

impl<'a> Iterator for ForEachDecl<'a> {
    type Item = &'a Rc<Loc<RpDecl>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(decl) = self.queue.pop_front() {
            self.queue.extend(decl.decls());
            Some(decl)
        } else {
            None
        }
    }
}

impl RpFile {
    /// Iterate over all declarations in file.
    pub fn for_each_decl(&self) -> ForEachDecl {
        let mut queue = LinkedList::new();
        queue.extend(self.decls.iter());
        ForEachDecl { queue: queue }
    }
}
