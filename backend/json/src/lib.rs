#[allow(unused)]
#[macro_use]
extern crate serde_derive;
#[allow(unused)]
#[macro_use]
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate serde_json;
extern crate serde;
extern crate toml;

mod collector;
mod json_backend;
mod json_compiler;
mod json_options;
mod listeners;

use self::backend::{ArgMatches, CompilerOptions, Environment, Options};
use self::backend::errors::*;
use self::json_backend::JsonBackend;
use self::json_options::JsonOptions;
use self::listeners::Listeners;
use manifest::{Lang, Manifest, NoModule, TryFromToml, self as m};
use std::path::Path;

const EXT: &str = "json";

#[derive(Default)]
pub struct JsonLang;

impl Lang for JsonLang {
    type Module = JsonModule;
}

#[derive(Debug)]
pub enum JsonModule {
}

impl TryFromToml for JsonModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> m::errors::Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> m::errors::Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

fn setup_listeners(modules: &[JsonModule]) -> Result<(JsonOptions, Box<Listeners>)> {
    let listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        match *module {
        }
    }

    let mut options = JsonOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile(
    env: Environment,
    _opts: Options,
    compiler_options: CompilerOptions,
    _matches: &ArgMatches,
    manifest: Manifest<JsonLang>,
) -> Result<()> {
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = JsonBackend::new(env, options, listeners);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}
