extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

extern crate reproto_ast as ast;
extern crate reproto_backend_csharp as csharp;
extern crate reproto_backend_go as go;
extern crate reproto_backend_java as java;
extern crate reproto_backend_js as js;
extern crate reproto_backend_json as json;
extern crate reproto_backend_python as python;
extern crate reproto_backend_reproto as reproto;
extern crate reproto_backend_rust as rust;
extern crate reproto_backend_openapi as openapi;
extern crate reproto_backend_swift as swift;
extern crate reproto_backend_dart as dart;
extern crate reproto_compile as compile;
extern crate reproto_core as core;
extern crate reproto_derive as derive;
extern crate reproto_manifest as manifest;
extern crate reproto_parser as parser;

use std::any::Any;
use std::collections::BTreeMap;
use std::str;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "reproto")]
    Reproto,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Output {
    #[serde(rename = "reproto")]
    Reproto,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "csharp")]
    Csharp,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "dart")]
    Dart,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "openapi")]
    OpenApi,
    #[serde(rename = "js")]
    JavaScript,
    #[serde(rename = "json")]
    Json,
}

impl Output {
    /// Convert into a manifest language and accumulate modules.
    fn into_lang(self, settings: Settings, modules: &mut Vec<Box<dyn Any>>) -> Box<dyn manifest::Lang> {
        match self {
            Output::Reproto => Box::new(reproto::ReprotoLang),
            Output::Java => {
                if settings.java.jackson {
                    modules.push(Box::new(java::JavaModule::Jackson));
                }

                if settings.java.lombok {
                    modules.push(Box::new(java::JavaModule::Lombok));
                }

                Box::new(java::JavaLang)
            }
            Output::Csharp => {
                if settings.csharp.json_net {
                    modules.push(Box::new(csharp::CsharpModule::JsonNet));
                }

                Box::new(csharp::CsharpLang)
            }
            Output::Go => {
                if settings.go.encoding_json {
                    modules.push(Box::new(go::GoModule::EncodingJson));
                }

                Box::new(go::GoLang)
            }
            Output::Swift => {
                if settings.swift.codable {
                    modules.push(Box::new(swift::SwiftModule::Codable));
                }

                if settings.swift.simple {
                    modules.push(Box::new(swift::SwiftModule::Simple));
                }

                Box::new(swift::SwiftLang)
            }
            Output::Dart => {
                Box::new(dart::DartLang)
            }
            Output::Python => {
                if settings.python.requests {
                    let config = python::module::RequestsConfig::default();
                    modules.push(Box::new(python::PythonModule::Requests(config)));
                }

                Box::new(python::PythonLang)
            }
            Output::Rust => {
                if settings.rust.chrono {
                    modules.push(Box::new(rust::RustModule::Chrono));
                }

                if settings.rust.reqwest {
                    modules.push(Box::new(rust::RustModule::Reqwest));
                }

                Box::new(rust::RustLang)
            }
            Output::OpenApi => {
                if settings.openapi.json {
                    modules.push(Box::new(openapi::OpenApiModule::Json));
                }

                Box::new(openapi::OpenApiLang)
            }
            Output::JavaScript => Box::new(js::JsLang),
            Output::Json => Box::new(json::JsonLang),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaSettings {
    jackson: bool,
    lombok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonSettings {
    requests: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsharpSettings {
    json_net: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoSettings {
    encoding_json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftSettings {
    codable: bool,
    simple: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DartSettings {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustSettings {
    chrono: bool,
    reqwest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSettings {
    json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    java: JavaSettings,
    python: PythonSettings,
    swift: SwiftSettings,
    dart: DartSettings,
    rust: RustSettings,
    openapi: OpenApiSettings,
    csharp: CsharpSettings,
    go: GoSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    package: String,
    version: Option<String>,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "content")]
    Content { content: String },
    #[serde(rename = "file_index")]
    FileIndex { index: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derive {
    content: Content,
    files: Vec<File>,
    root_name: String,
    format: Format,
    output: Output,
    package_prefix: Option<String>,
    settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    message: String,
    row_start: u32,
    row_end: u32,
    col_start: u32,
    col_end: u32,
}

impl Marker {
    /// Convert an error into a marker.
    fn try_from_error(
        source: &core::Source,
        p: &core::Span,
        message: &str,
    ) -> core::errors::Result<Marker> {
        let (_, line, (s, e)) = core::utils::find_line(source.read()?, (p.start, p.end))?;

        let marker = Marker {
            message: message.to_string(),
            row_start: line as u32,
            row_end: line as u32,
            col_start: s as u32,
            col_end: e as u32,
        };

        Ok(marker)
    }

    /// Safe building of markers with fallback.
    fn try_from_error_fb(source: &core::Source, p: &core::Span, message: &str) -> Marker {
        if let Ok(m) = Self::try_from_error(source, p, message) {
            return m;
        }

        // no positional information
        Self::safe_from_error(message)
    }

    /// Safely build a marker.
    fn safe_from_error(message: &str) -> Marker {
        Marker {
            message: message.to_string(),
            row_start: 0,
            row_end: 0,
            col_start: 0,
            col_end: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeriveFile {
    path: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeriveResult {
    files: Vec<DeriveFile>,
    error: Option<String>,
    error_markers: Vec<Marker>,
    info_markers: Vec<Marker>,
}

#[derive(Debug, Clone)]
pub struct ParsedFile {
    package: RpPackage,
    version: Option<core::Version>,
    content: String,
}

#[wasm_bindgen]
pub fn derive(derive: &JsValue) -> JsValue {
    let out = try_derive(derive).unwrap_or_else(|e| DeriveResult {
        files: vec![],
        error: Some(e.to_string()),
        error_markers: vec![],
        info_markers: vec![],
    });

    return JsValue::from_serde(&out).expect("bad output");

    fn try_derive(derive: &JsValue) -> Result<DeriveResult, String> {
        let derive: Derive = derive.into_serde().map_err(|e| e.to_string())?;

        let mut reporter: Vec<core::Reported> = Vec::new();

        let (source, package) = content_source(&derive).map_err(|e| e.display().to_string())?;

        let out = match inner_derive(derive, &source, package, &mut reporter) {
            Ok(result) => DeriveResult {
                files: result,
                error: None,
                error_markers: vec![],
                info_markers: vec![],
            },
            Err(e) => {
                let mut error_markers = Vec::new();
                let mut info_markers = Vec::new();

                for r in reporter {
                    for (source, d) in r.diagnostics_with_sources() {
                        match *d {
                            core::Diagnostic::Error {ref span, ref message } => {
                                error_markers.push(Marker::try_from_error_fb(
                                    source,
                                    span,
                                    message.as_str(),
                                ));
                            }
                            core::Diagnostic::Info { ref span, ref message } => {
                                info_markers.push(Marker::try_from_error_fb(
                                    source,
                                    span,
                                    message.as_str(),
                                ));
                            }
                            _ => {}
                        }
                    }
                }

                DeriveResult {
                    files: vec![],
                    error: Some(e.display().to_string()),
                    error_markers: error_markers,
                    info_markers: info_markers,
                }
            }
        };

        Ok(out)
    }

    /// Construct content source information.
    fn content_source(
        derive: &Derive,
    ) -> core::errors::Result<(core::Source, Option<RpVersionedPackage>)> {
        let out = match derive.content {
            Content::Content { ref content } => {
                let bytes = content.as_bytes().to_vec();
                let source = core::Source::bytes("web", bytes);

                (source, None)
            }
            Content::FileIndex { index } => {
                let file = derive
                    .files
                    .get(index)
                    .ok_or_else(|| format!("No file for index: {}", index))?;

                let bytes = file.content.as_bytes().to_vec();
                let source = core::Source::bytes(file.package.to_string(), bytes);

                let package = parse_package(&file)?;

                (source, Some(package))
            }
        };

        Ok(out)
    }

    fn inner_derive(
        derive: Derive,
        source: &core::Source,
        package: Option<RpVersionedPackage>,
        reporter: &mut dyn core::Reporter,
    ) -> core::errors::Result<Vec<DeriveFile>> {
        let package_prefix = derive
            .package_prefix
            .as_ref()
            .map(|s| RpPackage::parse(s))
            .unwrap_or_else(|| RpPackage::parse("io.reproto.github"));

        let input = match derive.format {
            Format::Json => derive_file(&derive, &package_prefix, source, Box::new(derive::Json))?,
            Format::Yaml => derive_file(&derive, &package_prefix, source, Box::new(derive::Yaml))?,
            Format::Reproto => compile::Input::Source(source.clone(), package),
        };

        let files = parse_files(derive.files)?;

        let mut resolver = MapResolver(files);

        let simple_compile = compile::SimpleCompile::new(input, reporter)
            .resolver(&mut resolver)
            .package_prefix(package_prefix);

        let mut modules = Vec::new();

        let settings = derive.settings;
        let lang = derive.output.into_lang(settings, &mut modules);

        let mut files = Vec::new();

        compile::simple_compile(
            |path, content| {
                files.push(DeriveFile {
                    path: path.to_string(),
                    content: content.to_string(),
                });

                Ok(())
            },
            simple_compile,
            modules,
            lang.as_ref(),
        )?;

        Ok(files)
    }

    fn parse_files(files: Vec<File>) -> core::errors::Result<Vec<ParsedFile>> {
        let mut out: Vec<ParsedFile> = Vec::new();

        for f in files {
            let package = parse_package(&f)?;

            out.push(ParsedFile {
                package: package.package,
                version: package.version,
                content: f.content,
            });
        }

        Ok(out)
    }

    fn parse_package(file: &File) -> core::errors::Result<RpVersionedPackage> {
        let package = RpPackage::parse(file.package.as_str());

        let version =
            if let Some(ref version) = file.version {
                Some(core::Version::parse(version.as_str()).map_err(|e| {
                    format!("{}: failed to parse version `{}`: {}", package, version, e)
                })?)
            } else {
                None
            };

        Ok(RpVersionedPackage::new(package, version))
    }

    fn derive_file<'input>(
        derive: &Derive,
        package_prefix: &RpPackage,
        source: &'input core::Source,
        format: Box<dyn derive::Format>,
    ) -> core::errors::Result<compile::Input<'input>> {
        let decl = derive::derive(
            derive::Derive::new(
                derive.root_name.to_string(),
                format,
                Some(package_prefix.clone()),
            ),
            source,
        )?;

        let file = ast::File {
            comment: vec!["Generated from reproto derive".to_string().into()],
            uses: vec![],
            attributes: vec![],
            decls: vec![decl],
        };

        let input = compile::Input::File(
            file,
            Some(RpVersionedPackage::new(package_prefix.clone(), None)),
        );

        Ok(input)
    }
}

/// Resolver using provided files.
struct MapResolver(Vec<ParsedFile>);

impl core::Resolver for MapResolver {
    fn resolve(
        &mut self,
        required: &RpRequiredPackage,
    ) -> core::errors::Result<Option<core::Resolved>> {
        let mut matches = BTreeMap::new();

        let package = &required.package;

        for file in self.0.iter() {
            if file.package != required.package {
                continue;
            }

            if file
                .version
                .as_ref()
                .map(|v| required.range.matches(v))
                .unwrap_or_else(|| required.range.matches_any())
            {
                let bytes = file.content.as_bytes().to_vec();
                let source = core::Source::bytes(package.to_string(), bytes);

                matches.insert(
                    file.version.clone(),
                    core::Resolved {
                        version: file.version.clone(),
                        source,
                    },
                );
            }
        }

        Ok(matches.into_iter().next_back().map(|v| v.1))
    }

    fn resolve_by_prefix(
        &mut self,
        prefix: &RpPackage,
    ) -> core::errors::Result<Vec<core::ResolvedByPrefix>> {
        let mut out = Vec::new();

        for file in self.0.iter() {
            if file.package.starts_with(prefix) {
                let bytes = file.content.as_bytes().to_vec();
                let source = core::Source::bytes(file.package.to_string(), bytes);
                let package =
                    RpVersionedPackage::new(file.package.clone(), file.version.clone());

                out.push(core::ResolvedByPrefix { package, source })
            }
        }

        Ok(out)
    }

    fn resolve_packages(&mut self) -> core::errors::Result<Vec<core::ResolvedByPrefix>> {
        self.resolve_by_prefix(&RpPackage::empty())
    }
}
