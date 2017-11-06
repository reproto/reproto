//! Command to print the local manifest.

use manifest::{Manifest, read_manifest};
use ops::imports::*;
use std::env;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("manifest");
    let out = compiler_base(out).about("Dump manifest configuration");
    out
}

pub fn entry(_matches: &ArgMatches) -> Result<()> {
    let mut manifest = Manifest::new();
    let path = env::current_dir()?.join("reproto.toml");
    read_manifest(&mut manifest, path)?;
    println!("{:?}", manifest);
    Ok(())
}
