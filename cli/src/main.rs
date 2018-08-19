extern crate atty;
extern crate clap;
extern crate reproto;
extern crate reproto_core as core;

use clap::{App, Arg, ArgMatches};
use core::errors::Result;
use core::RealFilesystem;
use reproto::{ops, output, VERSION};
use std::io;

fn setup_opts<'a, 'b>() -> App<'a, 'b> {
    App::new("reproto")
        .version(VERSION)
        .arg(
            Arg::with_name("color")
                .long("color")
                .help("Force colored output."),
        ).arg(
            Arg::with_name("no-color")
                .long("no-color")
                .help("Disable colored output."),
        ).arg(
            Arg::with_name("output-format")
                .long("output-format")
                .takes_value(true)
                .help("Select a different output format (json, human) (default: human)."),
        )
}

fn entry(matches: &ArgMatches, output: &output::Output) -> Result<()> {
    let fs = RealFilesystem::new();
    let mut reporter = Vec::new();

    match ops::entry(&fs, &mut reporter, matches, output) {
        Err(error) => {
            output.handle_error(&error, None)?;
            output.handle_context(&reporter)?;
            ::std::process::exit(1);
        }
        Ok(()) => {
            output.handle_context(&reporter)?;
        }
    }

    Ok(())
}

fn main() {
    let opts = setup_opts();
    let opts = ops::options(opts);
    let matches = opts.get_matches();

    let colored = matches.is_present("color")
        || !matches.is_present("no-color") && atty::is(atty::Stream::Stdout);

    let output_format = match matches.value_of("output-format") {
        Some("json") => output::OutputFormat::Json,
        _ => output::OutputFormat::Human,
    };

    let mut output: Box<output::Output> = match output_format {
        output::OutputFormat::Json => Box::new(output::Json::new(io::stdout())),
        _ if colored => Box::new(output::Colored::new(io::stdout())),
        _ => Box::new(output::NonColored::new(io::stdout())),
    };

    if let Err(error) = entry(&matches, output.as_mut()) {
        if let Err(report_error) = output.handle_error(&error, None) {
            eprintln!("Failed to report error: {}", report_error.display());
            eprintln!("Original error: {}", error.display());

            if let Some(bt) = error.backtrace() {
                eprintln!("Backtrace: {:?}", bt);
            }
        }

        ::std::process::exit(1);
    }

    ::std::process::exit(0);
}
