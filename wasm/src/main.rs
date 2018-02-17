extern crate genco;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate stdweb;

extern crate reproto_backend as backend;
extern crate reproto_backend_java as java;
extern crate reproto_backend_js as js;
extern crate reproto_backend_json as json;
extern crate reproto_backend_python as python;
extern crate reproto_backend_reproto as reproto;
extern crate reproto_backend_rust as rust;
extern crate reproto_core as core;
extern crate reproto_derive as derive;
extern crate reproto_manifest as manifest;
extern crate reproto_parser as parser;

use genco::WriteTokens;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Derive {
    content: String,
    format: Option<String>,
}

js_deserializable!(Derive);
js_serializable!(Derive);

fn derive(derive: Derive) -> String {
    js! {
        console.log("Fooooo", @{&derive}.content);
    }

    let format: Box<derive::Format> = match derive.format.as_ref().map(|s| s.as_str()) {
        None | Some("json") => Box::new(derive::Json),
        Some("yaml") => Box::new(derive::Yaml),
        Some(value) => panic!("Unsupported format: {}", value),
    };

    let bytes = derive.content.as_bytes().to_vec();
    let object = core::BytesObject::new("web".to_string(), Arc::new(bytes));

    let decl = derive::derive(format, &object).expect("bad derive");
    let toks = reproto::format(&decl).expect("bad format");

    let mut buffer = String::new();
    buffer.write_file(toks, &mut ()).expect("bad write");
    buffer
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.derive = @{derive};
    }
}
