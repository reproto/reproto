mod codegen;
mod compiler;
mod flavored;
pub mod module;
mod utils;

use crate::codegen::ServiceCodegen;
use crate::compiler::Compiler;
use crate::utils::VersionHelper;
use backend::Initializer;
use core::errors::Result;
use core::{CoreFlavor, Handle, Loc, RpField, Span};
use genco::prelude::*;
use genco::tokens::ItemStr;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use trans::Session;

const TYPE_SEP: &str = "_";
const INIT_PY: &str = "__init__.py";
const EXT: &str = "py";

#[derive(Clone, Copy, Default, Debug)]
pub struct PythonLang;

impl Lang for PythonLang {
    manifest::lang_base!(PythonModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("# {}", input))
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        // NB: combined set of keywords for Python 2/3 to avoid having two codegen implementations.
        vec![
            ("and", "_and"),
            ("as", "_as"),
            ("assert", "_assert"),
            ("break", "_break"),
            ("class", "_class"),
            ("continue", "_continue"),
            ("def", "_def"),
            ("del", "_del"),
            ("elif", "_elif"),
            ("else", "_else"),
            ("except", "_except"),
            ("exec", "_exec"),
            ("finally", "_finally"),
            ("for", "_for"),
            ("from", "_from"),
            ("global", "_global"),
            ("if", "_if"),
            ("import", "_import"),
            ("in", "_in"),
            ("is", "_is"),
            ("lambda", "_lambda"),
            ("nonlocal", "_nonlocal"),
            ("not", "_not"),
            ("or", "_or"),
            ("pass", "_pass"),
            ("print", "_print"),
            ("raise", "_raise"),
            ("return", "_return"),
            ("try", "_try"),
            ("while", "_while"),
            ("with", "_with"),
            ("yield", "_yield"),
            ("self", "_self"),
        ]
    }
}

#[derive(Debug)]
pub enum PythonModule {
    Requests(module::RequestsConfig),
    Python2(module::Python2Config),
}

impl TryFromToml for PythonModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::PythonModule::*;

        let result = match id {
            "requests" => Requests(module::RequestsConfig::default()),
            "python2" => Python2(module::Python2Config::default()),
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::PythonModule::*;

        let result = match id {
            "requests" => Requests(value.try_into()?),
            "python2" => Python2(value.try_into()?),
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub struct Options {
    pub build_getters: bool,
    pub build_constructor: bool,
    pub service_generators: Vec<Box<dyn ServiceCodegen>>,
    pub version_helper: Rc<dyn VersionHelper>,
}

#[derive(Debug, PartialEq, Eq)]
struct Python3VersionHelper {}

impl VersionHelper for Python3VersionHelper {
    fn is_string(&self, var: &ItemStr) -> Tokens<Python> {
        quote!(isinstance(#var, str))
    }
}

impl Options {
    pub fn new() -> Options {
        Options {
            build_getters: true,
            build_constructor: true,
            service_generators: Vec::new(),
            version_helper: Rc::new(Python3VersionHelper {}),
        }
    }
}

pub struct FileSpec(pub Tokens<Python>);

impl Default for FileSpec {
    fn default() -> Self {
        FileSpec(Tokens::new())
    }
}

pub fn setup_options(modules: Vec<PythonModule>) -> Result<Options> {
    use self::PythonModule::*;

    let mut options = Options::new();

    for module in modules {
        let initializer: Box<dyn Initializer<Options = Options>> = match module {
            Requests(config) => Box::new(module::Requests::new(config)),
            Python2(config) => Box::new(module::Python2::new(config)),
        };

        initializer.initialize(&mut options)?;
    }

    Ok(options)
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let modules = manifest::checked_modules(manifest.modules)?;
    let options = setup_options(modules)?;

    let packages = session.packages()?;

    let helper = options.version_helper.clone();
    let session = session.translate(flavored::PythonFlavorTranslator::new(
        packages,
        helper.clone(),
    ))?;

    let variant_field = Loc::new(
        RpField::new(
            "ordinal",
            flavored::Type::String {
                helper: helper.clone(),
            },
        )
        .with_safe_ident("_ordinal"),
        Span::empty(),
    );

    Compiler::new(&session, variant_field, options, handle).compile()
}
