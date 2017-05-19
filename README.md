# reProto

This project extends protobuf messages with interfaces to improve how polymorphic types can be
represented and described.

See [Specification][spec] for details on what the syntax of `.reproto` files is.

See [Examples][examples] for some example API specifications.

[spec]: /doc/spec.md
[examples]: /examples

# Example

FasterXML Backend:

```bash
$> cargo run -- --debug --backend java/fasterxml --out target/java --path examples heroic.v1
```

You can also include one of the extensions:

```bash
$> cargo run -- --debug --backend java/fasterxml --out target/java --path examples --path examples/ext heroic.v1
```

Plain Python Backend:

```bash
$> cargo run -- --debug --backend python/plain --out target/python --path examples heroic.v1
```
