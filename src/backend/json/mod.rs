mod json_backend;
mod json_compiler;
mod json_options;
mod listeners;

use backend::*;
use options::Options;
pub(crate) use self::json_backend::*;
pub(crate) use self::json_compiler::*;
pub(crate) use self::json_options::*;
pub(crate) use self::listeners::*;

pub(crate) const EXT: &str = "json";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn resolve(options: Options, env: Environment) -> Result<JsonBackend> {
    let mut listeners = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = JsonOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    return Ok(JsonBackend::new(options, env, Box::new(listeners)));
}
