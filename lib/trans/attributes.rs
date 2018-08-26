//! Handle parsing of attributes.

use core::errors::Error;
use core::flavored::{
    Attributes, RpAccept, RpChannel, RpEndpointArgument, RpEndpointHttp, RpHttpMethod, RpPathSpec,
    RpValue,
};
use core::{self, Diagnostics, Import, Loc, RpStringValidate, Span, Version, WithSpan};
use features::Feature;
use into_model::IntoModel;
use path_parser;
use regex_parser;
use scope::Scope;
use std::collections::HashMap;

/// `#![feature(..)]` attributes.
pub fn features<'s, I>(
    scope: &'s Scope<I>,
    diag: &mut Diagnostics,
    attributes: &mut Attributes,
) -> Result<Vec<Loc<&'s Feature>>, ()>
where
    I: Import,
{
    let selection = match attributes.take_selection("feature") {
        Some(selection) => selection,
        None => return Ok(vec![]),
    };

    let mut out = Vec::new();

    let (mut selection, _) = Loc::take_pair(selection);

    for feature in selection.take_words() {
        let (feature, span) = Loc::take_pair(feature);

        let feature = match feature.into_identifier() {
            Ok(feature) => feature,
            Err(e) => {
                diag.err(span, e.display());
                continue;
            }
        };

        let feature = match scope.features.get(&feature) {
            Some(feature) => feature,
            None => {
                diag.err(span, "no such feature");
                continue;
            }
        };

        out.push(Loc::new(feature, span));
    }

    check_selection!(diag, selection);

    if diag.has_errors() {
        return Err(());
    }

    Ok(out)
}

#[derive(Debug, Default)]
pub struct Reproto {
    pub version: Option<Version>,
}

/// `#![reproto(..)]` attribute.
pub fn reproto(diag: &mut Diagnostics, attributes: &mut Attributes) -> Result<Reproto, ()> {
    let mut reproto = Reproto::default();

    let mut selection = match attributes.take_selection("reproto") {
        Some(selection) => selection,
        None => return Ok(reproto),
    };

    if let Some(version) = selection.take("version") {
        let (mut version, span) = Loc::take_pair(version);

        let v = version
            .as_string()
            .map_err(|_| Error::from("expected string"))
            .and_then(|v| Version::parse(v).map_err(|e| format!("bad version: {}", e).into()))
            .with_span(diag, &span)?;

        reproto.version = Some(v);
    };

    check_selection!(diag, selection);
    Ok(reproto)
}

/// `#[reserved(..)]` attribute.
pub fn reserved(
    diag: &mut Diagnostics,
    attributes: &mut Attributes,
) -> Result<HashMap<String, Span>, ()> {
    let mut reserved: HashMap<String, Span> = HashMap::new();

    let selection = match attributes.take_selection("reserved") {
        None => return Ok(reserved),
        Some(selection) => selection,
    };

    let (mut selection, _pos) = Loc::take_pair(selection);

    for word in selection.take_words() {
        let (field, span) = Loc::take_pair(word);
        let field = field
            .as_string()
            .map(|id| id.to_string())
            .with_span(diag, span)?;
        reserved.insert(field, span);
    }

    check_selection!(diag, selection);

    Ok(reserved)
}

/// `#[http(..)]` attribute for endpoints.
pub fn endpoint_http<I>(
    diag: &mut Diagnostics,
    scope: &mut Scope<I>,
    attributes: &mut Attributes,
    request: &mut Option<RpEndpointArgument>,
    response: Option<&Loc<RpChannel>>,
    arguments: &Vec<RpEndpointArgument>,
) -> Result<RpEndpointHttp, ()>
where
    I: Import,
{
    let mut http = RpEndpointHttp::default();

    let selection = match attributes.take_selection("http") {
        Some(selection) => selection,
        None => return Ok(http),
    };

    let (mut selection, _pos) = Loc::take_pair(selection);

    // Keep track of used variables.
    let mut args = arguments
        .iter()
        .map(|a| (a.ident(), a))
        .collect::<HashMap<_, _>>();

    if let Some(path) = selection.take("path") {
        http.path = Some(parse_path(diag, scope, path, &mut args)?);
    }

    if let Some(method) = selection.take("method") {
        http.method = Some(parse_method(diag, method)?);
    }

    if let Some(accept) = selection.take("accept") {
        let (accept, span) = Loc::take_pair(accept);

        let a = accept.as_string().with_span(diag, span)?;

        let accept = match a {
            "application/json" => core::RpAccept::Json,
            "text/plain" => core::RpAccept::Text,
            _ => {
                diag.err(span, "unsupported media type");
                return Err(());
            }
        };

        let accept = Loc::new(accept, span);
        http_verify_accept(diag, &accept, response)?;
        http.accept = Loc::take(accept);
    }

    // All arguments used, no request body.
    if args.is_empty() {
        *request = None;
    }

    // Assert that all arguments are used somehow.
    if !args.is_empty() {
        for arg in args.values() {
            if let Some(ref request) = *request {
                if arg.ident() == request.ident() {
                    continue;
                }
            }

            diag.err(
                Loc::span(&arg.ident),
                "Argument not used in #[http(...)] attribute",
            );
        }

        if diag.has_errors() {
            return Err(());
        }
    }

    check_selection!(diag, selection);
    return Ok(http);

    /// Parse a path specification.
    fn parse_path<'a, 'b: 'a, I>(
        diag: &mut Diagnostics,
        scope: &mut Scope<I>,
        path: Loc<RpValue>,
        args: &'a mut HashMap<&'b str, &'b RpEndpointArgument>,
    ) -> Result<RpPathSpec, ()>
    where
        I: Import,
    {
        let (path, span) = Loc::take_pair(path);

        let path = path.as_string().with_span(diag, span)?;

        let path = match path_parser::parse(path) {
            Ok(path) => path,
            Err(e) => {
                diag.err(span, format!("bad path: {}", e.display()));
                return Err(());
            }
        };

        let path = (span, args, path).into_model(diag, scope)?;
        Ok(path)
    }

    /// Parse a method.
    fn parse_method(diag: &mut Diagnostics, method: Loc<RpValue>) -> Result<RpHttpMethod, ()> {
        use core::RpHttpMethod::*;

        let (method, span) = Loc::take_pair(method);

        let m = match method.as_string().with_span(diag, &span)? {
            "GET" => Get,
            "POST" => Post,
            "PUT" => Put,
            "UPDATE" => Update,
            "DELETE" => Delete,
            "PATCH" => Patch,
            "HEAD" => Head,
            method => {
                diag.err(span, format!("no such method: {}", method));
                return Err(());
            }
        };

        Ok(m)
    }

    /// Check that accept matches response.
    fn http_verify_accept(
        diag: &mut Diagnostics,
        accept: &Loc<RpAccept>,
        response: Option<&Loc<RpChannel>>,
    ) -> Result<(), ()> {
        let response = match response {
            Some(response) => response,
            None => return Ok(()),
        };

        let (accept, span) = Loc::borrow_pair(&accept);

        match *accept {
            // Can handle complex data types.
            ref accept if *accept == core::RpAccept::Json => return Ok(()),
            _ => {
                if let core::RpType::String(..) = *response.ty() {
                    return Ok(());
                }

                diag.err(
                    Loc::span(response),
                    "only `string` responses are supported for the given `accept`",
                );

                diag.info(span, "Specified here");
                return Err(());
            }
        }
    }
}

/// `#[import(..)]` attributes
pub fn import(diag: &mut Diagnostics, attributes: &mut Attributes) -> Result<Vec<Loc<String>>, ()> {
    let mut out = Vec::new();

    if let Some(mut imports) = attributes.take_selection("import") {
        for import in imports.take_words() {
            let (import, span) = Loc::take_pair(import);
            let import = import.as_str().with_span(diag, span)?;
            out.push(Loc::new(import.to_string(), span));
        }

        check_selection!(diag, imports);
    }

    Ok(out)
}

pub enum StringFormat {
    DateTime,
    Bytes,
}

/// `#[format(..)]` attributes on string fields.
pub fn string_format(
    diag: &mut Diagnostics,
    attributes: &mut Attributes,
) -> Result<Option<Loc<StringFormat>>, ()> {
    let selection = match attributes.take_selection("format") {
        Some(selection) => selection,
        None => return Ok(None),
    };

    let (mut selection, attribute_span) = Loc::take_pair(selection);

    let format = match selection.take_word() {
        Some(format) => format,
        None => {
            diag.err(attribute_span, "expected argument");
            return Err(());
        }
    };

    let (format, span) = Loc::take_pair(format);

    let format = match format.into_string() {
        Ok(format) => format,
        Err(e) => {
            diag.err(span, e.display());
            return Err(());
        }
    };

    let format = match format.as_str() {
        "datetime" => StringFormat::DateTime,
        "bytes" => StringFormat::Bytes,
        _ => {
            diag.err(span, "unexpected format");
            diag.info(span, "HINT: expected one of `datetime` or `bytes`");
            return Err(());
        }
    };

    check_selection!(diag, selection);
    Ok(Some(Loc::new(format, attribute_span)))
}

/// `#[validate(pattern = "[a-z]+")]` attributes on string fields.
pub fn string_validate(
    diag: &mut Diagnostics,
    attributes: &mut Attributes,
) -> Result<RpStringValidate, ()> {
    let mut out = RpStringValidate::default();

    let mut validate = match attributes.take_selection("validate") {
        Some(validate) => validate,
        None => return Ok(out),
    };

    if let Some(pattern) = validate.take("pattern") {
        let (pattern, span) = Loc::take_pair(pattern);
        let pattern = pattern.as_string().with_span(diag, span)?;

        let regex = match regex_parser::parse(pattern) {
            Ok(regex) => regex,
            Err(e) => {
                diag.err(span, format!("bad regex: {}", e.display()));
                return Err(());
            }
        };

        out.pattern = Some(regex);
    }

    check_selection!(diag, validate);
    Ok(out)
}
