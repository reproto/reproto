# [![reproto](/gfx/logo.128.png?raw=true "reproto")](https://github.com/reproto)

[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![Build status](https://ci.appveyor.com/api/projects/status/9vgjwv3kfhwwt155/branch/master?svg=true)](https://ci.appveyor.com/project/udoprog/reproto/branch/master)
[![Chat on Gitter](https://badges.gitter.im/reproto/reproto.svg)](https://gitter.im/reproto/reproto)

A better way to define schemas for your JSON.

## Introduction

If you want to take the system for a spin, please go to <https://reproto.github.io>.

Reproto is:

* **A compiler** capable of generating code for [various languages].<br />
  [try it out][trycompiler] &ndash; [documentation][langsupport]
* **A custom interface description language** that permits describing the schema of JSON and
  services in a concise, easy to understand way.<br />
  [documentation][idl]
* **Early and extensive soundness checking**, with excellent error handling. We catch schema issues
  before you know that you have them.<br />
  [ui tests](/it/ui/checks)
* **A derive command**, capable of deriving schemas directly from JSON.<br />
  [try it out][tryderive] &ndash; [documentation][derive].
* **A semantic version checker** which verifies that modifications to schemas do not violate
  [semantic versioning].<br />
  [documentation][semver]
* **A build system with a package manager using build manifests**, giving you all the control you
  need to integrate reproto into your project.<br />
  [documentation][build manifests]
* **A rich, markdown-based documentation generator**.<br />
  [documentation][docgen]

These things combined support an ecosystem where schemas can be maintained and shared across
many teams.

You can install a binary version of reproto by running:

```
curl https://raw.githubusercontent.com/reproto/reproto/master/install.sh -sSf | bash
```

**Note:** This project is in an early stage. Things will change a lot. Please take it for a spin,
but avoid building large repositories of schemas for now.

[idl]: /doc/spec.md
[derive]: /doc/derive.md
[various languages]: #language-support
[langsupport]: /doc/usage/language-support.md
[semver]: /doc/semck.md
[semantic versioning]: https://semver.org
[docgen]: #generating-documentation
[central repository]: https://github.com/reproto/reproto-index
[build manifests]: /doc/manifest.md
[stdweb]: https://github.com/koute/stdweb
[trycompiler]: https://reproto.github.io/?input=reproto&output=java&package=example.type
[tryderive]: https://reproto.github.io/?input=json&output=java&package=example.type

## Getting Started

* See the [documentation] for an overview of how the reproto language and its build manifest works.
* See [examples] for some example specifications and projects.
* See the [integration tests] for even more examples on how protocol specifications can be used.
* See [release notes] for past and coming changes.

[documentation]: /doc/README.md
[examples]: /examples
[integration tests]: /it
[release notes]: /RELEASES.md

## Helping Out

You want to help out? Great!

You might want to start on issues marked with [good first issue].
If you have a support for a programming language that you feel is lacking, please help out with
[language support].

For any of these, just poke the issue with a quick `I want to do this!`.
If mentoring instructions are lacking, they will be made available as soon as possible.
Also make sure to [join our Gitter channel].

[good first issue]: https://github.com/reproto/reproto/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[language support]: https://github.com/reproto/reproto/issues?q=is%3Aissue+is%3Aopen+label%3Alang-support
[join our Gitter channel]: https://gitter.im/reproto/reproto

## Language Support

| Language     | JSON | [gRPC] | HTTP/1.1*                  |
|--------------|------|--------|----------------------------|
| [Java]       | ✔️    | ✔️      | ✔️  [test][java-http]       |
| [Python]     | ✔️    | ✖️      | ✔️  [test][python-requests] |
| [C#]         | ✔️    | ✖️      | ✖️                          |
| [Rust]       | ✔️    | ✖️      | ✔️  [test][rust-reqwest]    |
| [JavaScript] | ✔️    | ✖️      | ✖️                          |
| [Swift]      | ✔️    | ✖️      | ✖️                          |
| [Go]         | ✔️    | ✖️      | ✖️                          |

*: HTTP/1.1 support is actively being outlined in [#2](https://github.com/reproto/reproto/issues/2)

[Java]: /doc/usage/language-support.md#java
[Python]: /doc/usage/language-support.md#python
[C#]: /doc/usage/language-support.md#csharp
[Rust]: /doc/usage/language-support.md#rust
[JavaScript]: /doc/usage/language-support.md#javascript
[Swift]: /doc/usage/language-support.md#swift
[Go]: /doc/usage/language-support.md#go
[gRPC]: https://grpc.io
[java-http]: /it/java_okhttp2/proto/test.reproto
[python-requests]: /it/python_requests/proto/test.reproto
[rust-reqwest]: /it/rust_reqwest/proto/test.reproto

## Generating Documentation

`reproto` can generate rich markdown-based documentation from your specifications.

Go to <https://reproto.github.io/reproto/doc-examples/> to see what this documentation looks like.

These have been generated from the [examples project] using [tools/update-doc-examples].

[examples project]: /examples/
[tools/update-doc-examples]: /tools/update-doc-examples

## Installing from Source

Make sure you have [gotten started with Rust][rust-get-started].

Initialize submodules:

```bash
git submodule update --init
```

Pack syntax highlighting and themes:

```bash
make dumps
```

Build and install the CLI.
This will install `reproto` into `~/.cargo/bin`, make sure it is in your PATH:

```bash
cargo install --path $PWD/cli reproto
```

[rust-get-started]: https://rustup.rs

## Tools

* [reproto-vim], a VIM plugin that provides syntax highlighting.
* [reproto-maven], Maven plugin that integrates reproto into the build lifecycle of a maven project.
* [reproto-vscode], a visual studio code extension providing syntax highlighting and in-editor error diagnostics.

[reproto-vim]: https://github.com/reproto/reproto-vim
[reproto-maven]: https://github.com/reproto/reproto-maven-plugin
[reproto-vscode]: https://github.com/reproto/reproto-vscode

## Testing

This project includes an extensive set of integration tests.

See `make help` for documentation on what can be done.

Suites are fast tests which compiles a given set of rules, and compares with expected output stored
in this repository.

```bash
make suites
```

Projects are complete project tests.
These are projects written for various programming languages, and are generally harder to build.

```bash
make projects
```

To run all tests, do:

```bash
make all
```

For more information, run `make help`.
