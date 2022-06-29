use genco::prelude::*;
use genco::tokens::FormatInto;

pub fn is_defined<T>(expr: T) -> Tokens<JavaScript>
where
    T: Copy + FormatInto<JavaScript>,
{
    quote!($expr !== null && $expr !== undefined)
}

pub fn is_not_defined<T>(expr: T) -> Tokens<JavaScript>
where
    T: Copy + FormatInto<JavaScript>,
{
    quote!($expr === null || $expr === undefined)
}
