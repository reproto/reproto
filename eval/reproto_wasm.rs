extern crate genco;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate stdweb;

extern crate reproto_backend_java as java;
extern crate reproto_backend_js as js;
extern crate reproto_backend_json as json;
extern crate reproto_backend_python as python;
extern crate reproto_backend_reproto as reproto;
extern crate reproto_backend_rust as rust;
extern crate reproto_compile as compile;
extern crate reproto_core as core;
extern crate reproto_derive as derive;
extern crate reproto_manifest as manifest;
extern crate reproto_parser as parser;

use genco::WriteTokens;
use std::str;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Format {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Output {
    #[serde(rename = "reproto")]
    Reproto,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "js")]
    JavaScript,
    #[serde(rename = "json")]
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JavaSettings {
    jackson: bool,
    lombok: bool,
}

js_serializable!(JavaSettings);
js_deserializable!(JavaSettings);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RustSettings {
    chrono: bool,
}

js_serializable!(RustSettings);
js_deserializable!(RustSettings);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    java: JavaSettings,
    rust: RustSettings,
}

js_serializable!(Settings);
js_deserializable!(Settings);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Derive {
    content: String,
    root_name: String,
    format: Format,
    output: Output,
    package_prefix: Option<String>,
    settings: Settings,
}

js_serializable!(Derive);
js_deserializable!(Derive);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeriveResult {
    result: Option<String>,
    error: Option<String>,
}

js_serializable!(DeriveResult);
js_deserializable!(DeriveResult);

fn derive(derive: Derive) -> DeriveResult {
    return match try_derive(derive) {
        Err(e) => DeriveResult {
            result: None,
            error: Some(e.display().to_string()),
        },
        Ok(result) => DeriveResult {
            result: Some(result),
            error: None,
        },
    };

    fn try_derive(derive: Derive) -> core::errors::Result<String> {
        let format: Box<derive::Format> = match derive.format {
            Format::Json => Box::new(derive::Json),
            Format::Yaml => Box::new(derive::Yaml),
        };

        let bytes = derive.content.as_bytes().to_vec();
        let object = core::BytesObject::new("web".to_string(), Arc::new(bytes));

        let package_prefix = derive
            .package_prefix
            .map(|s| core::RpPackage::parse(&s))
            .unwrap_or_else(|| core::RpPackage::parse("io.reproto.github"));

        let decl = derive::derive(
            derive::Derive::new(derive.root_name, format, Some(package_prefix.clone())),
            &object,
        )?;

        match derive.output {
            Output::Reproto => {
                let toks = reproto::format(&decl)?;
                let mut buffer = String::new();
                buffer.write_file(toks, &mut ())?;
                return Ok(buffer);
            }
            _ => {}
        };

        let mut buf = String::new();

        let simple_compile = compile::SimpleCompile {
            decl: decl,
            package_prefix: Some(package_prefix),
        };

        match derive.output {
            Output::Java => {
                let mut modules = Vec::new();

                if derive.settings.java.jackson {
                    modules.push(java::JavaModule::Jackson);
                }

                if derive.settings.java.lombok {
                    modules.push(java::JavaModule::Lombok);
                }

                compile::simple_compile::<java::JavaLang, _>(
                    &mut buf,
                    simple_compile,
                    modules,
                    java::compile,
                )?;
            }
            Output::Python => {
                let mut modules = Vec::new();

                compile::simple_compile::<python::PythonLang, _>(
                    &mut buf,
                    simple_compile,
                    modules,
                    python::compile,
                )?;
            }
            Output::Rust => {
                let mut modules = Vec::new();

                if derive.settings.rust.chrono {
                    modules.push(rust::RustModule::Chrono);
                }

                compile::simple_compile::<rust::RustLang, _>(
                    &mut buf,
                    simple_compile,
                    modules,
                    rust::compile,
                )?;
            }
            Output::JavaScript => {
                let mut modules = Vec::new();

                compile::simple_compile::<js::JsLang, _>(
                    &mut buf,
                    simple_compile,
                    modules,
                    js::compile,
                )?;
            }
            Output::Json => {
                let mut modules = Vec::new();

                compile::simple_compile::<json::JsonLang, _>(
                    &mut buf,
                    simple_compile,
                    modules,
                    json::compile,
                )?;
            }
            output => {
                return Err(format!("Unsupported output: {:?}", output).into());
            }
        }

        Ok(buf)
    }
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.derive = @{derive};
    }
}
