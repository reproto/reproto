use genco::{Cons, Dart, IntoTokens, Quoted, Tokens};

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

/// Assert that the given expression has the expected type.
pub struct AssertType<'el, E: 'el>(pub Dart<'el>, pub E);

impl<'el, E: 'el> IntoTokens<'el, Dart<'el>> for AssertType<'el, E>
where
    E: Into<Cons<'el>>,
{
    fn into_tokens(self) -> Tokens<'el, Dart<'el>> {
        let AssertType(ty, expr) = self;
        let expr = expr.into();
        let mut t = toks!();
        push!(t, "if (!(", expr, " is ", ty, ")) {");
        nested!(t, "throw 'expected ", ty, ", but got: $", expr, "';");
        push!(t, "}");
        t
    }
}

/// Assert that the given expression has the expected type.
pub struct AssertNotNull<E>(pub E);

impl<'el, E: 'el> IntoTokens<'el, Dart<'el>> for AssertNotNull<E>
where
    E: Into<Cons<'el>>,
{
    fn into_tokens(self) -> Tokens<'el, Dart<'el>> {
        let AssertNotNull(expr) = self;
        let expr = expr.into();
        let mut t = Tokens::new();
        push!(t, "if (", expr, " == null) {");
        nested!(t, "throw ", "expected value but was null".quoted(), ";");
        push!(t, "}");
        t
    }
}
