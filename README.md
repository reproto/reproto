# rePROTO

This project extends protobuf messages with interfaces to improve how polymorphic types can be
represented and described.

See [Specification][spec] for details on what the syntax of `.reproto` files is.

See [Examples][examples] for some example API specifications.

[spec]: /doc/spec.md
[examples]: /examples

# Example

Try out the FasterXML backend:

```bash
$> cargo run -- --debug --backend fasterxml --out target/generated-sources --path examples heroic.v1
```

You can also include one of the extensions:

```bash
$> cargo run -- --debug --backend fasterxml --out target/generated-sources --path examples --path examples/ext heroic.v1
```
