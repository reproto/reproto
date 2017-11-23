#[macro_use]
extern crate log;
#[macro_use]
extern crate genco;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate serde;
extern crate toml;

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
use core::Context;
use manifest::{Lang, Manifest, NoModule, TryFromToml, self as m};
use std::path::Path;
use std::rc::Rc;

const TYPE: &str = "type";
const TYPE_SEP: &str = "_";
const INIT_PY: &str = "__init__.py";
const EXT: &str = "py";
const PYTHON_CONTEXT: &str = "python";

#[derive(Default)]
pub struct PythonLang;

impl Lang for PythonLang {
    type Module = PythonModule;
}

#[derive(Debug)]
pub enum PythonModule {
}

impl TryFromToml for PythonModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> m::errors::Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> m::errors::Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

pub fn setup_listeners(modules: &[PythonModule]) -> Result<(PythonOptions, Box<Listeners>)> {
    let listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        match *module {
        }
    }

    let mut options = PythonOptions::new();

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
    manifest: Manifest<PythonLang>,
) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = PythonBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}
