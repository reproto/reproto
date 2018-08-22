//! Reporter of diagnostics.

use {Diagnostics, SourceDiagnostics};

pub trait Reporter {
    /// Report a collection of diagnostics.
    fn diagnostics(&mut self, diagnostics: Diagnostics);

    /// Report a collection of source diagnostics.
    fn source_diagnostics(&mut self, source_diagnostics: SourceDiagnostics);

    /// Check if reporter has diagnostics.
    fn has_diagnostics(&self) -> bool;
}

pub enum Reported {
    /// A reported set of diagnostics.
    Diagnostics(Diagnostics),
    /// A reported set of diagnostics where each item has a source.
    SourceDiagnostics(SourceDiagnostics),
}

impl Reporter for Vec<Reported> {
    fn diagnostics(&mut self, diagnostics: Diagnostics) {
        self.push(Reported::Diagnostics(diagnostics));
    }

    fn source_diagnostics(&mut self, source_diagnostics: SourceDiagnostics) {
        self.push(Reported::SourceDiagnostics(source_diagnostics));
    }

    fn has_diagnostics(&self) -> bool {
        !self.is_empty()
    }
}
