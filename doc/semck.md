# reproto semantic version checker (semck)

The semantic version checker (semck) is a component of reproto which verifies that updates to
a schema does not violate semantic versioning.

The version checker is triggered automatically during a `publish` command:

```bash
$ reproto publish
io.reproto.toystore-1.0.0:11:3-23:
 11:   get_toys() -> [Toy];
       ^^^^^^^^^^^^^^^^^^^^ - patch change violation: endpoint removed
Hint: Use `--no-semck` to disable semantic checking
```

It can also be invoked manually through `reproto check`.

```bash
$ reproto check
io.reproto.toystore-1.0.0:11:3-23:
 11:   get_toys() -> [Toy];
       ^^^^^^^^^^^^^^^^^^^^ - patch change violation: endpoint removed
```
