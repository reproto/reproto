#![recursion_limit = "1000"]

#[macro_use]
mod macros;
mod doc_builder;
mod doc_compiler;
mod enum_processor;
mod escape;
mod index_processor;
mod interface_processor;
mod package_processor;
mod processor;
mod rendering;
mod service_processor;
mod tuple_processor;
mod type_processor;

pub const NORMALIZE_CSS_NAME: &str = "normalize.css";
pub const DOC_CSS_NAME: &str = "doc.css";
pub const EXT: &str = "html";
pub const INDEX: &str = "index";
pub const DEFAULT_THEME: &str = "light";
pub const DEFAULT_SYNTAX_THEME: &str = "ayu-mirage";

use crate::doc_compiler::DocCompiler;
use clap::{App, Arg, ArgMatches};
use manifest::Manifest;
use reproto_core::errors::Result;
use reproto_core::CoreFlavor;
use std::collections::HashMap;
use syntect::dumps::from_binary;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use trans::Session;

include!(concat!(env!("OUT_DIR"), "/themes.rs"));

fn build_themes() -> HashMap<&'static str, &'static [u8]> {
    let mut m = HashMap::new();

    for (key, value) in build_themes_vec() {
        m.insert(key, value);
    }

    m
}

static SYNTAX_DUMP: &'static [u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/dumps/syntaxdump"));
static THEME_DUMP: &'static [u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/dumps/themedump"));

fn load_syntax_set() -> SyntaxSet {
    from_binary(SYNTAX_DUMP)
}

fn load_theme_set() -> ThemeSet {
    from_binary(THEME_DUMP)
}

pub fn shared_options<'a>(out: App<'a>) -> App<'a> {
    let out = out.arg(
        Arg::with_name("theme")
            .long("theme")
            .takes_value(true)
            .help("Theme to use (use `--list-themes` for available)"),
    );

    let out = out.arg(
        Arg::with_name("list-themes")
            .long("list-themes")
            .help("List available themes"),
    );

    let out = out.arg(
        Arg::with_name("syntax-theme")
            .long("syntax-theme")
            .takes_value(true)
            .help("Syntax theme to use (use `--list-syntax-themes` for available)"),
    );

    let out = out.arg(
        Arg::with_name("list-syntax-themes")
            .long("list-syntax-themes")
            .help("List available syntax themes"),
    );

    let out = out.arg(
        Arg::with_name("skip-static")
            .long("skip-static")
            .help("Skip building with static files"),
    );

    out
}

pub fn compile_options<'a>(out: App<'a>) -> App<'a> {
    shared_options(out).about("Compile Documentation")
}

/// Load and execute the provided clojure with a syntax theme.
fn with_initialized<F>(
    matches: &ArgMatches,
    manifest: Manifest,
    themes: &HashMap<&'static str, &'static [u8]>,
    f: F,
) -> Result<()>
where
    F: FnOnce(&Theme, &SyntaxSet, &[u8]) -> Result<()>,
{
    let syntax_theme = matches
        .try_get_one::<String>("syntax-theme")
        .ok()
        .and_then(|theme| Some(theme?.as_str()))
        .or_else(|| manifest.doc.syntax_theme.as_deref())
        .unwrap_or(DEFAULT_SYNTAX_THEME);

    let default_theme: Theme = Default::default();
    let theme_set = load_theme_set();
    let syntax_set = load_syntax_set();

    let syntax_theme = if let Some(syntax_theme) = theme_set.themes.get(syntax_theme) {
        syntax_theme
    } else {
        log::warn!(
            "No syntax theme named `{}`, falling back to default",
            syntax_theme
        );

        &default_theme
    };

    let theme = matches
        .try_get_one::<String>("theme")
        .ok()
        .and_then(|theme| Some(theme?.as_str()))
        .unwrap_or(DEFAULT_THEME);

    let theme_css = if let Some(theme_css) = themes.get(theme) {
        theme_css
    } else {
        log::warn!("No syntax theme named `{}`, falling back to default", theme);

        themes
            .get(DEFAULT_THEME)
            .ok_or_else(|| format!("no such default theme: {}", DEFAULT_THEME))?
    };

    f(syntax_theme, &syntax_set, theme_css)
}

fn list_themes(themes: &HashMap<&'static str, &'static [u8]>) -> Result<()> {
    let mut names = themes.keys().collect::<Vec<_>>();
    names.sort();

    println!("Available Themes:");

    for id in names {
        println!("{}", id);
    }

    Ok(())
}

fn list_syntax_themes() -> Result<()> {
    let theme_set = load_theme_set();

    let mut names: Vec<(&str, &Theme)> = theme_set
        .themes
        .iter()
        .map(|e| (e.0.as_str(), e.1))
        .collect::<Vec<_>>();

    names.sort_by(|a, b| a.0.cmp(b.0));

    println!("Available Syntax Themes:");

    for (id, theme) in names {
        let name = theme
            .name
            .as_ref()
            .map(String::as_str)
            .unwrap_or("*no name*");

        let author = theme
            .author
            .as_ref()
            .map(String::as_str)
            .unwrap_or("*unknown*");

        println!("{} - {} by {}", id, name, author);
    }

    Ok(())
}

pub fn compile(
    session: Session<CoreFlavor>,
    matches: &ArgMatches,
    manifest: Manifest,
) -> Result<()> {
    let session = session.translate_default()?;

    let themes = build_themes();

    let mut done = false;

    if matches.try_contains_id("list-themes").unwrap_or_default() {
        list_themes(&themes)?;
        done = true;
    }

    if matches
        .try_contains_id("list-syntax-themes")
        .unwrap_or_default()
    {
        list_syntax_themes()?;
        done = true;
    }

    // other task performed (e.g. listing themes).
    if done {
        return Ok(());
    }

    let skip_static = matches.try_contains_id("skip-static").unwrap_or_default();

    let out = manifest
        .output
        .as_ref()
        .ok_or("Missing `--out` or `output=`")?
        .clone();

    with_initialized(
        matches,
        manifest,
        &themes,
        |syntax_theme, syntax_set, theme_css| {
            let compiler = DocCompiler {
                session,
                out_path: out.clone(),
                skip_static,
                theme_css,
                syntax_theme,
                syntax_set,
            };

            compiler.compile()
        },
    )?;

    println!("Wrote documentation in: {}", out.display());

    Ok(())
}
