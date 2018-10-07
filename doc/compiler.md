# How the reproto compiler works

The compiler works in the following phases:

The [`Session`] ([`lib/trans`]) is responsible for module resolution and is generally
considered the driver of the compiler.

A simple example of this can be found in [`lib/compile`], which showcases a simplified compilation
through this module.
This module exists to support the [WASM-based compiler], and the one provided through the
[`reproto derive` command]

The [`Session`] requires a target for compilation, either through immediately importing a file,
or another package (through a [`resolver`]).

Once this has been provided, the content of the specification will be parsed using [`lib/parser`],
which performs lexing through [`lib/lexer`].

The parser is a [LALRPOP-based parser], and the [lexer is handwritten].

This translates the specification into an `AST` (defined in [`lib/ast`]).

After this, the AST is processed into RpIR defined in [`lib/core`]. These types are all defined in
[files named `rp_*.rs`].

All of this happens in [`lib/trans`], or more specifically: [`into_model.rs`].

[`lib/trans`]: /lib/trans
[`into_model.rs`]: /lib/trans/into_model.rs
[`lib/compile`]: /lib/compile
[`lib/parser`]: /lib/parser
[`lib/lexer`]: /lib/lexer
[`lib/ast`]: /lib/ast
[`lib/core`]: /lib/core
[`Session`]: /lib/trans/session.rs
[WASM-based compiler]: /eval/reproto_wasm.rs
[`reproto derive` command]: /cli/src/ops/derive.rs
[`resolver`]: /lib/core/src/resolver.rs
[LALRPOP-based parser]: /lib/parser/src/parser.lalrpop
[lexer is handwritten]: /lib/lexer/lexer.rs
[files named `rp_*.rs`]: /lib/core/src

## Intermediate representations

We've mentioned AST and RpIR in the previous sections.

The `AST` is an immediate reflection of a [parsed specification].
`RpIR` is after the AST has been processed a bit to make it easier and safer to compile into a
programming language.

This is a non-comprehensive list of what happens when translating `AST` to `RpIR`:

* Names are effectively references to other types, and they are translated into absolute
  references including package, version, and full paths.
  The specification permits relative names, and names imported from other files.
* Attributes are validated and translated into safer, easier to work with types.

## Flavors

RpIR can be flavored (default being `CoreFlavor`).

A flavor defines which types are used to store certain types of information.
RpIR can be translated from one flavor to another using `Session::translate`.

This process requires that one implements and provides the necessary `*Translator` traits which
decide how one flavor (e.g. `CoreFlavor`) is translated to another flavor.

If a backend wants to use `CoreFlavor`, `Session::translate_default` is available which does
the minimal amount of processing (referential integrity) but retains the original flavor.

[parsed specification]: /doc/spec.md
