//! Reporter of diagnostics.

use Diagnostics;

pub trait Reporter {
    /// Report a collection of diagnostics.
    fn diagnostics(&mut self, diagnostics: Diagnostics);

    /// Check if reporter has diagnostics.
    fn has_diagnostics(&self) -> bool;
}

impl Reporter for Vec<Diagnostics> {
    fn diagnostics(&mut self, diagnostics: Diagnostics) {
        self.push(diagnostics);
    }

    fn has_diagnostics(&self) -> bool {
        !self.is_empty()
    }
}
