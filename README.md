# reproto
[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg)](https://crates.io/crates/reproto)

This is the home of `reproto`, a system for managing JSON schemas.

`reproto` is the following things:

* A custom [interface description language] that permits describing the schema of JSON and
  bidirectional rpc services (like [gRPC]).
* A compiler which generates code for [various languages].
* A [semantic version checker] which verifies that modifications to schemas do not violate
  [semantic versioning].
* A build system with a package manager and a `reproto.toml` [build manifest].
* A rich, markdown-based [documentation generator].
* (eventually) A [central repository] of usable schemas.

These things combined support an ecosystem where schemas can be maintained and shared across
projects.

**Note:** This project is in an early stage. Things will change a lot. Please take it for a spin,
but avoid building large repositories of specifications right now.

[interface description language]: /doc/spec.md
[various languages]: #language-support
[semantic version checker]: /doc/semck.md
[semantic versioning]: https://semver.org
[documentation generator]: #generating-documentation
[central repository]: https://github.com/reproto/reproto-index
[build manifest]: /doc/manifest.md

## Getting Started

* See the [documentation] for an overview of how the reproto language and its build manifest works.
* See [examples] for some example specifications.
* See [config] for information on how to configure the system.
* See the [integration tests] for even more examples on how protocol specifications can be used.
* See the [TODO][todo] for a list of things that still needs to be done.

[documentation]: /doc/
[integration tests]: /it
[examples]: /examples
[config]: /doc/config.md
[todo]: /doc/todo.md

## Language Support

* Java (`java`)
  * Data models using [jackson] (`jackson`), and/or [lombok] (`lombok`).
  * [gRPC] services through the `grpc` module.
* Python (`python`)
  * Plain-python classes, compatible with 2 and 3 for binding data efficiently.
* Rust (`rust`)
  * [Serde]-based serialization for data structures.
  * `datetime` support through the [`chrono`] crate.
* JavaScript (`js`)
  * ES2015 classes, that can be transpiled using babel for older targets, see the
    [js integration test].

[gRPC]: https://grpc.io
[lombok]: https://projectlombok.org/
[Serde]: https://serde.rs
[jackson]: https://github.com/FasterXML/jackson-databind
[`chrono`]: https://crates.io/crates/chrono
[js integration test]: /it/workdir/js

## Generating Documentation

`reproto` can generate rich markdown-based documentation from your specifications.

Go to <https://reproto.github.io/reproto/doc-examples/> to see what this documentation looks like.

These have been generated from the [examples project] using [tools/update-doc-examples].

[examples project]: /examples/
[tools/update-doc-examples]: /tools/update-doc-examples

## Building and Installing

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

[rust-get-started]: https://rustup.rs

## [Maven Plugin]

A Maven plugin that integrates reproto into the build lifecycle of a maven project.

[Maven Plugin]: https://github.com/reproto/reproto-maven-plugin

## [VIM Plugin]

A VIM plugin that provides syntax highlighting.

[VIM Plugin]: https://github.com/reproto/reproto-vim

## Testing

This project includes an extensive set of integration tests.

See `make help` for documentation on what can be done.

Suites are tests which compiled a given set of rules, and compares with expected output.

Projects are complete project tests.
These are projects written for various programming languages, and are generally harder to build.

The tool [`check-project-deps`] is used to determine
which projects your local system can build.

To run all tests, do:

```bash
$> make clean all
```

[`check-project-deps`]: /tools/check-project-deps
