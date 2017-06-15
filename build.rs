extern crate lalrpop_snap;
extern crate toml;
extern crate handlebars;
extern crate serde_json;

use serde_json::value::Map;
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::result;

#[derive(Debug)]
pub enum Error {
    Message(&'static str),
    Io(::std::io::Error),
    Fmt(::std::fmt::Error),
    Render(handlebars::RenderError),
    Template(handlebars::TemplateError),
    TomlDe(toml::de::Error),
    Env(env::VarError),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Error {
        Error::Message(value)
    }
}

impl From<::std::io::Error> for Error {
    fn from(value: ::std::io::Error) -> Error {
        Error::Io(value)
    }
}

impl From<::std::fmt::Error> for Error {
    fn from(value: ::std::fmt::Error) -> Error {
        Error::Fmt(value)
    }
}

impl From<env::VarError> for Error {
    fn from(value: env::VarError) -> Error {
        Error::Env(value)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(value: handlebars::RenderError) -> Error {
        Error::Render(value)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(value: handlebars::TemplateError) -> Error {
        Error::Template(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Error {
        Error::TomlDe(value)
    }
}

type Result<T> = result::Result<T, Error>;

fn read_file(path: &str) -> String {
    let mut f = File::open(path).map_err(|e| format!("cannot open: {}: {}", path, e)).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    content
}

/// Generate and build themes.rs
fn process_colors() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_path = Path::new(&out_dir);

    let colors_content = read_file("themes.toml");
    let template_content = read_file("src/backend/doc/static/doc._.css.hbs");

    let value: toml::Value = colors_content.parse()?;
    let schemes = value.as_table().ok_or_else(|| Error::Message("not a table"))?;

    let mut handlebar = handlebars::Handlebars::new();

    handlebar.register_template_string("doc", template_content)?;

    let mut entries = Vec::new();
    let mut themes = String::new();

    for (key, value) in schemes {
        let colors_in = value.as_table().ok_or_else(|| Error::Message("not a table"))?;
        let mut colors = Map::new();

        for (k, color) in colors_in {
            let value = color.as_str().ok_or_else(|| Error::Message("expected string"))?;
            colors.insert(k.to_owned(), handlebars::to_json(&value));
        }

        let result = handlebar.render("doc", &colors)?;

        let key_upper = key.to_uppercase();

        let name = format!("doc.{}.css", key);

        writeln!(themes,
                 "const DOC_CSS_{}: &[u8] = include_bytes!(\"{}/{}\");",
                 key_upper,
                 out_path.display(),
                 name)?;

        entries.push(format!("(\"{}\", DOC_CSS_{})", key, key_upper));

        let mut f = File::create(out_path.join(name))?;
        f.write_all(&result.into_bytes())?;
    }

    writeln!(themes, "")?;
    writeln!(themes,
             "pub fn build_themes_vec() -> Vec<(&'static str, &'static [u8])> {{")?;
    writeln!(themes, "  vec![{}]", entries.join(", "))?;
    writeln!(themes, "}}")?;

    let mut themes_file = File::create(out_path.join("themes.rs"))?;
    themes_file.write_all(&themes.into_bytes())?;

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=themes.toml");
    println!("cargo:rerun-if-changed=src/parser/parser.lalrpop");

    lalrpop_snap::process_root().unwrap();
    process_colors().unwrap();
}
