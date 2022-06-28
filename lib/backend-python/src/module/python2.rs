//! Module that adds fasterxml annotations to generated classes.

use crate::utils::VersionHelper;
use crate::Options;
use backend::Initializer;
use genco::prelude::*;
use genco::tokens::ItemStr;
use reproto_core::errors::Result;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Config {}

pub(crate) struct Module {
    #[allow(dead_code)]
    config: Config,
}

impl Module {
    pub(crate) fn new(config: Config) -> Module {
        Module { config }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Python2VersionHelper {}

impl VersionHelper for Python2VersionHelper {
    fn is_string(&self, var: &ItemStr) -> Tokens<Python> {
        quote!(isinstance(#var, unicode))
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Options) -> Result<()> {
        options.version_helper = Rc::new(Python2VersionHelper {});

        Ok(())
    }
}
