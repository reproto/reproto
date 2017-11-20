pub(crate) use super::{Match, manifest_compile, manifest_preamble, semck_check, setup_matches,
                       setup_options, setup_path_resolver, setup_publish_matches, setup_repository};
pub(crate) use backend::{CompilerOptions, Environment, Options};
pub(crate) use clap::{App, Arg, ArgMatches, SubCommand};
pub(crate) use core::RpRequiredPackage;
pub(crate) use errors::*;
