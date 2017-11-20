# reproto
[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg)](https://crates.io/crates/reproto)

This is the home of `reproto`, a system for managing JSON schemas.

`reproto` has the following components:

* A custom [interface description language], known as `reproto` that permits describing the schema
  of JSON and bidirectional rpc services.
* A compiler for the `reproto` language, which generates native data structures in various
  languages.
* A [semantic version checker] which verifies that modifications to schemas do not violate
  [semantic versioning].
* A build system and package manager.
* A rich [documentation generator][#generating-documentation].

## Getting Started

* See the [specification][spec] for details on how the reproto language and its build manifest
  works.
* See [examples] for some example protocol specifications.
* See [config] for information on how to configure the system.
* See the [integration tests][it] for some examples of how protocol specifications can be used.
* See the [TODO][todo] for a list of things that still needs to be done.

**Note:** This project is in an early stage. Things will change a lot. Please take it for a spin,
but avoid building large repositories of specifications right now.

## Language Support

* Java (`java`)
  * Data models using [fasterxml jackson] (`fasterxml`), and/or [lombok] (`lombok`).
  * [gRPC][grpc] services through the `grpc` module.
* JavaScript (`js`)
  * ES2015 classes, that can be transpiled using babel for older targets, see the
    [integration test][js-it].
* Python (`python`)
  * Plain-python classes, compatible with 2 and 3 for binding data efficiently.
* Rust (`rust`)
  * [Serde]-based serialization for data structures.
  * `datetime` support through the [`chrono`] crate.

## Generating Documentation

`reproto` can generate rich markdown-based documentation from your specifications.

See <https://reproto.github.io/reproto/doc-examples/> for examples on what this can look like.

These have been generated from the [examples/reproto.toml] manifest using
[tools/update-doc-examples].

## Building

Make sure you have [gotten started with Rust][rust-get-started].

Initialize submodules:

```bash
$ git submodule update --init
```

Pack syntax highlighting and themes:

```bash
$ make dumps
```

Build and install the CLI.
This will install `reproto` into `~/.cargo/bin`, make sure it is in your PATH:

```bash
$ cargo install --path $PWD/cli reproto
```

## [Maven Plugin][maven-plugin]

A Maven plugin that integrates reproto into the build lifecycle of a maven project.

[maven-plugin]: https://github.com/reproto/reproto-maven-plugin

## [VIM Plugin][vim]

A VIM plugin that provides syntax highlighting.

[vim]: https://github.com/reproto/reproto-vim

## Testing

This project includes an extensive set of integration tests.

See `make help` for documentation on what can be done.

Suites are tests which compiled a given set of rules, and compares with expected output.

Projects are complete project tests.
These are projects written for various programming languages, and are generally harder to build.

The tool [`check-project-deps`](tools/check-project-deps) is used to determine
which projects your local system can build.

To run all tests, do:

```bash
$> make clean all
```

[rust-get-started]: https://doc.rust-lang.org/book/getting-started.html
[it]: /it
[Cargo]: https://github.com/rust-lang/cargo
[config]: /doc/config.md
[examples]: /examples
[examples/reproto.toml]: /examples/reproto.toml
[tools/update-doc-examples]: /tools/update-doc-examples
[grpc]: https://grpc.io
[idl]: #the-idl
[it]: /it
[fasterxml jackson]: https://github.com/FasterXML/jackson-databind
[js-it]: /it/js
[lombok]: https://projectlombok.org/
[semantic versioning]: https://semver.org
[semantic version checker]: /semck
[spec]: /doc/spec.md
[interface description language]: /doc/spec.md
[todo]: /doc/todo.md
[Serde]: https://serde.rs
[`chrono`]: https://crates.io/crates/chrono
