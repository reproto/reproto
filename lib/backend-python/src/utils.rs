use genco::{Cons, Element, IntoTokens, Python, Quoted, Tokens};
use std::fmt;

pub struct BlockComment<'el>(pub &'el [String]);

impl<'el> IntoTokens<'el, Python<'el>> for BlockComment<'el> {
    fn into_tokens(self) -> Tokens<'el, Python<'el>> {
        let c: Tokens<'el, Python<'el>> = self
            .0
            .iter()
            .map(|c| Element::Literal(c.as_str().into()))
            .collect();

        let mut toks = Tokens::new();

        toks.push("\"\"\"");
        toks.push(c.join(Element::Line));
        toks.push("\"\"\"");

        toks
    }
}

pub struct IfNoneThen<C, D>(pub C, pub D);

impl<'el, C, D> IntoTokens<'el, Python<'el>> for IfNoneThen<C, D>
where
    C: Into<Tokens<'el, Python<'el>>>,
    D: Into<Tokens<'el, Python<'el>>>,
{
    fn into_tokens(self) -> Tokens<'el, Python<'el>> {
        let mut toks = Tokens::new();

        let cond = self.0.into();
        let def = self.1.into();

        toks.push(toks!["if ", cond.clone(), " is None:"]);
        toks.nested(toks![cond, " = ", def]);

        toks
    }
}

#[derive(Clone)]
pub struct IfNoneRaise<C, M>(pub C, pub M);

impl<'el, C, M> IntoTokens<'el, Python<'el>> for IfNoneRaise<C, M>
where
    C: Into<Tokens<'el, Python<'el>>>,
    M: Clone + Into<Cons<'el>>,
{
    fn into_tokens(self) -> Tokens<'el, Python<'el>> {
        let IfNoneRaise(var, m) = self;

        let mut t = Tokens::new();
        push!(t, "if ", var.into(), " is None:");
        nested!(t, "raise ", Exception(m));
        t
    }
}

#[derive(Clone)]
pub struct Exception<M>(pub M);

impl<'el, M> From<Exception<M>> for Element<'el, Python<'el>>
where
    M: Into<Cons<'el>>,
{
    fn from(value: Exception<M>) -> Self {
        toks!["Exception(", value.0.into().quoted(), ")"].into()
    }
}

/// Utilities for handling differences between python versions.
pub trait VersionHelper: fmt::Debug {
    /// Check if the given variable is a string.
    ///
    /// In Python 3, strings are `str` objects.
    /// In Python 2, strings would be `unicode` objects.
    fn is_string<'el>(&self, var: Cons<'el>) -> Tokens<'el, Python<'el>>;
}
