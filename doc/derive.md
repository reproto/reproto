# Deriving schemas from JSON

reproto can derive schemas from existing JSON through the `reproto derive` command.

This uses the [`lib/derive`] component.

[`lib/derive`]: /lib/derive

```bash
reproto derive <<< '{"id": 42, "name": "Oscar"}'
```

This will print:

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

Included in the project is a larger example of a [Github Issue].
You can try to derive the schema from this file directly.
Fair warning though, it will be large:

```bash
reproto derive < doc/github-issue.json
```

[Github Issue]: /doc/github-issue.json

## De-duplication

reproto tries to de-duplicate generated types.
If a given type has a set of fields that is identical with another, it will only generate one
common type for both.

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
