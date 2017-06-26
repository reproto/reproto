use serde_json;
use std::io;
use toml;

error_chain! {
    foreign_links {
        Io(io::Error);
        TomlDe(toml::de::Error);
        SerdeJson(serde_json::Error);
        UrlParseError(::url::ParseError);
        OpenSSL(::openssl::error::ErrorStack);
        Git2(::git2::Error);
    }

    errors {
    }
}
