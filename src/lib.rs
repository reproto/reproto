#![recursion_limit = "1000"]

extern crate clap;

#[macro_use]
extern crate pest;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate codeviz;

pub mod ast;
pub mod backend;
pub mod commands;
pub mod errors;
pub mod logger;
pub mod naming;
pub mod options;
pub mod parser;
pub mod token;
