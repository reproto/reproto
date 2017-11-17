//! Code mixin for building tokens.

use core::{Loc, RpCode};
use for_context::ForContext;
use genco::{Custom, IntoTokens, Tokens};

pub struct Code<'el>(pub &'el Vec<Loc<RpCode>>, pub &'static str);

impl<'el, C: 'el> IntoTokens<'el, C> for Code<'el>
where
    C: Custom,
{
    fn into_tokens(self) -> Tokens<'el, C> {
        let mut tokens = Tokens::new();

        for code in self.0.iter().for_context(self.1) {
            for line in &code.lines {
                tokens.push(line.as_str());
            }
        }

        tokens
    }
}
