mod compiler;
mod flavored;
mod utils;

use crate::compiler::Compiler;
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use reproto_core::errors::Result;
use reproto_core::{CoreFlavor, Handle};
use std::any::Any;
use std::path::Path;
use trans::Session;

const EXT: &str = "dart";
const TYPE_SEP: &'static str = "_";

#[derive(Clone, Copy, Default, Debug)]
pub struct DartLang;

impl Lang for DartLang {
    manifest::lang_base!(DartModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn field_ident_naming(&self) -> Option<Box<dyn naming::Naming>> {
        Some(Box::new(naming::to_lower_camel()))
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("abstract", "abstract_"),
            ("dynamic", "dynamic_"),
            ("implements", "implements_"),
            ("show", "show_"),
            ("as", "as_"),
            ("else", "else_"),
            ("import", "import_"),
            ("static", "static_"),
            ("assert", "assert_"),
            ("enum", "enum_"),
            ("in", "in_"),
            ("super", "super_"),
            ("async", "async_"),
            ("export", "export_"),
            ("interface", "interface_"),
            ("switch", "switch_"),
            ("await", "await_"),
            ("external", "external_"),
            ("is", "is_"),
            ("sync", "sync_"),
            ("break", "break_"),
            ("extends", "extends_"),
            ("library", "library_"),
            ("this", "this_"),
            ("case", "case_"),
            ("factory", "factory_"),
            ("mixin", "mixin_"),
            ("throw", "throw_"),
            ("catch", "catch_"),
            ("false", "false_"),
            ("new", "new_"),
            ("true", "true_"),
            ("class", "class_"),
            ("final", "final_"),
            ("null", "null_"),
            ("try", "try_"),
            ("const", "const_"),
            ("finally", "finally_"),
            ("on", "on_"),
            ("typedef", "typedef_"),
            ("continue", "continue_"),
            ("for", "for_"),
            ("operator", "operator_"),
            ("var", "var_"),
            ("covariant", "covariant_"),
            ("Function", "Function_"),
            ("part", "part_"),
            ("void", "void_"),
            ("default", "default_"),
            ("get", "get_"),
            ("rethrow", "rethrow_"),
            ("while", "while_"),
            ("deferred", "deferred_"),
            ("hide", "hide_"),
            ("return", "return_"),
            ("with", "with_"),
            ("do", "do_"),
            ("if", "if_"),
            ("set", "set_"),
            ("yield", "yield_"),
        ]
    }
}

#[derive(Debug)]
pub enum DartModule {}

impl TryFromToml for DartModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        return NoModule::illegal(path, id, value);
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        return NoModule::illegal(path, id, value);
    }
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let _: Vec<DartModule> = manifest::checked_modules(manifest.modules)?;
    let packages = session.packages()?;
    let session = session.translate(flavored::DartFlavorTranslator::new(packages.clone()))?;

    Compiler::new(&session, handle).compile()
}
