#[macro_use]
mod macros;
mod doc_backend;
mod doc_builder;
mod doc_collector;
mod doc_compiler;
mod doc_listeners;
mod doc_options;
mod doc_writer;
mod escape;

pub use backend::*;
use clap::{App, ArgMatches};
pub use core::*;
pub use errors::*;
pub use options::Options;
pub use self::doc_backend::*;
pub use self::doc_builder::*;
pub use self::doc_collector::*;
pub use self::doc_compiler::*;
pub use self::doc_listeners::*;
pub use self::doc_options::*;
pub use self::doc_writer::*;
pub use self::escape::*;
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

pub fn setup_listeners(options: Options) -> Result<(DocOptions, Box<DocListeners>)> {
    let mut listeners: Vec<Box<DocListeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = DocOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Compile Documentation")
}

pub fn verify_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Verify for Documentation")
}

pub fn compile(env: Environment,
               options: Options,
               compiler_options: CompilerOptions,
               _matches: &ArgMatches)
               -> Result<()> {
    let theme = String::from("light");
    let (options, listeners) = setup_listeners(options)?;
    let backend = DocBackend::new(env, options, listeners, theme);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}

pub fn verify(env: Environment, options: Options, _matches: &ArgMatches) -> Result<()> {
    let theme = String::from("light");
    let (options, listeners) = setup_listeners(options)?;
    let backend = DocBackend::new(env, options, listeners, theme);
    backend.verify()
}
