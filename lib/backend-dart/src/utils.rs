use genco::{IntoTokens, Dart, Tokens};

/// Documentation comments.
pub struct Comments<'el, S: 'el>(pub &'el [S]);

impl<'el, S: 'el + AsRef<str>> IntoTokens<'el, Dart<'el>> for Comments<'el, S> {
    fn into_tokens(self) -> Tokens<'el, Dart<'el>> {
        let mut t = Tokens::new();

        for c in self.0.iter() {
            let line = c.as_ref();

            if line.is_empty() {
                t.push("///");
            } else {
                t.push(toks!["/// ", line]);
            }
        }

        t
    }
}

/// Repr attribute.
#[allow(unused)]
pub struct Repr<S>(pub S);

impl<'el, S: Into<Dart<'el>>> IntoTokens<'el, Dart<'el>> for Repr<S> {
    fn into_tokens(self) -> Tokens<'el, Dart<'el>> {
        toks!["#[repr(", self.0.into(), ")]"]
    }
}
