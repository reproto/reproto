#[macro_use]
extern crate genco;
#[macro_use]
extern crate log;
#[macro_use]
extern crate reproto_backend as backend;
#[macro_use]
extern crate reproto_core as core;
#[macro_use]
extern crate reproto_manifest as manifest;
extern crate reproto_trans as trans;
extern crate serde;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod compiler;
mod flavored;
mod module;
mod rust_file_spec;
mod utils;

use backend::Initializer;
use compiler::Compiler;
use core::errors::*;
use core::{CoreFlavor, Handle};
use flavored::RpPackage;
use genco::{Cons, Rust, Tokens};
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use rust_file_spec::RustFileSpec;
use std::any::Any;
use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;
use trans::{Environment, Packages};

const LIB: &str = "lib";
const MOD: &str = "mod";
const EXT: &str = "rs";
const TYPE_SEP: &'static str = "_";
const SCOPE_SEP: &'static str = "::";

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
    Reqwest,
}

impl TryFromToml for RustModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::RustModule::*;

        let result = match id {
            "chrono" => Chrono,
            "grpc" => Grpc,
            "reqwest" => Reqwest,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::RustModule::*;

        let result = match id {
            "chrono" => Chrono,
            "grpc" => Grpc,
            "reqwest" => Reqwest,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub struct Options {
    pub datetime: Option<Rust<'static>>,
    pub root: Vec<Box<RootCodegen>>,
    pub service: Vec<Box<ServiceCodegen>>,
    pub packages: Rc<Packages>,
}

pub struct Root<'a, 'el: 'a> {
    files: &'a mut BTreeMap<RpPackage, RustFileSpec<'el>>,
}

pub trait RootCodegen {
    /// Generate root code.
    fn generate(&self, root: Root) -> Result<()>;
}

pub struct Service<'a, 'el: 'a> {
    body: &'el flavored::RpServiceBody,
    container: &'a mut Tokens<'el, Rust<'el>>,
    name: Cons<'el>,
    attributes: &'a Tokens<'el, Rust<'el>>,
}

pub trait ServiceCodegen {
    /// Generate service code.
    fn generate(&self, service: Service) -> Result<()>;
}

fn options(modules: Vec<RustModule>, packages: Rc<Packages>) -> Result<Options> {
    use self::RustModule::*;

    let mut options = Options {
        datetime: None,
        root: Vec::new(),
        service: Vec::new(),
        packages: packages,
    };

    for m in modules {
        debug!("+module: {:?}", m);

        let initializer: Box<Initializer<Options = Options>> = match m {
            Chrono => Box::new(module::Chrono::new()),
            Grpc => Box::new(module::Grpc::new()),
            Reqwest => Box::new(module::Reqwest::new()),
        };

        initializer.initialize(&mut options)?;
    }

    Ok(options)
}

fn compile(handle: &Handle, env: Environment<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let modules = manifest::checked_modules(manifest.modules)?;
    let packages = env.packages()?;
    let options = options(modules, packages.clone())?;

    let translator = env.translator(flavored::RustFlavorTranslator::new(
        packages.clone(),
        options.datetime.clone(),
    ))?;
    let env = env.translate(translator)?;

    Compiler::new(&env, options, handle).compile()
}
