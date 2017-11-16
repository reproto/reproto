# ReProto Compiler 
[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg)](https://crates.io/crates/reproto)

The ReProto project is a language-neutral protocol specification, aimed towards describing and
generating code for handling messages exchanged through JSON-based APIs.

ReProto specifiec an [interface description language][idl] (IDL) for specifying schemas.
These schemas describe the structure of JSON, and can be used to generate data structures in
several different languages.

* See [Specification][spec] for details on what the syntax of `.reproto` files is.
* See [TODO][todo] for details on things that still needs to be done.
* See [Examples][examples] for some example protocol specifications.
* See [Config][config] for how to configure ReProto.
* See [Integration Tests][it] for some examples of how protocol specifications can be used.

**Note:** This project is in an Alpha-stage. Things will change a lot.

[idl]: #the-idl
[spec]: /doc/spec.md
[todo]: /doc/todo.md
[config]: /doc/config.md
[examples]: /examples
[it]: /it

# Supported Backends

* Java (`java`)
  * Data models using [fasterxml jackson][jackson] (`-m fasterxml`), and/or
    [lombok][lombok] (`-m lombok`).
* JavaScript (`js`)
  * ES2015 classes, that can be transpiled using babel (see [Integration Test][js-it]).
* Python (`python`)
  * Plain-python classes, compatible with 2 and 3 for databinding.
* Rust (`rust`)
  * Serde-based serialization.
* Doc (`doc`)
  * HTML-based documentation, based from contextual markdown comments.

[lombok]: https://projectlombok.org/
[jackson]: https://github.com/FasterXML/jackson-databind
[js-it]: /it/js

# Examples

Make you have [gotten started with Rust][rust-get-started].

Build ReProto using cargo:

```bash
$> cargo install --path $PWD/cli reproto
```

This will install `reproto` into `~/.cargo/bin`, make sure it is in your PATH.

The following is an example of how to build documentation for a package.

```bash
$> reproto doc -o target/doc \
  --path it/test-service/proto \
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

# The IDL

ReProto specifiec an interface description language (IDL) for specifying schemas.
These schemas describe the structure of JSON, and can be used to generate data structures in
several different languages.

The ReProto IDL is not based on an existing general purpose markup like JSON.

The goal is to have a compact, intuitive, and productive language for writing specifications.

The following is a simple petstore example using ReProto.

```reproto
/// # ReProto Petstore
///
/// A sample API that uses a petstore as an example to demonstrate features in the ReProto
/// specification
service Petstore {
  /// Returns all pets from the system that the user has access to.
  all_pets() -> stream Pet;
}

enum Size as string {
    LARGE;
    MEDIUM;
    SMALL;
}

type Pet {
  id: unsigned/64;
  name: string;
  size: Size;
}
```

You can compile the above into documentation using the following command:

```bash
$> reproto doc --out petstore-doc --path examples/src --package petstore
```

If you miss JSON, you can compile the specification to JSON as well.

```bash
$> reproto build --lang json --out petstore-json --path examples/petstore --package petstore
```
