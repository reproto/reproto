# Introduction

Reproto is a system for managing JSON schemas.
It permits a developer to specify the structure of JSON as specifications, that can then be used to
generated language-specific bindings.

This project has a number of goals:

**Implement a schema language that is concise and expressive** &mdash;
The specification of a JSON object should not be too verbose, otherwise it risks becoming a burden
to maintain.

**Be a stable, self-contained system** &mdash;
Everything related to reproto, from the compiler to all tools, is part of the same project.

**Push the envelope** &mdash;
Reproto is fundamentally an experiment intended to push the envelope in how to do schema management
for JSON.
We aim to improve the state-of-the art.

**Correctness before performance** &mdash;
Schemas are worthless if they don't provide guarantees in the structure they describe.
We do our best to guarantee that the generated code follow the specification provided to the
letter.

## Documentation

The documentation is composed of the following sections:

 * [Getting started](usage/getting-started.md).
 * [Language support](usage/language-support.md), language-specific options and considerations
   when using reproto.
 * [The specification language](spec.md), detailing how the `.reproto` specification language works.
 * [Build manifests](manifest.md), detailing how to use the `reproto.toml` build manifest to configure how
   specifications are built for your project.
 * [Automatically deriving schemas from JSON](derive.md), detailing how use the `reproto derive`
   command to quickly get started writing schemas.
 * [How to configure reproto](config.md).
 * [How the compiler works](compiler.md), this section is written towards people interested in
   working with the reproto compiler.
 * [Setting up a repository](usage/setting-up-a-repository.md).
