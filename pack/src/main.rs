extern crate syntect;
extern crate clap;

use clap::{App, Arg};
use std::path::Path;
use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn main() {
    let app = App::new("reproto-pack")
        .version("0.0.1")
        .author("John-John Tedro <udoprog@tedro.se>")
        .about("Creates binary packs for syntaxes and themes for ReProto")
        .arg(Arg::with_name("skip-defaults").long("skip-defaults").help(
            "skip building defaults",
        ))
        .arg(Arg::with_name("build-syntax").long("build-syntax").help(
            "build syntax",
        ))
        .arg(Arg::with_name("build-themes").long("build-themes").help(
            "build themes",
        ));

    let matches = app.get_matches();

    let skip_defaults = matches.is_present("skip-defaults");
    let build_syntax = matches.is_present("build-syntax") || !skip_defaults;
    let build_themes = matches.is_present("build-themes") || !skip_defaults;

    let syntaxes = Path::new("syntaxes");
    let themes = Path::new("themes");
    let dumps = Path::new("dumps");

    if build_syntax {
        let mut ss = SyntaxSet::new();
        ss.load_plain_text_syntax();
        ss.load_syntaxes(syntaxes, true).expect("syntaxes to load");
        let dump = dumps.join("syntaxdump");
        println!("building: {}", dump.display());
        dump_to_file(&ss, dump).expect("syntaxes to pack");
    }

    if build_themes {
        let ts = ThemeSet::load_from_folder(themes).expect("themes to load");
        let dump = dumps.join("themedump");
        println!("building: {}", dump.display());
        dump_to_file(&ts, dump).expect("themes to pack");
    }
}
