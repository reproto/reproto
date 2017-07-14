use hyper;
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
        FromHexError(::hex::FromHexError);
        HyperUriError(hyper::error::UriError);
        HyperError(hyper::Error);
    }

    errors {
        NoPublishIndex(url: String) {
            description("index does not support publishing")
            display("index does not support publishing: {}", url)
        }

        NoPublishObjects(url: String) {
            description("object storage does not support publishing")
            display("object storage does not support publishing: {}", url)
        }

        PoisonError {
            description("mutex poisoned")
        }
    }
}
