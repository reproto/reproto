# Deriving schemas from JSON

To use these features, make sure to have reproto installed:

```
cargo install reproto
```

reproto can derive schemas from existing JSON through the `reproto derive` command through the
[`lib/derive`] component.

[`lib/derive`]: /lib/derive

```bash
reproto derive <<< '{"id": 42, "name": "Oscar"}'
```

This will give you:

```reproto
type Generated {
  /// ## Examples
  ///
  /// ```json
  /// 42
  /// ```
  id: u64;

  /// ## Examples
  ///
  /// ```json
  /// "Oscar"
  /// ```
  name: string;
}
```

We can now try to build this schema into `rust`:

```bash
reproto derive <<< '{"id": 42, "name": "Oscar"}' > out.reproto
reproto build --file out.reproto --lang rust --package-prefix test -m chrono --out target/
cat target/test.rs
```

This gives:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Generated {
  id: u64,
  name: String,
}
```

Included in the project is a larger example of a [Github Issue].
You can try to derive the schema from this file directly.
Fair warning though, it will be large:

```bash
reproto derive < doc/github-issue.json
```

[Github Issue]: /doc/github-issue.json

## Interfaces

If a given type follows field-based polymorhpism, `derive` can detect that.
The discriminator must be a string, and common fields for this purpose (e.g. `@class` or `type`)
are preferred.

```bash
reproto derive <<< '[
    {"kind": "dragon", "name": "Stephen", "age": 4812, "fire": "blue"},
    {"kind": "knight", "name": "Olivia", "armor": "Unobtanium"}
]'
```

```reproto
#[type_info(strategy = "tagged", tag = "kind")]
interface Generated {
  Dragon as "dragon" {
    /// ## Examples
    ///
    /// ```json
    /// "Stephen"
    /// ```
    name: string;

    /// ## Examples
    ///
    /// ```json
    /// 4812
    /// ```
    age: u64;

    /// ## Examples
    ///
    /// ```json
    /// "blue"
    /// ```
    fire: string;
  }

  Knight as "knight" {
    /// ## Examples
    ///
    /// ```json
    /// "Olivia"
    /// ```
    name: string;

    /// ## Examples
    ///
    /// ```json
    /// "Unobtanium"
    /// ```
    armor: string;
  }
}
```

## Deduplication

reproto tries to deduplicate generated types.

If a given type has a set of fields that is identical with another, it will only generate one
common type for both:

```bash
reproto derive <<< '[{"id": 42, "name": "Oscar"}, {"id": 1, "name": "Sophie"}]'
```

```reproto
type Generated {
  /// ## Examples
  ///
  /// ```json
  /// 42
  /// 1
  /// ```
  id: u64;

  /// ## Examples
  ///
  /// ```json
  /// "Oscar"
  /// "Sophie"
  /// ```
  name: string;
}
```

## Type refining

Sometimes values diverge slighty in how they are provided.
When that happens, reproto tries to refine the provided type as more information becomes available.

For this example, the first object has a `null` field `name`.
Normally this would be given the `any` type.
But if reproto is provided with an additional document that has a type for the same field, this
field can be 'refined' into an _optional_ field of the given type.

```bash
reproto derive <<< '[{"id": 42, "name": null}, {"id": 2, "name": "Stephen"}]'
```

```reproto
type Generated {
  /// ## Examples
  ///
  /// ```json
  /// 42
  /// 2
  /// ```
  id: u64;

  /// ## Examples
  ///
  /// ```json
  /// "Stephen"
  /// ```
  name?: string;
}
```

Refinement can also realize when a type has a looser contract than before, as is showcased here
where a double is required to store `42.2`.

```bash
reproto derive <<< '[{"id": 1, "height": 40}, {"id":2, "height": 42.2}]'
```

```reproto
type Generated {
  /// ## Examples
  ///
  /// ```json
  /// 1
  /// 2
  /// ```
  id: u64;

  /// ## Examples
  ///
  /// ```json
  /// 10
  /// 42.2
  /// ```
  height: double;
}
```
