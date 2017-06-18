use std::io;
use toml;

error_chain! {
    foreign_links {
        Io(io::Error);
        TomlDe(toml::de::Error);
    }

    errors {
    }
}
