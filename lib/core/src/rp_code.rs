//! Literal code segments

use crate::Loc;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpContext {
    Csharp {},
    Go {},
    Java {
        /// Imports to perform for the given code block.
        /// These will be de-duplicated by the java backend.
        imports: Vec<Loc<String>>,
    },
    Js {},
    Json {},
    Python {},
    Reproto {},
    Rust {},
    Swift {},
    Dart {},
}

#[derive(Debug, Clone, Serialize)]
pub struct RpCode {
    pub context: RpContext,
    pub lines: Vec<String>,
}
