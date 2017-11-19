//! Backend module for Documentation.

use backend::{Environment, PackageUtils};
use core::Version;
use doc_listeners::DocListeners;
use doc_options::DocOptions;
use std::collections::HashMap;
use syntect::highlighting::Theme;

pub struct DocBackend<'a> {
    pub env: Environment,
    #[allow(dead_code)]
    options: DocOptions,
    listeners: Box<DocListeners>,
    pub theme: String,
    pub themes: HashMap<&'static str, &'static [u8]>,
    pub syntax_theme: &'a Theme,
}

include!(concat!(env!("OUT_DIR"), "/themes.rs"));

fn build_themes() -> HashMap<&'static str, &'static [u8]> {
    let mut m = HashMap::new();

    for (key, value) in build_themes_vec() {
        m.insert(key, value);
    }

    m
}

impl<'a> DocBackend<'a> {
    pub fn new(
        env: Environment,
        options: DocOptions,
        listeners: Box<DocListeners>,
        theme: String,
        syntax_theme: &Theme,
    ) -> DocBackend {
        DocBackend {
            env: env,
            options: options,
            listeners: listeners,
            theme: theme,
            themes: build_themes(),
            syntax_theme: syntax_theme,
        }
    }
}

impl<'a> PackageUtils for DocBackend<'a> {
    fn version_package(input: &Version) -> String {
        format!("{}", input).replace(Self::package_version_unsafe, "_")
    }
}
