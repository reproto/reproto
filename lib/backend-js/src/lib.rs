#[macro_use]
extern crate genco;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate reproto_naming as naming;
extern crate reproto_trans as trans;
extern crate serde;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
mod utils;
mod js_field;
mod listeners;
mod js_backend;
mod js_compiler;
mod js_file_spec;
mod js_options;

use core::Context;
use core::errors::*;
use js_backend::JsBackend;
use js_options::JsOptions;
use listeners::Listeners;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use std::path::Path;
use std::rc::Rc;
use trans::Environment;

const TYPE_SEP: &str = "_";
const EXT: &str = "js";
const JS_CONTEXT: &str = "js";

#[derive(Default)]
pub struct JsLang;

impl Lang for JsLang {
    type Module = JsModule;

    fn comment(input: &str) -> Option<String> {
        Some(format!("# {}", input))
    }
}

#[derive(Debug)]
pub enum JsModule {
}

impl TryFromToml for JsModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

fn setup_listeners(modules: &[JsModule]) -> Result<(JsOptions, Box<Listeners>)> {
    let listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        match *module {}
    }

    let mut options = JsOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile(ctx: Rc<Context>, env: Environment, manifest: Manifest<JsLang>) -> Result<()> {
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = JsBackend::new(env, options, listeners);
    let handle = ctx.filesystem(manifest.output.as_ref().map(AsRef::as_ref))?;
    let compiler = backend.compiler(handle.as_ref())?;
    compiler.compile()
}
