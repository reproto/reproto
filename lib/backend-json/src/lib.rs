#[allow(unused)]
#[macro_use]
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate serde;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod collector;
mod json_backend;
mod json_compiler;
mod json_options;
mod listeners;

use backend::Environment;
use core::Context;
use core::errors::*;
use json_backend::JsonBackend;
use json_options::JsonOptions;
use listeners::Listeners;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use std::path::Path;
use std::rc::Rc;

const EXT: &str = "json";

#[derive(Default)]
pub struct JsonLang;

impl Lang for JsonLang {
    type Module = JsonModule;

    fn comment(input: &str) -> Option<String> {
        Some(input.to_string())
    }
}

#[derive(Debug)]
pub enum JsonModule {
}

impl TryFromToml for JsonModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

fn setup_listeners(modules: &[JsonModule]) -> Result<(JsonOptions, Box<Listeners>)> {
    let listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        match *module {}
    }

    let mut options = JsonOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile(ctx: Rc<Context>, env: Environment, manifest: Manifest<JsonLang>) -> Result<()> {
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = JsonBackend::new(env, options, listeners);
    let handle = ctx.filesystem(manifest.output.as_ref().map(AsRef::as_ref))?;
    let compiler = backend.compiler(handle.as_ref())?;
    compiler.compile()
}
