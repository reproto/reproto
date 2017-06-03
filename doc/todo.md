# TODO

* General
  * ~~Treat named types as (named) simple types, like tuples.~~
  * ~~Enums.~~
  * Documentation syntax (preferably not comments).
  * Validators
    * Tricky, too many extensions to DSL to be useful.
      Might just skip for now but push towards `2.0`.

* ~~Maven Plugin + Maven Artifacts w/ Static Builds~~~
  * For clean integration into Java ecosystem.

* Java Backend
  * ~~Generate _good_ builder.~~
  * ~~Generated equals/hashCode (disabled when using lombok).~~
  * ~~Generated toString (disabled when using lombok).~~
  * ~~Tuple decoding (in `fasterxml`).~~

* Python Backend
  * ~~Encode support (e.g. `instance.encode()`)~~
  * ~~Relative import, especially with package prefixes.~~ (not needed with aliases)
  * ~~Create missing `__init__.py` files.~~
  * ~~Array decoding.~~
  * ~~Map decoding.~~
  * ~~Tuple decoding.~~
  * ~~Optional support.~~
  * ~~Encode `type` field.~~
  * ~~Enum support~~
  * Declarations need to be hoisted to support static initialization of enums since they mind
    depend on types in the same file. Alternatively, move initialization blocks to end of file.
  * Strict decoding/encoding where types are deeply verified.

* HTML Backend
  * Generate static documentation.

* JavaScript Backend
  * For browser compatibility.
