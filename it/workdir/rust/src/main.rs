extern crate rust;
extern crate serde_json as json;

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();

    for line in input.lines() {
        let line = line.unwrap();
        let entry: rust::generated::test::Entry = json::from_str(&line).unwrap();
        println!("{:?}", entry);
        println!("{}", json::to_string(&entry).unwrap());
    }
}
