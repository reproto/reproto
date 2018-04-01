# reproto example project

See the [reproto.toml] manifest for details on how this project is built.

Packages:

 * [`com.github`] - An (attempted) formalization of the [GitHub v3 API].
 * [`io.reproto`] - A set of synthetic specifications showcasing the features of reproto.

Feel free to build this specification using:

```bash
$ cargo run --manifest-path=../cli/Cargo.toml -- doc
```

[reproto.toml]: reproto.toml
[GitHub v3 API]: https://developer.github.com/v3/
[`com.github`]: src/com/github
[`io.reproto`]: src/io/reproto

## Full example projects

This folder has a number of complete example projects which showcases a simple interaction with the
Github V3 API:

| Language             | Comment  |
|----------------------|----------|
| [Rust](rust-example) | Complete |

Note: to update examples, run `make update-examples` in the root of the project.
