# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- [automatic discovery of packages]. ([#49]).

### Fixed
- language-server: fix range translations to handle surrogate pairs correctly ([#51]).
- language-server: integrate rope-based sources into resolver. ([#49]).

[automatic discovery of packages]: https://github.com/reproto/reproto/blob/master/doc/manifest.md#package-discovery

## [0.3.38] - 2018-04-29
### Added
- language-server: Support to find references ([#34]).

### Fixed
- language-server: clean up and fix bugs ([#47]).

## [0.3.37] - 2018-04-28
### Added
- language-server: Support type renaming and symbol lookups ([#43])

## [0.3.36] - 2018-04-28
### Added
- Support custom notification `$/openUrl` to permit vscode to jump to helpful locations when
  needed.
- Generic initializer logic moved into `env::initialize` and implemented in language server.
- language-server: Support for renaming package prefixes ([#34]).
- language-server: Support for introducing package prefixes from implicit prefixes ([#34]).

[#34]: https://github.com/reproto/reproto/issues/34
[#43]: https://github.com/reproto/reproto/issues/43
[#47]: https://github.com/reproto/reproto/issues/47
[#49]: https://github.com/reproto/reproto/issues/49
[#51]: https://github.com/reproto/reproto/issues/51

[Unreleased]: https://github.com/reproto/reproto/compare/0.3.38...master
[0.3.38]: https://github.com/reproto/reproto/compare/0.3.37...0.3.38
[0.3.37]: https://github.com/reproto/reproto/compare/0.3.36...0.3.37
[0.3.36]: https://github.com/reproto/reproto/compare/0.3.35...0.3.36
