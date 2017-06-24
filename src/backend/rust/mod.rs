pub mod listeners;
pub mod rust_backend;
pub mod rust_compiler;
pub mod rust_options;

use backend::*;
pub(crate) use backend::*;
pub(crate) use codeviz::rust::*;
pub(crate) use core::*;
pub(crate) use errors::*;
use options::Options;
use self::listeners::*;
use self::rust_backend::*;
use self::rust_compiler::*;
use self::rust_options::*;

pub(crate) const MOD: &str = "mod";
pub(crate) const EXT: &str = "rs";
pub(crate) const RUST_CONTEXT: &str = "rust";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn resolve(options: Options, env: Environment) -> Result<RustBackend> {
    let id_converter = options.id_converter;

    let mut listeners = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = RustOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    return Ok(RustBackend::new(options, env, id_converter, Box::new(listeners)));
}
