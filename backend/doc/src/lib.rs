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
mod doc_builder;
mod doc_compiler;
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
use self::doc_compiler::DocCompiler;
use highlighting::THEME_SET;
use manifest::Manifest;
use std::collections::HashMap;
use syntect::highlighting::Theme;

include!(concat!(env!("OUT_DIR"), "/themes.rs"));

fn build_themes() -> HashMap<&'static str, &'static [u8]> {
    let mut m = HashMap::new();

    for (key, value) in build_themes_vec() {
        m.insert(key, value);
    }

    m
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
fn with_initialized<F>(matches: &ArgMatches, manifest: &Manifest, f: F) -> Result<()>
where
    F: FnOnce(&Theme, &[u8]) -> Result<()>,
{
    let themes = build_themes();

    let syntax_theme = matches
        .value_of("syntax-theme")
        .or_else(|| manifest.doc.syntax_theme.as_ref().map(String::as_str))
        .unwrap_or(DEFAULT_SYNTAX_THEME);

    let default_theme: Theme = Default::default();

    let syntax_theme = if let Some(syntax_theme) = THEME_SET.themes.get(syntax_theme) {
        syntax_theme
    } else {
        warn!(
            "No syntax theme named `{}`, falling back to default",
            syntax_theme
        );

        &default_theme
    };

    let theme = matches.value_of("theme").unwrap_or(DEFAULT_THEME);

    let theme_css = if let Some(theme_css) = themes.get(theme) {
        theme_css
    } else {
        warn!("No syntax theme named `{}`, falling back to default", theme);

        themes.get(DEFAULT_THEME).ok_or_else(|| {
            format!("no such default theme: {}", DEFAULT_THEME)
        })?
    };

    f(syntax_theme, theme_css)
}

pub fn compile(
    env: Environment,
    _options: Options,
    compiler_options: CompilerOptions,
    matches: &ArgMatches,
    manifest: &Manifest,
) -> Result<()> {
    if matches.is_present("list-syntax-themes") {
        let mut names: Vec<(&str, &Theme)> = THEME_SET
            .themes
            .iter()
            .map(|e| (e.0.as_str(), e.1))
            .collect::<Vec<_>>();
        names.sort_by(|a, b| a.0.cmp(b.0));

        println!("Available Syntax Themes:");

        for (id, theme) in names {
            let name = theme.name.as_ref().map(String::as_str).unwrap_or(
                "*no name*",
            );
            let author = theme.author.as_ref().map(String::as_str).unwrap_or(
                "*unknown*",
            );
            println!("{} - {} by {}", id, name, author);
        }

        return Ok(());
    }

    let skip_static = matches.is_present("skip-static");

    with_initialized(matches, manifest, move |syntax_theme, theme_css| {
        let compiler = DocCompiler {
            env: env,
            out_path: compiler_options.out_path,
            skip_static: skip_static,
            theme_css: theme_css,
            syntax_theme: syntax_theme,
        };

        compiler.compile()
    })
}
