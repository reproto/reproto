extern crate lalrpop;

fn main() {
    let config = lalrpop::Configuration::new();
    config.process_current_dir().unwrap();
}
