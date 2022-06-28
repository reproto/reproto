mod codegen;
mod compiler;
mod flavored;
mod module;

use crate::compiler::Compiler;
use crate::flavored::Type;
use backend::Initializer;
use genco::prelude::*;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::{CoreFlavor, Handle};
use std::any::Any;
use std::path::Path;
use trans::Session;

const EXT: &str = "swift";
const TYPE_SEP: &'static str = "_";

#[derive(Clone, Copy, Default, Debug)]
pub struct SwiftLang;

impl Lang for SwiftLang {
    manifest::lang_base!(SwiftModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn package_naming(&self) -> Option<Box<dyn naming::Naming>> {
        Some(Box::new(naming::to_upper_camel()))
    }

    fn safe_packages(&self) -> bool {
        true
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("as", "as_"),
            ("associatedtype", "associatedtype_"),
            ("associativity", "associativity_"),
            ("break", "break_"),
            ("case", "case_"),
            ("catch", "catch_"),
            ("class", "class_"),
            ("continue", "continue_"),
            ("convenience", "convenience_"),
            ("default", "default_"),
            ("defer", "defer_"),
            ("deinit", "deinit_"),
            ("do", "do_"),
            ("dynamic", "dynamic_"),
            ("else", "else_"),
            ("enum", "enum_"),
            ("extension", "extension_"),
            ("fallthrough", "fallthrough_"),
            ("false", "false_"),
            ("fileprivate", "fileprivate_"),
            ("final", "final_"),
            ("for", "for_"),
            ("func", "func_"),
            ("get", "get_"),
            ("guard", "guard_"),
            ("if", "if_"),
            ("import", "import_"),
            ("in", "in_"),
            ("indirect", "indirect_"),
            ("infix", "infix_"),
            ("init", "init_"),
            ("inout", "inout_"),
            ("internal", "internal_"),
            ("is", "is_"),
            ("lazy", "lazy_"),
            ("left", "left_"),
            ("let", "let_"),
            ("mutating", "mutating_"),
            ("nil", "nil_"),
            ("none", "none_"),
            ("nonmutating", "nonmutating_"),
            ("open", "open_"),
            ("operator", "operator_"),
            ("optional", "optional_"),
            ("override", "override_"),
            ("postfix", "postfix_"),
            ("precedence", "precedence_"),
            ("prefix", "prefix_"),
            ("private", "private_"),
            ("protocol", "protocol_"),
            ("public", "public_"),
            ("repeat", "repeat_"),
            ("required", "required_"),
            ("rethrows", "rethrows_"),
            ("return", "return_"),
            ("right", "right_"),
            ("self", "self_"),
            ("set", "set_"),
            ("static", "static_"),
            ("struct", "struct_"),
            ("subscript", "subscript_"),
            ("super", "super_"),
            ("switch", "switch_"),
            ("throw", "throw_"),
            ("throws", "throws_"),
            ("true", "true_"),
            ("try", "try_"),
            ("typealias", "typealias_"),
            ("unowned", "unowned_"),
            ("var", "var_"),
            ("weak", "weak_"),
            ("where", "where_"),
            ("while", "while_"),
        ]
    }
}

#[derive(Debug)]
pub(crate) enum SwiftModule {
    Grpc,
    Simple,
    Codable,
}

impl TryFromToml for SwiftModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::SwiftModule::*;

        let result = match id {
            "grpc" => Grpc,
            "simple" => Simple,
            "codable" => Codable,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::SwiftModule::*;

        let result = match id {
            "grpc" => Grpc,
            "simple" => Simple,
            "codable" => Codable,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub(crate) struct Options {
    /// All types that the struct model should extend.
    pub(crate) struct_model_extends: Vec<swift::Tokens>,
    /// The provided Any type that should be used in structs.
    pub(crate) any_type: Vec<(&'static str, Type)>,
    pub(crate) gen: codegen::Generators,
}

impl Options {
    pub(crate) fn new() -> Options {
        Options {
            struct_model_extends: Vec::new(),
            any_type: Vec::new(),
            gen: codegen::Generators::default(),
        }
    }
}

pub(crate) fn options(modules: Vec<SwiftModule>) -> Result<Options> {
    use self::SwiftModule::*;

    let mut options = Options::new();

    for m in modules {
        log::debug!("+module: {:?}", m);

        let initializer: Box<dyn Initializer<Options = Options>> = match m {
            Grpc => Box::new(module::Grpc::new()),
            Simple => Box::new(module::Simple::new()),
            Codable => Box::new(module::Codable::new()),
        };

        initializer.initialize(&mut options)?;
    }

    Ok(options)
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let modules = manifest::checked_modules(manifest.modules)?;
    let options = options(modules)?;

    let packages = session.packages()?;

    let session = session.translate(flavored::SwiftFlavorTranslator::new(
        packages.clone(),
        &options,
    )?)?;

    Compiler::new(&session, options, handle)?.compile(&packages)
}
