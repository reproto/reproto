#[macro_use]
extern crate genco;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate serde;
extern crate toml;

#[macro_use]
mod utils;
mod js_field;
mod listeners;
mod js_backend;
mod js_compiler;
mod js_file_spec;
mod js_options;

use self::backend::{ArgMatches, CompilerOptions, Environment, Options};
use self::backend::errors::*;
use self::js_backend::JsBackend;
use self::js_options::JsOptions;
use self::listeners::Listeners;
use core::Context;
use manifest::{Lang, Manifest, NoModule, TryFromToml, self as m};
use std::path::Path;
use std::rc::Rc;

const TYPE: &str = "type";
const TYPE_SEP: &str = "_";
const EXT: &str = "js";
const JS_CONTEXT: &str = "js";

#[derive(Default)]
pub struct JsLang;

impl Lang for JsLang {
    type Module = JsModule;
}

#[derive(Debug)]
pub enum JsModule {
}

impl TryFromToml for JsModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> m::errors::Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> m::errors::Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

fn setup_listeners(modules: &[JsModule]) -> Result<(JsOptions, Box<Listeners>)> {
    let listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        match *module {
        }
    }

    let mut options = JsOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile(
    _ctx: Rc<Context>,
    env: Environment,
    opts: Options,
    compiler_options: CompilerOptions,
    _matches: &ArgMatches,
    manifest: Manifest<JsLang>,
) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = JsBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}
