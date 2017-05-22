# ReProto Compiler

[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg?maxAge=2592000)](https://crates.io/crates/reproto)

The ReProto project is a language-neutral protocol specification, aimed towards describing and generating
code for handling messages exchanged through JSON-based APIs.

See [Specification][spec] for details on what the syntax of `.reproto` files is.
See [Examples][examples] for some example API specifications.

[spec]: /doc/spec.md
[examples]: /examples

# Example

Make you have [gotten started with Rust][rust-get-started].

Install reproto through cargo:

```bash
$> cargo install reproto
```

This will install the command into `~/.cargo/bin`, make sure it is in your `$PATH`.

Using the [FasterXML][fasterxml] backend for Java:

```bash
$> reproto --debug --b java -m fasterxml -o target/java \
  --path examples \
  heroic.v1
```

Multiple paths can be included, and the same declarations will extend each other:

```bash
$> reproto --debug -b java -m fasterxml -o target/java \
  --path examples \
  --path examples/ext \
  heroic.v1
```

This will generate code for the plain python backend:

```bash
$> reproto --debug -b python -o target/python \
  --path examples \
  heroic.v1
```

[fasterxml]: https://github.com/FasterXML/jackson-annotations

## [Maven Plugin][maven-plugin]

A Maven plugin that integrates reproto into the build lifecycle of a maven project.

[maven-plugin]: https://github.com/reproto/reproto-maven-plugin

## [VIM Plugin][vim]

A VIM plugin that provides syntax highlighting.

[vim]: https://github.com/reproto/reproto-vim
[rust-get-started]: https://doc.rust-lang.org/book/getting-started.html
