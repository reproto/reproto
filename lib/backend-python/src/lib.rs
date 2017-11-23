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


use self::ErrorKind::*;
use backend::{ArgMatches, Environment};
use backend::errors::*;
use core::Context;
use listeners::Listeners;
use manifest::{Lang, Manifest, NoModule, TryFromToml, self as m};
use python_backend::PythonBackend;
use python_options::PythonOptions;
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
    _matches: &ArgMatches,
    manifest: Manifest<PythonLang>,
) -> Result<()> {
    let out = manifest.output.ok_or(MissingOutput)?;
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = PythonBackend::new(env, options, listeners);
    let compiler = backend.compiler(out)?;
    compiler.compile()
}
