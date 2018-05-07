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
extern crate reproto_naming as naming;
extern crate reproto_trans as trans;
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod codegen;
mod compiler;
mod csharp_field;
mod csharp_file;
mod flavored;
mod module;
mod options;
mod processor;
mod utils;

use codegen::Configure;
use compiler::Compiler;
use core::errors::Result;
use core::{CoreFlavor, Handle};
use manifest::{checked_modules, Lang, Manifest, NoModule, TryFromToml};
use options::Options;
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use trans::Environment;

#[derive(Clone, Copy, Default, Debug)]
pub struct CsharpLang;

impl Lang for CsharpLang {
    lang_base!(CsharpModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn safe_packages(&self) -> bool {
        true
    }

    fn package_naming(&self) -> Option<Box<naming::Naming>> {
        Some(Box::new(naming::to_upper_camel()))
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("abstract", "_abstract"),
            ("as", "_as"),
            ("base", "_base"),
            ("bool", "_bool"),
            ("break", "_break"),
            ("byte", "_byte"),
            ("case", "_case"),
            ("catch", "_catch"),
            ("char", "_char"),
            ("checked", "_checked"),
            ("class", "_class"),
            ("const", "_const"),
            ("continue", "_continue"),
            ("decimal", "_decimal"),
            ("default", "_default"),
            ("delegate", "_delegate"),
            ("do", "_do"),
            ("double", "_double"),
            ("else", "_else"),
            ("enum", "_enum"),
            ("event", "_event"),
            ("explicit", "_explicit"),
            ("extern", "_extern"),
            ("false", "_false"),
            ("finally", "_finally"),
            ("fixed", "_fixed"),
            ("float", "_float"),
            ("for", "_for"),
            ("foreach", "_foreach"),
            ("goto", "_goto"),
            ("if", "_if"),
            ("implicit", "_implicit"),
            ("in", "_in"),
            ("int", "_int"),
            ("interface", "_interface"),
            ("internal", "_internal"),
            ("is", "_is"),
            ("lock", "_lock"),
            ("long", "_long"),
            ("namespace", "_namespace"),
            ("new", "_new"),
            ("null", "_null"),
            ("object", "_object"),
            ("operator", "_operator"),
            ("out", "_out"),
            ("override", "_override"),
            ("params", "_params"),
            ("private", "_private"),
            ("protected", "_protected"),
            ("public", "_public"),
            ("readonly", "_readonly"),
            ("ref", "_ref"),
            ("return", "_return"),
            ("sbyte", "_sbyte"),
            ("sealed", "_sealed"),
            ("short", "_short"),
            ("sizeof", "_sizeof"),
            ("stackalloc", "_stackalloc"),
            ("static", "_static"),
            ("string", "_string"),
            ("struct", "_struct"),
            ("switch", "_switch"),
            ("this", "_this"),
            ("throw", "_throw"),
            ("true", "_true"),
            ("try", "_try"),
            ("typeof", "_typeof"),
            ("uint", "_uint"),
            ("ulong", "_ulong"),
            ("unchecked", "_unchecked"),
            ("unsafe", "_unsafe"),
            ("ushort", "_ushort"),
            ("using", "_using"),
            ("virtual", "_virtual"),
            ("void", "_void"),
            ("volatile", "_volatile"),
            ("while", "_while"),
        ]
    }
}

#[derive(Debug)]
pub enum CsharpModule {
    JsonNet,
}

impl TryFromToml for CsharpModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::CsharpModule::*;

        let result = match id {
            "Json.NET" => JsonNet,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::CsharpModule::*;

        let result = match id {
            "Json.NET" => JsonNet,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

fn setup_options<'a>(modules: Vec<CsharpModule>) -> Options {
    use self::CsharpModule::*;

    let mut options = Options::new();

    for module in modules {
        let c = Configure {
            options: &mut options,
        };

        match module {
            JsonNet => module::JsonNet.initialize(c),
        };
    }

    options
}

fn compile(handle: &Handle, env: Environment<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let packages = env.packages()?;

    let translator = env.translator(flavored::CsharpFlavorTranslator::new(packages))?;

    let env = env.translate(translator)?;
    let env = Rc::new(env);

    let modules = checked_modules(manifest.modules)?;
    let options = setup_options(modules);
    let compiler = Compiler::new(env.clone(), options);

    compiler.compile(handle)
}
