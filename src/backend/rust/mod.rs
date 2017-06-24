pub mod listeners;
pub mod rust_backend;
pub mod rust_compiler;
pub mod rust_options;

use backend::*;
pub(crate) use backend::*;
use clap::{App, ArgMatches};
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

pub fn setup_listeners(modules: Vec<String>) -> Result<(RustOptions, Box<Listeners>)> {
    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        listeners.push(setup_module(module.as_str())?);
    }

    let mut options = RustOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Compile for Rust")
}

pub fn verify_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Verify for Rust")
}

pub fn compile(env: Environment,
               opts: Options,
               compiler_options: CompilerOptions,
               _matches: &ArgMatches)
               -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = RustBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}

pub fn verify(env: Environment, opts: Options, _matches: &ArgMatches) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = RustBackend::new(env, options, listeners, id_converter);
    backend.verify()
}
