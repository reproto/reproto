mod compiler;
mod flavored;
mod utils;

use crate::compiler::Compiler;
use genco::prelude::*;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::{CoreFlavor, Handle};
use std::any::Any;
use std::path::Path;
use trans::Session;

const TYPE_SEP: &str = "_";
const EXT: &str = "js";

#[derive(Clone, Copy, Default, Debug)]
pub struct JsLang;

impl Lang for JsLang {
    manifest::lang_base!(JsModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("# {}", input))
    }

    fn safe_packages(&self) -> bool {
        // NB: JavaScript imports by string literals, no keyword escaping needed.
        true
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("abstract", "_abstract"),
            ("await", "_await"),
            ("boolean", "_boolean"),
            ("break", "_break"),
            ("byte", "_byte"),
            ("case", "_case"),
            ("catch", "_catch"),
            ("char", "_char"),
            ("class", "_class"),
            ("const", "_const"),
            ("continue", "_continue"),
            ("debugger", "_debugger"),
            ("default", "_default"),
            ("delete", "_delete"),
            ("do", "_do"),
            ("double", "_double"),
            ("else", "_else"),
            ("enum", "_enum"),
            ("export", "_export"),
            ("extends", "_extends"),
            ("false", "_false"),
            ("final", "_final"),
            ("finally", "_finally"),
            ("float", "_float"),
            ("for", "_for"),
            ("function", "_function"),
            ("goto", "_goto"),
            ("if", "_if"),
            ("implements", "_implements"),
            ("import", "_import"),
            ("in", "_in"),
            ("instanceof", "_instanceof"),
            ("int", "_int"),
            ("interface", "_interface"),
            ("let", "_let"),
            ("long", "_long"),
            ("native", "_native"),
            ("new", "_new"),
            ("null", "_null"),
            ("package", "_package"),
            ("private", "_private"),
            ("protected", "_protected"),
            ("public", "_public"),
            ("return", "_return"),
            ("short", "_short"),
            ("static", "_static"),
            ("super", "_super"),
            ("switch", "_switch"),
            ("synchronized", "_synchronized"),
            ("this", "_this"),
            ("throw", "_throw"),
            ("throws", "_throws"),
            ("transient", "_transient"),
            ("true", "_true"),
            ("try", "_try"),
            ("typeof", "_typeof"),
            ("var", "_var"),
            ("void", "_void"),
            ("volatile", "_volatile"),
            ("while", "_while"),
            ("with", "_with"),
            ("yield", "_yield"),
        ]
    }
}

#[derive(Debug)]
pub enum JsModule {}

impl TryFromToml for JsModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        NoModule::illegal(path, id, value)
    }
}

pub struct Options {
    pub build_getters: bool,
    pub build_constructor: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            build_getters: false,
            build_constructor: true,
        }
    }
}

pub struct FileSpec(pub Tokens<JavaScript>);

impl Default for FileSpec {
    fn default() -> Self {
        FileSpec(Tokens::new())
    }
}

fn compile(handle: &dyn Handle, env: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let packages = env.packages()?;

    let env = env.translate(flavored::JavaScriptFlavorTranslator::new(packages))?;

    let _modules: Vec<JsModule> = manifest::checked_modules(manifest.modules)?;
    let options = Options::new();

    Compiler::new(&env, options, handle).compile()
}
