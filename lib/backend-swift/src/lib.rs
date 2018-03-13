#[macro_use]
extern crate genco;
#[macro_use]
extern crate log;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
#[macro_use]
extern crate reproto_manifest as manifest;
extern crate reproto_naming as naming;
extern crate reproto_trans as trans;
extern crate serde;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod compiler;
mod module;
mod swift;

use backend::{Initializer, IntoBytes};
use compiler::Compiler;
use core::Context;
use core::errors::Result;
use genco::Tokens;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use naming::Naming;
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use swift::Swift;
use trans::Environment;

const EXT: &str = "swift";

#[derive(Clone, Copy, Default, Debug)]
pub struct SwiftLang;

impl Lang for SwiftLang {
    lang_base!(SwiftModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn package_naming(&self) -> Option<Box<Naming>> {
        Some(Box::new(naming::to_upper_camel()))
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
pub enum SwiftModule {
    Grpc,
}

impl TryFromToml for SwiftModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::SwiftModule::*;

        let result = match id {
            "grpc" => Grpc,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::SwiftModule::*;

        let result = match id {
            "grpc" => Grpc,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub struct Options {}

impl Options {
    pub fn new() -> Options {
        Options {}
    }
}

pub fn options(modules: Vec<SwiftModule>) -> Result<Options> {
    use self::SwiftModule::*;

    let mut options = Options::new();

    for m in modules {
        debug!("+module: {:?}", m);

        let initializer: Box<Initializer<Options = Options>> = match m {
            Grpc => Box::new(module::Grpc::new()),
        };

        initializer.initialize(&mut options)?;
    }

    Ok(options)
}

pub struct FileSpec<'a>(pub Tokens<'a, Swift<'a>>);

impl<'el> Default for FileSpec<'el> {
    fn default() -> Self {
        FileSpec(Tokens::new())
    }
}

impl<'el> IntoBytes<Compiler<'el>> for FileSpec<'el> {
    fn into_bytes(self, _: &Compiler<'el>) -> Result<Vec<u8>> {
        let out = self.0.join_line_spacing().to_file()?;
        Ok(out.into_bytes())
    }
}

fn compile(ctx: Rc<Context>, env: Environment, manifest: Manifest) -> Result<()> {
    let modules = manifest::checked_modules(manifest.modules)?;
    let options = options(modules)?;
    let handle = ctx.filesystem(manifest.output.as_ref().map(AsRef::as_ref))?;
    Compiler::new(&env, options, handle.as_ref()).compile()
}
