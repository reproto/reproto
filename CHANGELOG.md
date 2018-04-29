# Change Log
All notable changes to the reproto-vscode extension will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

- language-server: Support to find references ([#34]).
- language-server: clean up and fix bugs ([#47]).

[Unreleased]: https://github.com/reproto/reproto/compare/0.3.37...master

## [0.3.37]

- language-server: Support type renaming and symbol lookups ([#43])

[Unreleased]: https://github.com/reproto/reproto/compare/0.3.36...0.3.37
[#43]: https://github.com/reproto/reproto/issues/43

## [0.3.36]
- Support custom notification `$/openUrl` to permit vscode to jump to helpful locations when
  needed.
- Generic initializer logic moved into `env::initialize` and implemented in language server.
- language-server: Support for renaming package prefixes ([#34]).
- language-server: Support for introducing package prefixes from implicit prefixes ([#34]).

[0.3.36]: https://github.com/reproto/reproto/compare/0.3.35...0.3.36
[#34]: https://github.com/reproto/reproto/issues/34
