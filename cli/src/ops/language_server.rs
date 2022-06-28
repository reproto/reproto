//! Update action that synchronizes all repositories.

use clap::{App, Arg, ArgMatches, SubCommand};
use log;
use reproto_core::errors::Result;
use std::fs;
use std::io;

pub fn options<'a>() -> App<'a> {
    let out = SubCommand::with_name("language-server").about("Run the language server for reproto");

    let out = out.arg(
        Arg::with_name("lang")
            .long("lang")
            .takes_value(true)
            .help("Language to build for"),
    );

    let out = out.arg(
        Arg::with_name("log")
            .long("log")
            .takes_value(true)
            .help("Log to the given path"),
    );

    out
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    #[cfg(feature = "languageserver")]
    {
        let input = io::stdin();
        let output = io::stdout();

        let level = if matches.try_contains_id("debug").unwrap_or_default() {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        };

        let log = match matches.try_get_one::<String>("log") {
            Ok(Some(log)) => Some(fs::File::create(log)?),
            _ => None,
        };

        languageserver::server(log, input, output, level)?;
        Ok(())
    }

    #[cfg(not(feature = "languageserver"))]
    {
        Err("languageserver feature is not enabled".into())
    }
}
