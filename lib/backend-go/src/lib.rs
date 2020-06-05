mod compiler;
mod flavored;
mod module;

use crate::compiler::Compiler;
use crate::flavored::{GoName, RpEnumBody, RpField, RpInterfaceBody, RpTupleBody};
use backend::Initializer;
use core::errors::Result;
use core::{CoreFlavor, Handle};
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr, Tokens};
use manifest::{Lang, Manifest, NoModule, TryFromToml};
use naming::Naming;
use std::any::Any;
use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;
use trans::Session;

const TYPE_SEP: &str = "_";
const EXT: &str = "go";

#[derive(Clone, Copy, Default, Debug)]
pub struct GoLang;

impl Lang for GoLang {
    manifest::lang_base!(GoModule, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn safe_packages(&self) -> bool {
        true
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![]
    }

    fn field_ident_naming(&self) -> Option<Box<dyn Naming>> {
        Some(Box::new(naming::to_upper_camel()))
    }
}

#[derive(Debug)]
pub enum GoModule {
    EncodingJson,
}

impl TryFromToml for GoModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        use self::GoModule::*;

        let result = match id {
            "encoding/json" => EncodingJson,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        use self::GoModule::*;

        let result = match id {
            "encoding/json" => EncodingJson,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

pub struct Options {
    pub field_gens: Vec<Box<dyn FieldCodegen>>,
    pub enum_gens: Vec<Box<dyn EnumCodegen>>,
    pub tuple_gens: Vec<Box<dyn TupleCodegen>>,
    pub interface_gens: Vec<Box<dyn InterfaceCodegen>>,
}

impl Options {
    pub fn new() -> Options {
        Options {
            field_gens: Vec::new(),
            enum_gens: Vec::new(),
            tuple_gens: Vec::new(),
            interface_gens: Vec::new(),
        }
    }
}

pub fn options(modules: Vec<GoModule>) -> Result<Options> {
    use self::GoModule::*;

    let mut options = Options::new();

    for m in modules {
        log::debug!("+module: {:?}", m);

        let initializer: Box<dyn Initializer<Options = Options>> = match m {
            EncodingJson => Box::new(module::EncodingJson::new()),
        };

        initializer.initialize(&mut options)?;
    }

    Ok(options)
}

pub struct FileSpec(pub Tokens<Go>);

impl Default for FileSpec {
    fn default() -> Self {
        FileSpec(Tokens::new())
    }
}

/// Build codegen hooks.
macro_rules! codegen {
    ($c:tt, $e:ty) => {
        pub trait $c {
            fn generate(&self, e: $e) -> Result<()>;
        }

        impl<T> $c for Rc<T>
        where
            T: $c,
        {
            fn generate(&self, e: $e) -> Result<()> {
                self.as_ref().generate(e)
            }
        }
    };
}

/// Event emitted when a field has been added.
pub struct FieldAdded<'a> {
    pub tags: &'a mut Tags,
    pub field: &'a RpField,
}

codegen!(FieldCodegen, FieldAdded);

/// Event emitted when an enum has been added
pub struct EnumAdded<'a> {
    pub container: &'a mut Tokens<Go>,
    pub name: &'a GoName,
    pub body: &'a RpEnumBody,
}

codegen!(EnumCodegen, EnumAdded);

/// Event emitted when a tuple has been added.
pub struct TupleAdded<'a> {
    pub container: &'a mut Tokens<Go>,
    pub name: &'a GoName,
    pub body: &'a RpTupleBody,
}

codegen!(TupleCodegen, TupleAdded);

/// Event emitted when an interface has been added.
pub struct InterfaceAdded<'a> {
    pub container: &'a mut Tokens<Go>,
    pub name: &'a GoName,
    pub body: &'a RpInterfaceBody,
}

codegen!(InterfaceCodegen, InterfaceAdded);

/// Structure for Tags - a type of Go metadata
pub struct Tags {
    values: BTreeMap<ItemStr, Vec<ItemStr>>,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Push a string tag.
    pub fn push_str<K: Into<ItemStr>, V: Into<ItemStr>>(&mut self, key: K, value: V) {
        self.values
            .entry(key.into())
            .or_insert_with(Vec::new)
            .push(value.into());
    }
}

impl FormatInto<Go> for Tags {
    fn format_into(self, t: &mut Tokens<Go>) {
        if self.values.is_empty() {
            return;
        }

        let mut it = self.values.into_iter().peekable();

        let mut s = String::new();

        s.push('`');

        while let Some((key, vals)) = it.next() {
            s.push_str(&key);
            s.push(':');

            let mut val_it = vals.into_iter().peekable();

            s.push('"');

            while let Some(v) = val_it.next() {
                s.push_str(&v);

                if val_it.peek().is_some() {
                    s.push(',');
                }
            }

            s.push('"');

            if it.peek().is_some() {
                t.append(" ")
            }
        }

        s.push('`');
        t.append(s);
    }
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let packages = session.packages()?;

    let session = session.translate(flavored::GoFlavorTranslator::new(packages))?;

    let modules = manifest::checked_modules(manifest.modules)?;
    let options = options(modules)?;
    Compiler::new(&session, options, handle)?.compile()
}
