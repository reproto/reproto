extern crate reproto_backend as backend;
extern crate reproto_core as core;
#[macro_use]
extern crate genco;

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

const TYPE: &str = "type";
const TYPE_SEP: &str = "_";
const EXT: &str = "js";
const JS_CONTEXT: &str = "js";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn setup_listeners(modules: Vec<String>) -> Result<(JsOptions, Box<Listeners>)> {
    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        listeners.push(setup_module(module.as_str())?);
    }

    let mut options = JsOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile(
    env: Environment,
    opts: Options,
    compiler_options: CompilerOptions,
    _matches: &ArgMatches,
) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = JsBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}

pub fn verify(env: Environment, opts: Options, _matches: &ArgMatches) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = JsBackend::new(env, options, listeners, id_converter);
    backend.verify()
}
