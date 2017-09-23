#![recursion_limit = "1000"]
#[macro_use]
extern crate log;
extern crate codeviz_common;
extern crate reproto_backend;
extern crate pulldown_cmark;

#[macro_use]
mod macros;
mod doc_backend;
mod doc_builder;
mod doc_collector;
mod doc_compiler;
mod doc_listeners;
mod doc_options;
mod doc_writer;
mod escape;
mod imports;

use self::imports::*;

fn setup_module(module: &str) -> Result<Box<DocListeners>> {
    let _module: Box<DocListeners> = match module {
        _ => return Err(format!("No such module: {}", module).into()),
    };
}

pub fn setup_listeners(options: Options) -> Result<(DocOptions, Box<DocListeners>)> {
    let mut listeners: Vec<Box<DocListeners>> = Vec::new();

    for module in &options.modules {
        listeners.push(setup_module(module)?);
    }

    let mut options = DocOptions::new();

    for listener in &listeners {
        listener.configure(&mut options)?;
    }

    Ok((options, Box::new(listeners)))
}

pub fn shared_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    let out = out.arg(
        Arg::with_name("theme")
            .long("theme")
            .takes_value(true)
            .help("Theme to use"),
    );

    let out = out.arg(Arg::with_name("skip_static").long("skip-static").help(
        "Skip building \
         with static \
         files",
    ));

    out
}

pub fn compile_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    shared_options(out).about("Compile Documentation")
}

pub fn verify_options<'a, 'b>(out: App<'a, 'b>) -> App<'a, 'b> {
    shared_options(out).about("Verify for Documentation")
}

pub fn compile(
    env: Environment,
    options: Options,
    compiler_options: CompilerOptions,
    matches: &ArgMatches,
) -> Result<()> {
    let theme = matches
        .value_of("theme")
        .unwrap_or(DEFAULT_THEME)
        .to_owned();
    let skip_static = matches.is_present("skip_static");

    let (options, listeners) = setup_listeners(options)?;
    let backend = DocBackend::new(env, options, listeners, theme);
    let compiler = DocCompiler::new(&backend, compiler_options.out_path, skip_static);
    compiler.compile()
}

pub fn verify(env: Environment, options: Options, _matches: &ArgMatches) -> Result<()> {
    let theme = String::from("light");
    let (options, listeners) = setup_listeners(options)?;
    let backend = DocBackend::new(env, options, listeners, theme);
    backend.verify()
}
