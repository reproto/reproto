# ReProto Compiler 
[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg)](https://crates.io/crates/reproto)

The ReProto project is a language-neutral protocol specification, aimed towards describing and
generating code for handling messages exchanged through JSON-based APIs.

See [Specification][spec] for details on what the syntax of `.reproto` files is.
See [Integreation Tests][it] for some examples of protocol specifications.

**Note:** This project is in an Alpha-stage. Things will change a lot.

[spec]: /doc/spec.md
[examples]: /examples
[it]: /it

# Examples

Make you have [gotten started with Rust][rust-get-started].

Install reproto through cargo:

```bash
$> cargo install reproto
```

This will install the command into `~/.cargo/bin`, make sure it is in your `$PATH`.

Build documentation:

```bash
$> reproto compile doc -o target/doc --path it/test-service/proto \
  --package test \
  --package service@1.0.0 \
  --package service@2.0.0
$> open target/doc/index.html
```

For more example, please have a look at our [integration tests][it].

[rust-get-started]: https://doc.rust-lang.org/book/getting-started.html
[it]: /it

## [Maven Plugin][maven-plugin]

A Maven plugin that integrates reproto into the build lifecycle of a maven project.

[maven-plugin]: https://github.com/reproto/reproto-maven-plugin

## [VIM Plugin][vim]

A VIM plugin that provides syntax highlighting.

[vim]: https://github.com/reproto/reproto-vim

# Testing

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
