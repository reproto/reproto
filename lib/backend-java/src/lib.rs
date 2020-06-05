mod codegen;
mod compiler;
mod flavored;
mod module;
mod options;

use crate::compiler::Compiler;
use crate::options::Options;
use core::errors::Result;
use core::{CoreFlavor, Handle};
use manifest::{checked_modules, Lang, Manifest, NoModule, TryFromToml};
use naming::Naming;
use std::any::Any;
use std::path::Path;
use std::rc::Rc;
use trans::Session;

#[derive(Clone, Copy, Default, Debug)]
pub struct JavaLang;

impl Lang for JavaLang {
    manifest::lang_base!(Module, compile);

    fn comment(&self, input: &str) -> Option<String> {
        Some(format!("// {}", input))
    }

    fn field_ident_naming(&self) -> Option<Box<dyn Naming>> {
        Some(Box::new(naming::to_lower_camel()))
    }

    fn endpoint_ident_naming(&self) -> Option<Box<dyn Naming>> {
        Some(Box::new(naming::to_lower_camel()))
    }

    fn keywords(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("abstract", "_abstract"),
            ("assert", "_assert"),
            ("boolean", "_boolean"),
            ("break", "_break"),
            ("byte", "_byte"),
            ("case", "_case"),
            ("catch", "_catch"),
            ("char", "_char"),
            ("class", "_class"),
            ("const", "_const"),
            ("continue", "_continue"),
            ("default", "_default"),
            ("do", "_do"),
            ("double", "_double"),
            ("else", "_else"),
            ("enum", "_enum"),
            ("extends", "_extends"),
            ("false", "_false"),
            ("final", "_final"),
            ("finally", "_finally"),
            ("float", "_float"),
            ("for", "_for"),
            ("goto", "_goto"),
            ("if", "_if"),
            ("implements", "_implements"),
            ("import", "_import"),
            ("instanceof", "_instanceof"),
            ("int", "_int"),
            ("interface", "_interface"),
            ("long", "_long"),
            ("native", "_native"),
            ("new", "_new"),
            ("null", "_null"),
            ("package", "_package"),
            ("private", "_private"),
            ("protected", "_protected"),
            ("public", "_public"),
            ("return", "_return"),
            ("short", "_short"),
            ("static", "_static"),
            ("strictfp", "_strictfp"),
            ("super", "_super"),
            ("switch", "_switch"),
            ("synchronized", "_synchronized"),
            ("this", "_this"),
            ("throw", "_throw"),
            ("throws", "_throws"),
            ("transient", "_transient"),
            ("true", "_true"),
            ("try", "_try"),
            ("void", "_void"),
            ("volatile", "_volatile"),
            ("while", "_while"),
        ]
    }
}

#[derive(Debug)]
pub enum Module {
    Jackson,
    Lombok,
    //Grpc,
    Builder,
    ConstructorProperties,
    Mutable,
    Nullable,
    //OkHttp(module::OkHttpConfig),
}

impl TryFromToml for Module {
    fn try_from_string(path: &Path, id: &str, value: String) -> Result<Self> {
        let result = match id {
            "jackson" => Self::Jackson,
            "lombok" => Self::Lombok,
            //"grpc" => Grpc,
            "builder" => Self::Builder,
            "constructor_properties" => Self::ConstructorProperties,
            "mutable" => Self::Mutable,
            "nullable" => Self::Nullable,
            //"okhttp" => OkHttp(module::OkHttpConfig::default()),*/
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }

    fn try_from_value(path: &Path, id: &str, value: toml::Value) -> Result<Self> {
        let result = match id {
            "jackson" => Self::Jackson,
            "lombok" => Self::Lombok,
            //"grpc" => Grpc,
            "builder" => Self::Builder,
            "constructor_properties" => Self::ConstructorProperties,
            "mutable" => Self::Mutable,
            "nullable" => Self::Nullable,
            //"okhttp" => OkHttp(value.try_into()?),
            _ => return NoModule::illegal(path, id, value),
        };

        Ok(result)
    }
}

fn setup_options(modules: Vec<Module>) -> Result<Options> {
    let mut options = Options::new();

    for module in modules {
        match module {
            Module::Jackson => module::Jackson.initialize(&mut options),
            Module::Lombok => module::Lombok.initialize(&mut options),
            //Grpc => module::Grpc.initialize(c),
            Module::Builder => module::Builder.initialize(&mut options),
            Module::ConstructorProperties => module::ConstructorProperties.initialize(&mut options),
            Module::Mutable => module::Mutable.initialize(&mut options),
            Module::Nullable => module::Nullable.initialize(&mut options),
            //OkHttp(config) => {
            //let serialization = c.options.get_serialization()?;
            //module::OkHttp::new(config).initialize(c, serialization);
            //}
        };
    }

    Ok(options)
}

fn compile(handle: &dyn Handle, session: Session<CoreFlavor>, manifest: Manifest) -> Result<()> {
    let packages = session.packages()?;
    let session = session.translate(flavored::JavaFlavorTranslator::new(packages.clone()))?;

    let session = Rc::new(session);
    let modules = checked_modules(manifest.modules)?;
    let options = setup_options(modules)?;

    let compiler = Compiler::new(&session, options);

    compiler.compile(handle)
}
