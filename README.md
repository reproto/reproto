# reProto

[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg?maxAge=2592000)](https://crates.io/crates/reproto)

This project is a language-neutral protocol specification geared towards describing and generating
code for handling JSON-based APIs.

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

This will install the command into `~/.cargo/bin`.

FasterXML Backend:

```bash
$> reproto --debug --b java -m fasterxml -o target/java \
  --path examples \
  heroic.v1
```

You can also include one of the extensions:

```bash
$> reproto --debug -b java -m fasterxml -o target/java \
  --path examples \
  --path examples/ext \
  heroic.v1
```

Plain Python Backend:

```bash
$> reproto --debug -b python -o target/python \
  --path examples \
  heroic.v1
```

# [Maven Plugin][maven-plugin]

A Maven plugin that integrates reproto into the build lifecycle of a maven project.

[maven-plugin]: https://github.com/reproto/reproto-maven-plugin

# [VIM Plugin][vim]

A VIM plugin that provides syntax highlighting.

[vim]: https://github.com/reproto/reproto-vim
[rust-get-started]: https://doc.rust-lang.org/book/getting-started.html
