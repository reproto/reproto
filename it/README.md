# reproto integration tests

Expected output for each suite, test, and language is listed in the [`expected`] directory.

Each test directory does the following.

* Build a _suite_, which is a very fast compilation of all the source specifications in the `proto`
  directory. These are then compared against the reference output in the `expected` directory.
* Build one or more _projects_, see [Projects](#projects)

Tests are all defined in [`tools/it/tests.rs`].
Some default options are defined in [`tools/it/main.rs`].

[`expected`]: expected
[`tools/it/tests.rs`]: /tools/it/tests.rs
[`tools/it/main.rs`]: /tools/it/main.rs

# Running Tests

Tests should be run from the root of the project:

```bash
$> make suites projects
```

A single test can be targeted with the `FILTER` parameter:

```bash
$> make suites projects FILTER="inner"
```

For more information, see:

```bash
$> make help
```

# Projects

Projects are complete projects written in the target language for reproto.

They are intended to test the complete integration of reproto, to verify that it actually works as
intended.

Each project is expected to fulfill the following protocol:

* Running `make` in the directory should generate an executable `script.sh` file.
* Running the generated `script.sh` should read JSON from stdin, line-by-line. And feed them into
  the deserialize implementation.
  * `script.sh` exiting with a non-zero exit status indicated a failure.
  * The script when run is expected to read JSON documents from stdin, and print them to stdout
    without modification.
    This assumes that the documents have been feed through internal models, therefore testing that
    the serialization is sound.

The base project available are:

* [Java](workdir/java)
* [JavaScript](workdir/js)
* [Rust](workdir/rust)
* [Python](workdir/python)
* [Python 3](workdir/python3)
* [C#](workdir/csharp)
