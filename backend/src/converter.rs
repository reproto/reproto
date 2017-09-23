//! # Converter for core data structures into processor-specific ones.

use codeviz_common::Element;
use codeviz_common::VariableFormat;
use container::Container;
use core::{Pos, RpName};
use errors::*;

pub trait Converter {
    type Elements: Clone + Into<Element<Self::Variable>> + Container<Self::Variable>;
    type Stmt: Clone + Into<Element<Self::Variable>>;
    type Type;
    type Variable: Clone + VariableFormat;

    fn new_var(&self, name: &str) -> Self::Stmt;

    fn convert_type(&self, pos: &Pos, name: &RpName) -> Result<Self::Type>;

    fn convert_constant(&self, pos: &Pos, name: &RpName) -> Result<Self::Type> {
        self.convert_type(pos, name)
    }
}
