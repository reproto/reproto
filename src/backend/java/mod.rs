pub mod processor;
pub mod fasterxml;
pub mod constructor_properties;

use environment::Environment;
use options::Options;
use parser::ast;

use errors::*;

fn setup_module(module: &str) -> Result<Box<processor::Listeners>> {
    let module: Box<processor::Listeners> = match module {
        "fasterxml" => Box::new(fasterxml::Module::new()),
        "constructor_properties" => Box::new(constructor_properties::Module::new()),
        _ => return Err(format!("No such module: {}", module).into()),
    };

    Ok(module)
}

pub fn resolve(options: Options, env: Environment) -> Result<processor::Processor> {
    let package_prefix = options.package_prefix
        .clone()
        .map(|prefix| ast::Package::new(prefix.split(".").map(ToOwned::to_owned).collect()));

    let mut listeners: Vec<Box<processor::Listeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    Ok(processor::Processor::new(options, env, package_prefix, Box::new(listeners)))
}
