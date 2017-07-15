use reproto_core::errors as core;
use reproto_repository::errors as repository;

error_chain!{
    links {
        Repository(repository::Error, repository::ErrorKind);
        Core(core::Error, core::ErrorKind);
    }

    foreign_links {
        Log(::log::SetLoggerError);
        IoError(::std::io::Error);
        AddParseError(::std::net::AddrParseError);
        Hyper(::hyper::Error);
    }

    errors {
        PoisonError {
            description("posion error")
        }

        BadRequest(message: &'static str) {
            description("bad request")
            display("bad request: {}", message)
        }
    }
}
