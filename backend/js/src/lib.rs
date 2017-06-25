extern crate reproto_backend;
#[macro_use]
extern crate codeviz;

#[macro_use]
mod utils;
mod models;
mod listeners;
mod js_backend;
mod js_compiler;
mod js_file_spec;
mod js_options;

pub(crate) use codeviz::js::*;
pub(crate) use reproto_backend::*;
pub(crate) use reproto_backend::errors::*;
pub(crate) use self::js_backend::*;
pub(crate) use self::js_compiler::*;
pub(crate) use self::js_file_spec::*;
pub(crate) use self::js_options::*;
pub(crate) use self::listeners::*;
pub(crate) use self::models::*;
pub(crate) use self::utils::*;

pub(crate) const TYPE: &str = "type";
pub(crate) const EXT: &str = "js";
pub(crate) const JS_CONTEXT: &str = "js";

fn setup_module(module: &str) -> Result<Box<Listeners>> {
    let _module: Box<Listeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn setup_listeners(modules: Vec<String>) -> Result<(JsOptions, Box<Listeners>)> {
    let mut listeners: Vec<Box<Listeners>> = Vec::new();

    for module in modules {
        listeners.push(setup_module(module.as_str())?);
    }

    let mut options = JsOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Compile for JavaScript")
}

pub fn verify_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    out.about("Verify for JavaScript")
}

pub fn compile(env: Environment,
               opts: Options,
               compiler_options: CompilerOptions,
               _matches: &ArgMatches)
               -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = JsBackend::new(env, options, listeners, id_converter);
    let compiler = backend.compiler(compiler_options)?;
    compiler.compile()
}

pub fn verify(env: Environment, opts: Options, _matches: &ArgMatches) -> Result<()> {
    let id_converter = opts.id_converter;
    let (options, listeners) = setup_listeners(opts.modules)?;
    let backend = JsBackend::new(env, options, listeners, id_converter);
    backend.verify()
}
