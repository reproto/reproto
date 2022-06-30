mod compiler;
mod flavored;
mod module;
mod utils;

use crate::compiler::Compiler;
use crate::flavored::*;
use genco::prelude::*;
use genco::tokens::ItemStr;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::{CoreFlavor, Handle};
use std::any::Any;
use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;
use trans::{Packages, Session};

const MOD: &str = "mod";
const EXT: &str = "rs";
const TYPE_SEP: &'static str = "_";
const SCOPE_SEP: &'static str = "::";

#[derive(Clone, Copy, Default, Debug)]
pub struct RustLang;

impl Lang for RustLang {
    manifest::lang_base!(Module, compile);

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
            ("try", "_try"),
            ("await", "_await"),
        ]
    }
}

#[derive(Debug)]
pub(crate) enum Module {
    Chrono,
    Reqwest,
}

impl TryFromToml for Module {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        let result = match id {
            "chrono" => Module::Chrono,
            "reqwest" => Module::Reqwest,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let result = match id {
            "chrono" => Module::Chrono,
            "reqwest" => Module::Reqwest,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub(crate) struct Options {
    pub(crate) datetime: Option<Type>,
    pub(crate) root: Vec<Box<dyn RootCodegen>>,
    pub(crate) service: Vec<Box<dyn ServiceCodegen>>,
    pub(crate) packages: Rc<Packages>,
}

pub(crate) struct Root<'a> {
    files: &'a mut BTreeMap<RpPackage, rust::Tokens>,
}

pub(crate) trait RootCodegen {
    /// Generate root code.
    fn generate(&self, root: Root) -> Result<()>;
}

pub(crate) struct Service<'a> {
    body: &'a flavored::RpServiceBody,
    container: &'a mut Tokens<Rust>,
    name: ItemStr,
    attributes: &'a Tokens<Rust>,
}

pub(crate) trait ServiceCodegen {
    /// Generate service code.
    fn generate(&self, service: Service<'_>) -> Result<()>;
}

fn options(modules: Vec<Module>, packages: Rc<Packages>) -> Result<Options> {
    let mut options = Options {
        datetime: None,
        root: Vec::new(),
        service: Vec::new(),
        packages,
    };

    for m in modules {
        log::debug!("+module: {:?}", m);

        match m {
            Module::Chrono => module::chrono::initialize(&mut options)?,
            Module::Reqwest => module::reqwest::initialize(&mut options)?,
        }
    }

    Ok(options)
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let modules = manifest::checked_modules(manifest.modules)?;
    let packages = session.packages()?;
    let options = options(modules, packages.clone())?;

    let session = session.translate(flavored::RustFlavorTranslator::new(
        packages.clone(),
        options.datetime.clone(),
    ))?;

    Compiler::new(&session, options, handle).compile()
}
