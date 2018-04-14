use genco::{IntoTokens, Rust, Tokens};

/// Documentation comments.
pub struct Comments<'el, S: 'el>(pub &'el [S]);

impl<'el, S: 'el + AsRef<str>> IntoTokens<'el, Rust<'el>> for Comments<'el, S> {
    fn into_tokens(self) -> Tokens<'el, Rust<'el>> {
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

impl<'el, S: Into<Rust<'el>>> IntoTokens<'el, Rust<'el>> for Repr<S> {
    fn into_tokens(self) -> Tokens<'el, Rust<'el>> {
        toks!["#[repr(", self.0.into(), ")]"]
    }
}
