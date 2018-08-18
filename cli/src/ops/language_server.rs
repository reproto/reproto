//! Update action that synchronizes all repositories.

#[cfg(feature = "languageserver")]
extern crate reproto_languageserver as ls;

use clap::{App, Arg, ArgMatches, SubCommand};
use core::errors::Result;
use log;
use std::fs;
use std::io;

pub fn options<'a, 'b>() -> App<'a, 'b> {
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

        let level = if matches.is_present("debug") {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        };

        let log = match matches.value_of("log") {
            Some(log) => Some(fs::File::create(log)?),
            None => None,
        };

        ls::server(log, input, output, level)?;
        Ok(())
    }

    #[cfg(not(feature = "languageserver"))]
    {
        Err("languageserver feature is not enabled".into())
    }
}
