extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;

use core::{ContextItem, Object, RelativePath, Resolver, RpPackage, RpVersionedPackage};
use manifest::Lang;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::str;

/// Input to the compiler.
pub enum Input<'input> {
    /// Already derive file.
    File(ast::File<'input>, Option<RpVersionedPackage>),
    /// Object that should be parsed.
    Object(Box<Object>, Option<RpVersionedPackage>),
}

/// A simple compilation stage.
pub struct SimpleCompile<'input> {
    pub input: Input<'input>,
    pub package_prefix: Option<RpPackage>,
    pub resolver: Option<Box<Resolver>>,
    pub items: Option<Rc<RefCell<Vec<ContextItem>>>>,
}

impl<'input> SimpleCompile<'input> {
    /// Build a new compilation stage.
    pub fn new(input: Input<'input>) -> SimpleCompile {
        Self {
            input: input,
            package_prefix: None,
            resolver: None,
            items: None,
        }
    }

    /// Set package prefix.
    pub fn package_prefix(self, package: RpPackage) -> Self {
        Self {
            package_prefix: Some(package),
            ..self
        }
    }

    /// Set resolver.
    pub fn resolver(self, resolver: Box<Resolver>) -> Self {
        Self {
            resolver: Some(resolver),
            ..self
        }
    }

    /// Set a reference to collect items.
    pub fn with_items(self, items: Rc<RefCell<Vec<ContextItem>>>) -> Self {
        Self {
            items: Some(items),
            ..self
        }
    }
}

/// Perform a simplified compilation that outputs the result into the provided Write
/// implementation.
pub fn simple_compile<O>(
    mut out: O,
    config: SimpleCompile,
    modules: Vec<Box<Any>>,
    lang: &Lang,
) -> core::errors::Result<()>
where
    O: FnMut(&RelativePath, &str) -> core::errors::Result<()>,
{
    let SimpleCompile {
        input,
        package_prefix,
        resolver,
        items,
    } = config;

    let resolver = resolver.unwrap_or_else(|| Box::new(core::EmptyResolver));

    let capturing = core::CapturingFilesystem::new();

    let ctx = core::Context::new(capturing.filesystem());

    // Set items reference, if configured.
    let ctx = if let Some(items) = items {
        ctx.with_items(items)
    } else {
        ctx
    };

    let ctx = Rc::new(ctx);

    let mut env = lang.into_env(ctx.clone(), package_prefix.clone(), resolver);

    match input {
        Input::File(file, package) => {
            env.import_file(file, package)?;
        }
        Input::Object(object, package) => {
            env.import_object(object.as_ref(), package)?;
        }
    }

    let preamble = manifest::ManifestPreamble::new(Some(manifest::Language::Java), None);
    let mut manifest = manifest::read_manifest(lang, preamble)?;
    manifest.modules = modules;
    manifest.package_prefix = package_prefix;

    lang.compile(ctx, env, manifest)?;

    let borrowed = capturing.files().try_borrow()?;

    let mut it = borrowed.iter().peekable();

    while let Some((path, content)) = it.next() {
        let content = str::from_utf8(content)?;
        out(path, content)?;
    }

    Ok(())
}
