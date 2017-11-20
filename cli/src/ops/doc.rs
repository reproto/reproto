//! Action to build documentation.

use super::{setup_compiler_options, setup_environment};
use super::imports::*;
use manifest::{Lang, Manifest};

pub fn options<'a, 'b>() -> App<'a, 'b> {
    ::doc::shared_options(SubCommand::with_name("doc").about("Generate documentation"))
}

pub fn entry(matches: &ArgMatches) -> Result<()> {
    let preamble = manifest_preamble(matches)?;

    return do_manifest_use!(matches, preamble, inner);

    fn inner<L>(matches: &ArgMatches, manifest: Manifest<L>) -> Result<()>
    where
        L: Lang,
    {
        let env = setup_environment(&manifest)?;
        let options = setup_options(&manifest)?;
        let compiler_options = setup_compiler_options(&manifest, matches)?;

        ::doc::compile(env, options, compiler_options, matches, manifest).map_err(Into::into)
    }
}
