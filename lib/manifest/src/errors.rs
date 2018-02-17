use core::errors as core;

error_chain! {
    links {
        Core(core::Error, core::ErrorKind);
    }

    foreign_links {
        IO(::std::io::Error);
        TomlDe(::toml::de::Error);
    }

    errors {
    }
}
