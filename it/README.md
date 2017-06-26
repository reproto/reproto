# Integration Tests for ReProto

Each test directory (`test-*`) does the following.

* Build a _suite_, which is a very fast compilation of all the source specifications in the `proto`
  directory. These are then compared against the reference output in the `expected` directory.
* Build one or more _projects_, see [Projects](#projects)

Each test is configured in the `Makefile` for that test.
This is based by a framework defined in [`lib.mk`](lib.mk).

# Projects

Projects are complete projects written in the target language for ReProto.

They are intended to test the complete integration of ReProto, to verify that it actually works as
intended.

Each project is expected to fullfill the following protocol:

* Running `make` in the directory should generate an executable `script.sh` file.
* Running the generated `script.sh` should read JSON from stdin, line-by-line. And feed them into
  the deserialize implementation.
  * `script.sh` exiting with a non-zero exit status indicated a failure.
  * The run script is permitted to output anything it wants. This will be compared against the
    expected output.
  * Most implementation also attempts to serialize the output, in which case it is also printed to
    stdout.

The base project available are:

* [Java](java)
* [JavaScript](js)
* [Rust](rust)
* [Python](python)
