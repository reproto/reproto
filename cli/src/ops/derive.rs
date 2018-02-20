//! Derive a schema from the given input.

use clap::{App, Arg, ArgMatches, SubCommand};
use compile;
use core::{Context, Object, PathObject, RpPackage, StdinObject};
use core::errors::Result;
use derive;
use genco::IoFmt;
use java;
use js;
use json;
use manifest::{Lang, TryFromToml};
use python;
use reproto;
use rust;
use std::io;
use std::path::Path;
use std::rc::Rc;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("derive").about("Derive a schema from the given input");

    let out = out.arg(
        Arg::with_name("file")
            .long("file")
            .short("i")
            .takes_value(true)
            .help("File to read from, otherwise will read from stdin"),
    );

    let out = out.arg(
        Arg::with_name("root-name")
            .long("root-name")
            .takes_value(true)
            .help("Name of the root object to generate"),
    );

    let out = out.arg(
        Arg::with_name("package-prefix")
            .long("package-prefix")
            .takes_value(true)
            .help("Package prefix to use"),
    );

    let out = out.arg(
        Arg::with_name("format")
            .long("format")
            .short("F")
            .takes_value(true)
            .help("Format to decode, valid values: json, yaml"),
    );

    let out = out.arg(
        Arg::with_name("lang")
            .long("lang")
            .takes_value(true)
            .help("Language to compile to"),
    );

    let out = out.arg(
        Arg::with_name("module")
            .long("module")
            .short("m")
            .takes_value(true)
            .help("Modules to enable"),
    );

    out
}

pub fn entry(_ctx: Rc<Context>, matches: &ArgMatches) -> Result<()> {
    let root_name = match matches.value_of("root-name") {
        None => "Generated".to_string(),
        Some(name) => name.to_string(),
    };

    let package_prefix = match matches.value_of("package-prefix") {
        None => RpPackage::parse("io.github.reproto"),
        Some(name) => RpPackage::parse(name),
    };

    let format: Box<derive::Format> = match matches.value_of("format") {
        None | Some("json") => Box::new(derive::Json),
        Some("yaml") => Box::new(derive::Yaml),
        Some(value) => return Err(format!("Unsupported format: {}", value).into()),
    };

    let object: Box<Object> = match matches.value_of("file") {
        Some(file) => Box::new(PathObject::new(None, Path::new(file))),
        None => Box::new(StdinObject::new()),
    };

    let derive = derive::Derive::new(root_name, format, Some(package_prefix.clone()));

    let decl = derive::derive(derive, object.as_ref())?;

    let stdout = io::stdout();

    let simple_compile = compile::SimpleCompile {
        decl: decl,
        package_prefix: Some(package_prefix),
    };

    let modules: Vec<String> = matches
        .values_of("module")
        .into_iter()
        .flat_map(|s| s.map(|s| s.to_string()))
        .collect();

    match matches.value_of("lang") {
        Some("reproto") => {
            compile::simple_compile::<reproto::ReprotoLang, _>(
                &mut IoFmt(&mut stdout.lock()),
                simple_compile,
                load_modules::<reproto::ReprotoLang>(modules)?,
                reproto::compile,
            )?;
        }
        Some("java") => {
            compile::simple_compile::<java::JavaLang, _>(
                &mut IoFmt(&mut stdout.lock()),
                simple_compile,
                load_modules::<java::JavaLang>(modules)?,
                java::compile,
            )?;
        }
        Some("python") => {
            compile::simple_compile::<python::PythonLang, _>(
                &mut IoFmt(&mut stdout.lock()),
                simple_compile,
                load_modules::<python::PythonLang>(modules)?,
                python::compile,
            )?;
        }
        Some("js") => {
            compile::simple_compile::<js::JsLang, _>(
                &mut IoFmt(&mut stdout.lock()),
                simple_compile,
                load_modules::<js::JsLang>(modules)?,
                js::compile,
            )?;
        }
        Some("rust") => {
            compile::simple_compile::<rust::RustLang, _>(
                &mut IoFmt(&mut stdout.lock()),
                simple_compile,
                load_modules::<rust::RustLang>(modules)?,
                rust::compile,
            )?;
        }
        Some("json") => {
            compile::simple_compile::<json::JsonLang, _>(
                &mut IoFmt(&mut stdout.lock()),
                simple_compile,
                load_modules::<json::JsonLang>(modules)?,
                json::compile,
            )?;
        }
        Some(lang) => return Err(format!("Unsupported language: {}", lang).into()),
        None => return Err("Language not specified".into()),
    }

    return Ok(());

    fn load_modules<L: Lang>(names: Vec<String>) -> Result<Vec<L::Module>> {
        let mut modules = Vec::new();

        for name in names {
            modules.push(L::Module::try_from_string(
                Path::new("."),
                name.as_str(),
                name.to_string(),
            )?);
        }

        Ok(modules)
    }
}
