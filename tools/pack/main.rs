extern crate clap;
extern crate syntect;

use clap::{App, Arg};
use std::env;
use std::path::Path;
use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn main() {
    let app = App::new("reproto-pack")
        .version("0.0.1")
        .author("John-John Tedro <udoprog@tedro.se>")
        .about("Creates binary packs for syntaxes and themes for reproto")
        .arg(
            Arg::with_name("build-syntax")
                .long("build-syntax")
                .help("build syntax")
                .takes_value(true),
        ).arg(
            Arg::with_name("build-themes")
                .long("build-themes")
                .help("build themes")
                .takes_value(true),
        );

    let mut args = env::args();

    let root = args
        .next()
        .and_then(|arg| Path::new(arg.as_str()).canonicalize().ok())
        .and_then(|p| {
            p.parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
                .map(Path::to_owned)
        }).expect("locating root directory");

    let matches = app.get_matches();

    let themes = root.join("themes");
    let syntaxes = root.join("syntaxes");

    if !themes.is_dir() {
        panic!("no such directory: {}", themes.display());
    }

    if !syntaxes.is_dir() {
        panic!("no such directory: {}", syntaxes.display());
    }

    if let Some(path) = matches.value_of("build-syntax").map(Path::new) {
        let mut ss = SyntaxSet::new();
        ss.load_plain_text_syntax();
        ss.load_syntaxes(syntaxes, true).expect("syntaxes to load");
        println!("building: {}", path.display());
        dump_to_file(&ss, path).expect("syntaxes to pack");
    }

    if let Some(path) = matches.value_of("build-themes").map(Path::new) {
        let ts = ThemeSet::load_from_folder(themes).expect("themes to load");
        println!("building: {}", path.display());
        dump_to_file(&ts, path).expect("themes to pack");
    }
}
