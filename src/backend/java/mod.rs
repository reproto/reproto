mod models;
mod builder;
mod constructor_properties;
mod fasterxml;
mod java_backend;
mod java_options;
mod java_compiler;
mod listeners;
mod lombok;
mod mutable;
mod nullable;

use backend::*;
pub(crate) use codeviz::java::*;
pub(crate) use errors::*;
use options::Options;
use self::java_backend::*;
use self::java_compiler::*;
use self::java_options::*;
use self::listeners::*;
pub(crate) use self::models::*;

pub const JAVA_CONTEXT: &str = "java";

fn setup_module(module: &str) -> Result<Box<listeners::Listeners>> {
    let module: Box<listeners::Listeners> = match module {
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

pub fn resolve(options: Options, env: Environment) -> Result<JavaBackend> {
    let package_prefix = options.package_prefix
        .clone()
        .map(|prefix| RpPackage::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    let mut listeners: Vec<Box<listeners::Listeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = JavaOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok(JavaBackend::new(options, env, package_prefix, Box::new(listeners)))
}
