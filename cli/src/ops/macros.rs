//! Helper macros for CLI.

/// Helper macro to perform a correct manifest_use invocation.
///
/// In particular, this guarantees that static invocation for manifest_use is correct.
///
/// If no language is specified, the special NoLang target will be used which prevents illegal
/// modules from being specified.
macro_rules! do_manifest_use {
    ($ctx:expr, $matches:expr, $preamble:expr, $fn:expr) => {{
        let language = $preamble.language.as_ref().cloned();

        match language {
            Some(::manifest::Language::Java) => {
                ::build_spec::manifest_use::<::java::JavaLang, _>(
                    $ctx, $matches, $preamble, $fn
                )
            }
            Some(::manifest::Language::Js) => {
                ::build_spec::manifest_use::<::js::JsLang, _>(
                    $ctx, $matches, $preamble, $fn
                )
            }
            Some(::manifest::Language::Json) => {
                ::build_spec::manifest_use::<::json::JsonLang, _>(
                    $ctx, $matches, $preamble, $fn
                )
            }
            Some(::manifest::Language::Python) => {
                ::build_spec::manifest_use::<::python::PythonLang, _>(
                    $ctx, $matches, $preamble, $fn
                )
            }
            Some(::manifest::Language::Rust) => {
                ::build_spec::manifest_use::<::rust::RustLang, _>(
                    $ctx, $matches, $preamble, $fn
                )
            }
            None => ::build_spec::manifest_use::<::manifest::NoLang, _>(
                $ctx, $matches, $preamble, $fn
            ),
        }
    }};
}
