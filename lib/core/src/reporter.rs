//! Reporter of diagnostics.

use {Diagnostic, Diagnostics, Source, SourceDiagnostics};

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

impl Reported {
    /// Iterate over all diagnostics with sources.
    pub fn diagnostics_with_sources<'a>(
        &'a self,
    ) -> impl Iterator<Item = (&'a Source, &'a Diagnostic)> {
        let mut out = Vec::new();

        match *self {
            Reported::Diagnostics(ref diagnostics) => {
                for item in diagnostics.items() {
                    out.push((&diagnostics.source, item));
                }
            }
            Reported::SourceDiagnostics(ref diagnostics) => {
                for item in diagnostics.items() {
                    out.push((&item.0, &item.1));
                }
            }
        }

        out.into_iter()
    }
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
