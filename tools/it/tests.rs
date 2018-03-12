use Language::*;

define!{
    allstructures => {
        allstructures.arg(Rust, &["-m", "chrono"]);
    },
    alltypes => {
        alltypes.arg(Rust, &["-m", "chrono"]);
    },
    basic => {
    },
    code => {
    },
    enum_ => {
    },
    inner => {
    },
    interfaces => {
    },
    java_grpc => {
        java_grpc.include(Java);
    },
    java_keywords => {
        java_keywords.include(Java);
    },
    java_okhttp1 => {
        java_okhttp1.include(Java);
    },
    java_okhttp2 => {
        java_okhttp2.include(Java);
    },
    js_keywords => {
        js_keywords.include(JavaScript);
    },
    python_keywords => {
        python_keywords.include(Python);
    },
    python_requests => {
        python_requests.include(Python);
    },
    service => {
        service.package("service");
        service.arg(Java, &["-m", "grpc"]);
        service.include(Java);
        service.include(Rust);
    },
    rust_keywords => {
        rust_keywords.include(Rust);
    },
    tuple => {},
    versions => {},
}
