extern crate syntect;

use std::path::Path;
use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn main() {
    let sublime = Path::new("sublime");
    let themes = Path::new("themes");
    let syntaxes = Path::new("dumps");

    let mut ss = SyntaxSet::new();
    ss.load_plain_text_syntax();
    ss.load_syntaxes(sublime, true).expect("syntaxes to load");
    dump_to_file(&ss, syntaxes.join("syntaxdump")).expect("syntaxes to pack");

    let ts = ThemeSet::load_from_folder(themes).expect("themes to load");

    println!("themes:");

    for (i, path) in ts.themes.keys().enumerate() {
        println!("{}: {}", i, path);
    }

    dump_to_file(&ts, syntaxes.join("themedump")).expect("themes to pack");
}
