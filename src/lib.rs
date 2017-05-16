#![feature(proc_macro)]
#![recursion_limit = "1000"]

extern crate getopts;

#[macro_use]
extern crate pest;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate codegen;

pub mod backend;
pub mod backends;
pub mod errors;
pub mod logger;
pub mod parser;
pub mod options;
pub mod environment;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
