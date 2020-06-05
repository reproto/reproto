use genco::prelude::*;
use genco::tokens::{static_literal, FormatInto, ItemStr};

/// Documentation comments.
pub struct Comments<I>(pub I);

impl<I> FormatInto<Dart> for Comments<I>
where
    I: IntoIterator,
    I::Item: Into<ItemStr>,
{
    fn format_into(self, t: &mut Tokens<Dart>) {
        for line in self.0.into_iter() {
            let line = line.into();

            t.push();

            if line.is_empty() {
                t.append(static_literal("///"));
            } else {
                t.append(static_literal("///"));
                t.space();
                t.append(line);
            }
        }
    }
}
