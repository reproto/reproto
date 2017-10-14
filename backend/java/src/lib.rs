#[macro_use]
extern crate log;
#[macro_use]
extern crate genco;
extern crate reproto_backend as backend;
extern crate reproto_core as core;

mod builder;
mod constructor_properties;
mod fasterxml;
mod java_backend;
mod java_options;
mod listeners;
mod lombok;
mod java_field;
mod mutable;
mod nullable;

use self::backend::{App, ArgMatches, CompilerOptions, Environment, Options};
use self::backend::errors::*;
use self::java_backend::JavaBackend;
use self::java_options::JavaOptions;
use self::listeners::Listeners;

pub const JAVA_CONTEXT: &str = "java";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let module: Box<Listeners> = match module {
        "builder" => Box::new(builder::Module::new()),
        "constructor_properties" => Box::new(constructor_properties::Module::new()),
        "fasterxml" => Box::new(fasterxml::Module::new()),
        "lombok" => Box::new(lombok::Module::new()),
        "mutable" => Box::new(mutable::Module::new()),
        "nullable" => Box::new(nullable::Module::new()),
        _ => return Err(format!("No such module: {}", module).into()),
    };

    Ok(module)
}

pub fn setup_listeners(options: Options) -> Result<(JavaOptions, Box<Listeners>)> {
    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = JavaOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Compile for Java")
}

pub fn verify_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Verify for Java")
}

pub fn compile(
    env: Environment,
    options: Options,
    compiler_options: CompilerOptions,
    _matches: &ArgMatches,
) -> Result<()> {
    let (options, listeners) = setup_listeners(options)?;
    let backend = JavaBackend::new(env, options, listeners);
    backend.compile(&compiler_options.out_path)
}

pub fn verify(env: Environment, options: Options, _matches: &ArgMatches) -> Result<()> {
    let (options, listeners) = setup_listeners(options)?;
    let backend = JavaBackend::new(env, options, listeners);
    backend.verify()
}
