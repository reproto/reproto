# reproto documentation

reproto is a system for handling JSON schemas.

This document will cover the different parts of this system, which includes:

 * A custom [interface description language] that permits describing the schema of JSON and
   bidirectional rpc services (like [gRPC]).
 * A compiler which generates code for [various languages].
 * A [semantic version checker] which verifies that modifications to schemas do not violate
   [semantic versioning].
 * A build system and package manager.
 * A rich, markdown-based [documentation generator].

[gRPC]: https://grpc.io
[interface description language]: spec.md
[various languages]: spec.md#language-support
[semantic version checker]: semck.md
[semantic versioning]: https://semver.org
[documentation generator]: spec.md#documentation

The documentation is divided into the following sections:

 * The [specification language], detailing how the `reproto` language works.
 * The [build manifest], detailing how to use the `reproto.toml` build manifest.
 * How to [configure reproto].
 * How to [use reproto].

[specification language]: spec.md
[build manifest]: manifest.md
[use reproto]: usage.md
[configure reproto]: config.md
