#[macro_use]
extern crate log;
#[macro_use]
extern crate genco;
extern crate reproto_backend as backend;
extern crate reproto_core as core;

mod listeners;
mod python_backend;
mod python_compiler;
mod python_field;
mod python_file_spec;
mod python_options;

use self::backend::{ArgMatches, CompilerOptions, Environment, Options};
use self::backend::errors::*;
use self::listeners::Listeners;
use self::python_backend::PythonBackend;
use self::python_options::PythonOptions;

const TYPE: &str = "type";
const TYPE_SEP: &str = "_";
const INIT_PY: &str = "__init__.py";
const EXT: &str = "py";
const PYTHON_CONTEXT: &str = "python";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn setup_listeners(modules: Vec<String>) -> Result<(PythonOptions, Box<Listeners>)> {
    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        listeners.push(setup_module(module.as_str())?);
    }

    let mut options = PythonOptions::new();

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
    let backend = PythonBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}

pub fn verify(env: Environment, opts: Options, _matches: &ArgMatches) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = PythonBackend::new(env, options, listeners, id_converter);
    backend.verify()
}
