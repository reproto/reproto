#![recursion_limit = "1000"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate genco;
extern crate reproto_backend as backend;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate pulldown_cmark;
extern crate syntect;

#[macro_use]
mod macros;
mod doc_backend;
mod doc_builder;
mod doc_compiler;
mod doc_listeners;
mod doc_options;
mod escape;
mod processor;
mod service_processor;
mod tuple_processor;
mod type_processor;
mod enum_processor;
mod interface_processor;
mod index_processor;
mod package_processor;
mod rendering;
mod highlighting;

pub const NORMALIZE_CSS_NAME: &str = "normalize.css";
pub const DOC_CSS_NAME: &str = "doc.css";
pub const EXT: &str = "html";
pub const INDEX: &str = "index";
pub const DEFAULT_THEME: &str = "light";
pub const DEFAULT_SYNTAX_THEME: &str = "ayu-mirage";

use self::backend::{App, Arg, ArgMatches, CompilerOptions, Environment, Options};
use self::backend::errors::*;
use self::doc_backend::DocBackend;
use self::doc_compiler::DocCompiler;
use self::doc_listeners::DocListeners;
use self::doc_options::DocOptions;
use highlighting::THEME_SET;
use manifest::Manifest;
use syntect::highlighting::Theme;

fn setup_module(module: &str) -> Result<Box<DocListeners>> {
    let _module: Box<DocListeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn setup_listeners(options: Options) -> Result<(DocOptions, Box<DocListeners>)> {
    let mut listeners: Vec<Box<DocListeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = DocOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn shared_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.arg(
        Arg::with_name("theme")
            .long("theme")
            .takes_value(true)
            .help("Theme to use"),
    );

    let out = out.arg(
        Arg::with_name("syntax-theme")
            .long("syntax-theme")
            .takes_value(true)
            .help(
                "Syntax theme to use (use --list-syntax-themes for available)",
            ),
    );

    let out = out.arg(Arg::with_name("skip-static").long("skip-static").help(
        "Skip building \
         with static \
         files",
    ));

    let out = out.arg(
        Arg::with_name("list-syntax-themes")
            .long("list-syntax-themes")
            .help("List available syntax themes"),
    );

    out
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    shared_options(out).about("Compile Documentation")
}

/// Load and execute the provided clojure with a syntax theme.
fn with_syntax_theme<F>(matches: &ArgMatches, manifest: &Manifest, f: F) -> Result<()>
where
    F: FnOnce(&Theme) -> Result<()>,
{
    let syntax_theme = matches
        .value_of("syntax-theme")
        .or_else(|| manifest.doc.syntax_theme.as_ref().map(String::as_str))
        .unwrap_or(DEFAULT_SYNTAX_THEME);

    if let Some(syntax_theme) = THEME_SET.themes.get(syntax_theme) {
        f(syntax_theme)
    } else {
        warn!(
            "No syntax theme named `{}`, falling back to default",
            syntax_theme
        );
        let default_theme: Theme = Default::default();
        f(&default_theme)
    }
}

pub fn compile(
    env: Environment,
    options: Options,
    compiler_options: CompilerOptions,
    matches: &ArgMatches,
    manifest: &Manifest,
) -> Result<()> {
    if matches.is_present("list-syntax-themes") {
        let mut names: Vec<(&str, &Theme)> = THEME_SET.themes.iter().map(|e| (e.0.as_str(), e.1)).collect::<Vec<_>>();
        names.sort_by(|a, b| a.0.cmp(b.0));

        println!("Available Syntax Themes:");

        for (id, theme) in names {
            let name = theme.name.as_ref().map(String::as_str).unwrap_or("*no name*");
            let author = theme.author.as_ref().map(String::as_str).unwrap_or("*unknown*");
            println!("{} - {} by {}", id, name, author);
        }

        return Ok(());
    }

    let theme = matches
        .value_of("theme")
        .unwrap_or(DEFAULT_THEME)
        .to_owned();

    let skip_static = matches.is_present("skip-static");

    with_syntax_theme(matches, manifest, |syntax_theme| {
        let (options, listeners) = setup_listeners(options)?;
        let backend = DocBackend::new(env, options, listeners, theme, syntax_theme);
        let compiler = DocCompiler::new(backend, compiler_options.out_path, skip_static);
        compiler.compile()
    })
}
