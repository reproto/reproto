# Feature: `format_attribute`

Stable since: *not stable*

This feature deprecates the `datetime` and `bytes` types in favor of a new attribute: `#[format(..)]`.

So the following two specs are functionally identical:

```reproto
type Example {
    datetime_field: datetime;
    bytes_field: bytes;
}
```

```reproto
#![feature(format_attribute)]

type Example {
    #[format(datetime)]
    datetime_field: string;

    #[format(bytes)]
    bytes_field: string;
}
```

Switching to the format attribute has a couple of benefits.
The underlying type is clearly represented.
JSON doesn't support `datetime`, so they are effectively serialized as strings.

It is clearer for backends which have a hard time to support a specific format to gracefully
degrade to the underlying type instead.
