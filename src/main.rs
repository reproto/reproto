extern crate reproto;
#[macro_use]
extern crate log;
extern crate getopts;

use std::path::{Path, PathBuf};
use std::fs;
use std::env;

use reproto::errors::*;

use reproto::logger;
use reproto::backends;
use reproto::parser;
use reproto::options::Options;

/// List the given directory recursively and pass each visited path to the visitor clojure.
fn path_visitor<F>(path: &Path, mut visitor: F) -> Result<()>
    where F: FnMut(&Path) -> Result<()>
{
    let mut v: Vec<PathBuf> = Vec::new();

    v.push(path.to_owned());

    loop {
        if let Some(path) = v.pop() {
            let children = fs::read_dir(path.as_path())?;

            for child in children {
                let child = child?;
                let full = child.path();

                if full.is_dir() {
                    v.push(full.to_owned());
                } else {
                    visitor(&full)?;
                }
            }

            continue;
        }

        break;
    }

    Ok(())
}

fn setup_opts() -> getopts::Options {
    let mut opts = getopts::Options::new();

    opts.reqopt("b",
                "backend",
                "Backend to used to emit code (required)",
                "<backend>");

    opts.reqopt("o", "out", "Path to write output to (required)", "<dir>");

    opts.optflag("h", "help", "Print help");

    opts.optflag("", "debug", "Enable debug logging");

    opts.optflag("r",
                 "recursive",
                 "Process the arguments recursively (looking for .reproto files)");

    opts
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("hello: {}", opts.usage(&brief));
}

/// Configure logging
///
/// If debug (--debug) is specified, logging should be configured with LogLevelFilter::Debug.
fn setup_logger(matches: &getopts::Matches) -> Result<()> {
    let level: log::LogLevelFilter = match matches.opt_present("debug") {
        true => log::LogLevelFilter::Debug,
        false => log::LogLevelFilter::Info,
    };

    logger::init(level)?;

    Ok(())
}

fn entry() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let opts = setup_opts();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&args[0], opts);
            return Err(f.into());
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return Ok(());
    }

    if matches.free.len() < 1 {
        print_usage(&args[0], opts);
        return Ok(());
    }

    setup_logger(&matches)?;

    let backend = matches.opt_str("backend").ok_or("--backend <backend> is required")?;
    let mut backend = backends::resolve(&backend)?;

    let out_path = matches.opt_str("out").ok_or("--out <dir> is required")?;
    let out_path = Path::new(&out_path);

    let options = Options { out_path: out_path.to_path_buf() };

    fs::create_dir_all(&out_path)?;

    for argument in matches.free {
        path_visitor(Path::new(argument.as_str()), |path| {
            backend.add_file(parser::parse_file(&path)
                .chain_err(|| format!("failed to parse: {}", path.display()))?)
        })?;
    }

    backend.process(&options)?;
    Ok(())
}

fn main() {
    match entry() {
        Err(e) => {
            error!("{}", e);

            for e in e.iter().skip(1) {
                error!("  caused by: {}", e);
            }

            if let Some(backtrace) = e.backtrace() {
                error!("  backtrace: {:?}", backtrace);
            }

            ::std::process::exit(1);
        }
        _ => {}
    };

    ::std::process::exit(0);
}
