mod codegen;
mod compiler;
mod flavored;
mod module;
mod options;
mod processor;

use crate::compiler::Compiler;
use crate::options::Options;
use manifest::{checked_modules, Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::{CoreFlavor, Handle};
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use trans::Session;

#[derive(Clone, Copy, Default, Debug)]
pub struct CsharpLang;

impl Lang for CsharpLang {
    manifest::lang_base!(CsharpModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn safe_packages(&self) -> bool {
        true
    }

    fn package_naming(&self) -> Option<Box<dyn naming::Naming>> {
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
        let result = match id {
            "Json.NET" => CsharpModule::JsonNet,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let result = match id {
            "Json.NET" => CsharpModule::JsonNet,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

fn setup_options<'a>(modules: Vec<CsharpModule>) -> Options {
    let mut options = Options::new();

    for module in modules {
        match module {
            CsharpModule::JsonNet => {
                module::json_net::initialize(&mut options);
            }
        };
    }

    options
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let packages = session.packages()?;

    let session = session.translate(flavored::CsharpFlavorTranslator::new(packages))?;
    let session = Rc::new(session);

    let modules = checked_modules(manifest.modules)?;
    let options = setup_options(modules);
    let compiler = Compiler::new(session.clone(), options);

    compiler.compile(handle)
}
