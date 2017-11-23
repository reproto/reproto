#[macro_use]
extern crate log;
#[macro_use]
extern crate genco;
#[macro_use]
extern crate reproto_backend as backend;
#[allow(unused)]
#[macro_use]
extern crate serde_derive;
extern crate reproto_core as core;
extern crate reproto_manifest as manifest;
extern crate toml;
extern crate serde;

mod builder;
mod constructor_properties;
mod jackson;
mod java_backend;
mod java_options;
mod listeners;
mod lombok;
mod java_field;
mod mutable;
mod nullable;
mod grpc;

use self::ErrorKind::*;
use backend::{ArgMatches, Environment};
use backend::errors::*;
use core::Context;
use java_backend::JavaBackend;
use java_options::JavaOptions;
use listeners::Listeners;
use manifest::{Lang, Manifest, NoModule, TryFromToml, self as m};
use std::path::Path;
use std::rc::Rc;

pub const JAVA_CONTEXT: &str = "java";

#[derive(Default)]
pub struct JavaLang;

impl Lang for JavaLang {
    type Module = JavaModule;
}

#[derive(Debug)]
pub enum JavaModule {
    Jackson,
    Lombok,
    Grpc,
    Builder,
    ConstructorProperties,
    Mutable,
    Nullable,
}

impl TryFromToml for JavaModule {
    fn try_from_string(path: &Path, id: &str, value: String) -> m::errors::Result<Self> {
        use self::JavaModule::*;

        let result = match id {
            "jackson" => Jackson,
            "lombok" => Lombok,
            "grpc" => Grpc,
            "builder" => Builder,
            "constructor_properties" => ConstructorProperties,
            "mutable" => Mutable,
            "nullable" => Nullable,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> m::errors::Result<Self> {
        use self::JavaModule::*;

        let result = match id {
            "jackson" => Jackson,
            "lombok" => Lombok,
            "grpc" => Grpc,
            "builder" => Builder,
            "constructor_properties" => ConstructorProperties,
            "mutable" => Mutable,
            "nullable" => Nullable,
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

fn setup_listeners(modules: &[JavaModule]) -> Result<(JavaOptions, Box<Listeners>)> {
    use self::JavaModule::*;

    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        let listener = match *module {
            Jackson => Box::new(jackson::Module::new()) as Box<Listeners>,
            Lombok => Box::new(lombok::Module::new()) as Box<Listeners>,
            Grpc => Box::new(grpc::Module::new()) as Box<Listeners>,
            Builder => Box::new(builder::Module::new()) as Box<Listeners>,
            ConstructorProperties => {
                Box::new(constructor_properties::Module::new()) as Box<Listeners>
            }
            Mutable => Box::new(mutable::Module::new()) as Box<Listeners>,
            Nullable => Box::new(nullable::Module::new()) as Box<Listeners>,
        };

        listeners.push(listener);
    }

    let mut options = JavaOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile(
    _ctx: Rc<Context>,
    env: Environment,
    _matches: &ArgMatches,
    manifest: Manifest<JavaLang>,
) -> Result<()> {
    let out = manifest.output.ok_or(MissingOutput)?;
    let (options, listeners) = setup_listeners(&manifest.modules)?;
    let backend = JavaBackend::new(env, options, listeners);
    backend.compile(&out)
}
