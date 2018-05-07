extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;

use core::{RelativePath, Reporter, Resolver, RpPackage, RpVersionedPackage, Source};
use manifest::Lang;
use std::any::Any;
use std::str;

/// Input to the compiler.
pub enum Input<'input> {
    /// Already derive file.
    File(ast::File<'input>, Option<RpVersionedPackage>),
    /// Source that should be parsed.
    Source(Source, Option<RpVersionedPackage>),
}

/// A simple compilation stage.
pub struct SimpleCompile<'a, 'input> {
    pub input: Input<'input>,
    pub reporter: &'a mut Reporter,
    pub package_prefix: Option<RpPackage>,
    pub resolver: Option<&'a mut Resolver>,
}

impl<'a, 'input> SimpleCompile<'a, 'input> {
    /// Build a new compilation stage.
    pub fn new(input: Input<'input>, reporter: &'a mut Reporter) -> SimpleCompile<'a, 'input> {
        Self {
            input: input,
            reporter: reporter,
            package_prefix: None,
            resolver: None,
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
    pub fn resolver(self, resolver: &'a mut Resolver) -> Self {
        Self {
            resolver: Some(resolver),
            ..self
        }
    }
}

/// Perform a simplified compilation that outputs the result into the provided Write
/// implementation.
pub fn simple_compile<'a, 'input, O>(
    mut out: O,
    config: SimpleCompile<'a, 'input>,
    modules: Vec<Box<Any>>,
    lang: &Lang,
) -> core::errors::Result<()>
where
    O: FnMut(&RelativePath, &str) -> core::errors::Result<()>,
{
    let SimpleCompile {
        input,
        package_prefix,
        reporter,
        resolver,
    } = config;

    let mut empty_resolver = core::EmptyResolver;
    let resolver = resolver.unwrap_or_else(|| &mut empty_resolver);

    let capturing = core::CapturingFilesystem::new();
    let fs = capturing.filesystem();

    let mut manifest = manifest::Manifest::default();
    manifest.lang = Some(lang.copy());
    manifest.modules = Some(modules);
    manifest.package_prefix = package_prefix.clone();

    let handle = fs.open_root(manifest.output.as_ref().map(AsRef::as_ref))?;

    let mut env = lang.into_env(package_prefix, reporter, resolver);

    match input {
        Input::File(file, package) => {
            env.import_file(file, package)?;
        }
        Input::Source(source, package) => {
            env.import_source(source, package)?;
        }
    }

    lang.compile(handle.as_ref(), env, manifest)?;

    let borrowed = capturing.files().try_borrow()?;

    let mut it = borrowed.iter().peekable();

    while let Some((path, content)) = it.next() {
        let content = str::from_utf8(content)?;
        out(path, content)?;
    }

    Ok(())
}
