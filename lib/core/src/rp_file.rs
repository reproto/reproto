//! File declarations

use super::RpDecl;
use std::collections::LinkedList;

#[derive(Debug, Clone, Serialize)]
pub struct RpFile {
    pub comment: Vec<String>,
    pub decls: Vec<RpDecl>,
}

/// Iterator over all declarations in a file.
#[allow(linkedlist)]
pub struct ForEachDecl<'a> {
    queue: LinkedList<&'a RpDecl>,
}

impl<'a> Iterator for ForEachDecl<'a> {
    type Item = &'a RpDecl;

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
