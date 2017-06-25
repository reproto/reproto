extern crate reproto_backend;
extern crate serde_json;

mod collector;
mod json_backend;
mod json_compiler;
mod json_options;
mod listeners;

pub(crate) use reproto_backend::*;
pub(crate) use reproto_backend::errors::*;
pub(crate) use self::collector::*;
pub(crate) use self::json_backend::*;
pub(crate) use self::json_compiler::*;
pub(crate) use self::json_options::*;
pub(crate) use self::listeners::*;

pub(crate) const EXT: &str = "json";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn setup_listeners(modules: Vec<String>) -> Result<(JsonOptions, Box<Listeners>)> {
    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        listeners.push(setup_module(module.as_str())?);
    }

    let mut options = JsonOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Compile to JSON")
}

pub fn verify_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Verify for JSON")
}

pub fn compile(env: Environment,
               opts: Options,
               compiler_options: CompilerOptions,
               _matches: &ArgMatches)
               -> Result<()> {
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = JsonBackend::new(env, options, listeners);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}

pub fn verify(env: Environment, opts: Options, _matches: &ArgMatches) -> Result<()> {
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = JsonBackend::new(env, options, listeners);
    backend.verify()
}
