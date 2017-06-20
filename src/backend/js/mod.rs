mod models;
#[macro_use]
mod utils;
pub mod listeners;
pub mod js_backend;
pub mod js_compiler;
pub mod js_options;

use backend::*;
pub(crate) use codeviz::js::*;
pub(crate) use errors::*;
use options::Options;
use self::js_backend::*;
use self::js_compiler::*;
use self::js_options::*;
use self::listeners::*;
pub(crate) use self::models::*;
pub(crate) use self::utils::*;

pub(crate) const TYPE: &str = "type";
pub(crate) const EXT: &str = "js";
pub(crate) const JS_CONTEXT: &str = "js";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn resolve(options: Options, env: Environment) -> Result<JsBackend> {
    let id_converter = options.id_converter;

    let package_prefix = options.package_prefix
        .clone()
        .map(|prefix| RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    let mut listeners = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = JsOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    return Ok(JsBackend::new(options,
                             env,
                             id_converter,
                             package_prefix,
                             Box::new(listeners)));
}
