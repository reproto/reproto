//! Helper macros for CLI.

/// Helper macro to perform a correct manifest_use invocation.
///
/// In particular, this guarantees that static invocation for manifest_use is correct.
///
/// If no language is specified, the special NoLang target will be used which prevents illegal
/// modules from being specified.
macro_rules! do_manifest_use {
    ($matches:expr, $preamble:expr, $fn:expr) => {{
        let language = $preamble.language.as_ref().cloned();

        match language {
            Some(::manifest::Language::Java) => {
                ::ops::manifest_use::<::java::JavaLang, _>($matches, $preamble, $fn)
            }
            Some(::manifest::Language::Js) => {
                ::ops::manifest_use::<::js::JsLang, _>($matches, $preamble, $fn)
            }
            Some(::manifest::Language::Json) => {
                ::ops::manifest_use::<::json::JsonLang, _>($matches, $preamble, $fn)
            }
            Some(::manifest::Language::Python) => {
                ::ops::manifest_use::<::python::PythonLang, _>($matches, $preamble, $fn)
            }
            Some(::manifest::Language::Rust) => {
                ::ops::manifest_use::<::rust::RustLang, _>($matches, $preamble, $fn)
            }
            None => ::ops::manifest_use::<::manifest::NoLang, _>($matches, $preamble, $fn),
        }
    }};
}
