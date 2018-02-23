#[macro_use]
extern crate genco;
#[macro_use]
extern crate log;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate reproto_naming as naming;
extern crate reproto_trans as trans;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod codegen;
mod python_backend;
mod python_compiler;
mod python_field;
mod python_file_spec;
mod options;
mod module;
mod utils;

use backend::Initializer;
use core::Context;
use core::errors::*;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use options::Options;
use python_backend::PythonBackend;
use std::path::Path;
use std::rc::Rc;
use trans::Environment;

const TYPE_SEP: &str = "_";
const INIT_PY: &str = "__init__.py";
const EXT: &str = "py";
const PYTHON_CONTEXT: &str = "python";

#[derive(Default)]
pub struct PythonLang;

impl Lang for PythonLang {
    type Module = PythonModule;

    fn comment(input: &str) -> Option<String> {
        Some(format!("# {}", input))
    }
}

#[derive(Debug)]
pub enum PythonModule {
    Requests(module::RequestsConfig),
}

impl TryFromToml for PythonModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::PythonModule::*;

        let result = match id {
            "requests" => Requests(module::RequestsConfig::default()),
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::PythonModule::*;

        let result = match id {
            "requests" => Requests(value.try_into()?),
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub fn setup_options(modules: Vec<PythonModule>) -> Result<Options> {
    use self::PythonModule::*;

    let mut options = Options::new();

    for module in modules {
        let initializer: Box<Initializer<Options = Options>> = match module {
            Requests(config) => Box::new(module::Requests::new(config)),
        };

        initializer.initialize(&mut options)?;
    }

    Ok(options)
}

pub fn compile(ctx: Rc<Context>, env: Environment, manifest: Manifest<PythonLang>) -> Result<()> {
    let options = setup_options(manifest.modules)?;
    let backend = PythonBackend::new(env, options);
    let handle = ctx.filesystem(manifest.output.as_ref().map(AsRef::as_ref))?;
    let compiler = backend.compiler(handle.as_ref())?;
    compiler.compile()
}
