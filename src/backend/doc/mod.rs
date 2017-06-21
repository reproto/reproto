#[macro_use]
mod macros;
mod doc_backend;
mod doc_compiler;
mod doc_collector;
mod doc_options;
mod doc_listeners;

pub use backend::*;
pub use core::*;
pub use errors::*;
pub use options::Options;
pub use self::doc_backend::*;
pub use self::doc_collector::*;
pub use self::doc_compiler::*;
pub use self::doc_listeners::*;
pub use self::doc_options::*;
pub use self::macros::*;

pub(crate) const NORMALIZE_CSS_NAME: &str = "normalize.css";
pub(crate) const DOC_CSS_NAME: &str = "doc.css";
pub(crate) const EXT: &str = "html";
pub(crate) const INDEX: &str = "index";

fn setup_module(module: &str) -> Result<Box<DocListeners>> {
    let _module: Box<DocListeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn resolve(options: Options, env: Environment) -> Result<DocBackend> {
    let package_prefix = options.package_prefix
        .clone()
        .map(|prefix| RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    let mut listeners = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = DocOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    // TODO: make theme configurable.
    let theme = "light".to_owned();

    return Ok(DocBackend::new(options, env, package_prefix, theme, Box::new(listeners)));
}
