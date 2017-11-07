//! Command to print the local manifest.

use super::MANIFEST_NAME;
use manifest::{Manifest, read_manifest};
use ops::imports::*;
use std::env;
use std::fs::File;

pub fn options<'a, 'b>() -> App<'a, 'b> {
    let out = SubCommand::with_name("manifest").about("Dump manifest configuration");
    out
}

pub fn entry(_matches: &ArgMatches) -> Result<()> {
    let mut manifest = Manifest::default();
    let path = env::current_dir()?.join(MANIFEST_NAME);
    let reader = File::open(path.clone())?;
    read_manifest(&mut manifest, path, reader)?;
    println!("{:?}", manifest);
    Ok(())
}
