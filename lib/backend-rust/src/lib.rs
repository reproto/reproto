#[allow(unused)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate genco;
#[macro_use]
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate serde;
extern crate toml;

mod listeners;
mod rust_backend;
mod rust_compiler;
mod rust_file_spec;
mod rust_options;
mod module;

use self::backend::{ArgMatches, CompilerOptions, Environment, Options};
use self::backend::errors::*;
use self::listeners::Listeners;
use self::rust_backend::RustBackend;
use self::rust_options::RustOptions;
use core::Context;
use manifest::{Lang, Manifest, NoModule, TryFromToml, self as m};
use std::path::Path;
use std::rc::Rc;

const MOD: &str = "mod";
const EXT: &str = "rs";
const RUST_CONTEXT: &str = "rust";

#[derive(Default)]
pub struct RustLang;

impl Lang for RustLang {
    type Module = RustModule;
}

#[derive(Debug)]
pub enum RustModule {
    Chrono,
    Grpc,
}

impl TryFromToml for RustModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> m::errors::Result<Self> {
        use self::RustModule::*;

        let result = match id {
            "chrono" => Chrono,
            "grpc" => Grpc,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> m::errors::Result<Self> {
        use self::RustModule::*;

        let result = match id {
            "chrono" => Chrono,
            "grpc" => Grpc,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub fn setup_listeners(modules: &[RustModule]) -> Result<(RustOptions, Box<Listeners>)> {
    use self::RustModule::*;

    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        debug!("+module: {:?}", module);

        let listener = match *module {
            Chrono => Box::new(module::Chrono::new()) as Box<Listeners>,
            Grpc => Box::new(module::Grpc::new()) as Box<Listeners>,
        };

        listeners.push(listener);
    }

    let mut options = RustOptions::new();

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
    manifest: Manifest<RustLang>,
) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = RustBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}
