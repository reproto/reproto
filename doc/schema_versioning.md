# Schema Versioning

Schema versioning is _how_ changes are introduced into the reproto language.

Every schema file has a _schema version_ which describes the version of the reproto schema that it
declares itself compatible to.

Schema versions are declared using the `#![reproto(version = ..)]` attribute.
The value is a semantic version:

```reproto
#![reproto(version = "1.0.0")]

// rest of schema...
```

Schema versions implicitly enable a set of features.
Features are how _changes_ into the schema language is introduced.

Let's say there is a feature called `format_attribute` that is in development.
This feature will only be active if explicitly activated through a `#![feature(format_attribute)]`
attribute.
Like this:

```reproto
#![reproto(version = "1.0.0")]
#![feature(format_attribute)]

// rest of schema...
```

When this feature is stable, we release a new schema version and mark that this feature should be
enabled by default assuming that a schema declares compatibility for that version.

A schema _cannot_ be published if a feature is enabled.
Allowing this would break forward compatibility guarantees, since features which are not yet stable
are subject to change.
Features are purely made available to permit local experimentation with up and coming schema
changes.

After a while the `format_attribute` feature is marked as stable in `2.0.0`.
Declaring compatibility with `2.0.0` and beyond will now forever have this feature enabled.

```reproto
#![reproto(version = "2.0.0")]
// #![feature(format_attribute)]

// rest of schema...
```
