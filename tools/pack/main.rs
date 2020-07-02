use anyhow::{format_err, Context as _, Result};
use clap::{App, Arg};
use std::path::Path;
use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSetBuilder;

fn main() -> Result<()> {
    let app = App::new("reproto-pack")
        .version("0.0.1")
        .author("John-John Tedro <udoprog@tedro.se>")
        .about("Creates binary packs for syntaxes and themes for reproto")
        .arg(
            Arg::with_name("build-syntax")
                .long("build-syntax")
                .help("build syntax")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("build-themes")
                .long("build-themes")
                .help("build themes")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let root = std::env::current_dir()?;
    let themes = root.join("themes");
    let syntaxes = root.join("syntaxes");

    if !themes.is_dir() {
        panic!("no such directory: {}", themes.display());
    }

    if !syntaxes.is_dir() {
        panic!("no such directory: {}", syntaxes.display());
    }

    if let Some(path) = matches.value_of("build-syntax").map(Path::new) {
        let mut ss = SyntaxSetBuilder::new();
        ss.add_plain_text_syntax();
        ss.add_from_folder(syntaxes, true)
            .map_err(|e| format_err!("{}", e))
            .with_context(|| format_err!("syntaxes to load"))?;

        let ss = ss.build();
        println!("building: {}", path.display());
        dump_to_file(&ss, path).with_context(|| format_err!("syntaxes to pack"))?;
    }

    if let Some(path) = matches.value_of("build-themes").map(Path::new) {
        println!("loading themes from: {}", themes.display());
        let ts = ThemeSet::load_from_folder(themes)
            .map_err(|e| format_err!("{}", e))
            .with_context(|| format_err!("themes to load"))?;
        println!("building: {}", path.display());
        dump_to_file(&ts, path).with_context(|| format_err!("themes to pack"))?;
    }

    Ok(())
}
