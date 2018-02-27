#[macro_use]
extern crate genco;
#[macro_use]
extern crate log;
#[macro_use]
extern crate reproto_backend as backend;
extern crate reproto_core as core;
#[macro_use]
extern crate reproto_manifest as manifest;
extern crate reproto_trans as trans;
extern crate serde;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod listeners;
mod rust_backend;
mod rust_compiler;
mod rust_file_spec;
mod rust_options;
mod module;

use core::Context;
use core::errors::*;
use listeners::Listeners;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use rust_backend::RustBackend;
use rust_options::RustOptions;
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use trans::Environment;

const MOD: &str = "mod";
const EXT: &str = "rs";
const RUST_CONTEXT: &str = "rust";

#[derive(Clone, Copy, Default, Debug)]
pub struct RustLang;

impl Lang for RustLang {
    lang_base!(RustModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("as", "_as"),
            ("break", "_break"),
            ("const", "_const"),
            ("continue", "_continue"),
            ("crate", "_crate"),
            ("else", "_else"),
            ("enum", "_enum"),
            ("extern", "_extern"),
            ("false", "_false"),
            ("fn", "_fn"),
            ("for", "_for"),
            ("if", "_if"),
            ("impl", "_impl"),
            ("in", "_in"),
            ("let", "_let"),
            ("loop", "_loop"),
            ("match", "_match"),
            ("mod", "_mod"),
            ("move", "_move"),
            ("mut", "_mut"),
            ("pub", "_pub"),
            ("ref", "_ref"),
            ("return", "_return"),
            ("self", "_self"),
            ("static", "_static"),
            ("struct", "_struct"),
            ("super", "_super"),
            ("trait", "_trait"),
            ("true", "_true"),
            ("type", "_type"),
            ("unsafe", "_unsafe"),
            ("use", "_use"),
            ("where", "_where"),
            ("while", "_while"),
            ("abstract", "_abstract"),
            ("alignof", "_alignof"),
            ("become", "_become"),
            ("box", "_box"),
            ("do", "_do"),
            ("final", "_final"),
            ("macro", "_macro"),
            ("offsetof", "_offsetof"),
            ("override", "_override"),
            ("priv", "_priv"),
            ("proc", "_proc"),
            ("pure", "_pure"),
            ("sizeof", "_sizeof"),
            ("typeof", "_typeof"),
            ("unsized", "_unsized"),
            ("virtual", "_virtual"),
            ("yield", "_yield"),
        ]
    }
}

#[derive(Debug)]
pub enum RustModule {
    Chrono,
    Grpc,
}

impl TryFromToml for RustModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::RustModule::*;

        let result = match id {
            "chrono" => Chrono,
            "grpc" => Grpc,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::RustModule::*;

        let result = match id {
            "chrono" => Chrono,
            "grpc" => Grpc,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub fn setup_listeners(modules: Vec<RustModule>) -> Result<(RustOptions, Box<Listeners>)> {
    use self::RustModule::*;

    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for m in modules {
        debug!("+module: {:?}", m);

        let listener = match m {
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

fn compile(ctx: Rc<Context>, env: Environment, manifest: Manifest) -> Result<()> {
    let modules = manifest::checked_modules(manifest.modules)?;
    let (options, listeners) = setup_listeners(modules)?;
    let backend = RustBackend::new(env, options, listeners);
    let handle = ctx.filesystem(manifest.output.as_ref().map(AsRef::as_ref))?;
    let compiler = backend.compiler(handle.as_ref())?;
    compiler.compile()
}
