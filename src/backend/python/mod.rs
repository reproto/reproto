mod field;
mod listeners;
mod python_backend;
mod python_compiler;
mod python_options;

use backend::*;
pub(crate) use backend::*;
pub(crate) use codeviz::python::*;
pub(crate) use core::*;
pub(crate) use errors::*;
use options::Options;
use self::field::*;
use self::listeners::*;
use self::python_backend::*;
use self::python_compiler::*;
use self::python_options::*;

pub(crate) const TYPE: &str = "type";
pub(crate) const INIT_PY: &str = "__init__.py";
pub(crate) const EXT: &str = "py";
pub(crate) const PYTHON_CONTEXT: &str = "python";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn resolve(options: Options, env: Environment) -> Result<PythonBackend> {
    let id_converter = options.id_converter;

    let package_prefix = options.package_prefix
        .clone()
        .map(|prefix| RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    let mut listeners = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = PythonOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    return Ok(PythonBackend::new(options,
                                 env,
                                 id_converter,
                                 package_prefix,
                                 Box::new(listeners)));
}
