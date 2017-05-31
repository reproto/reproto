mod models;
pub mod builder;
pub mod constructor_properties;
pub mod fasterxml;
pub mod lombok;
pub mod mutable;
pub mod nullable;
pub mod processor;

use backend::*;
use backend::models as m;
use options::Options;

fn setup_module(module: &str) -> Result<Box<processor::Listeners>> {
    let module: Box<processor::Listeners> = match module {
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

pub fn resolve(options: Options, env: Environment) -> Result<processor::Processor> {
    let package_prefix = options.package_prefix
        .clone()
        .map(|prefix| m::Package::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    let mut listeners: Vec<Box<processor::Listeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = processor::ProcessorOptions::new(options);

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok(processor::Processor::new(options, env, package_prefix, Box::new(listeners)))
}
