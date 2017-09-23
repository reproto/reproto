# User Configuration

Example configuration file (put in `~/.reproto/config.toml`).

```toml
# path to where to store local repositories.
local_repos = "/var/lib/reproto/repos"
# path to where to store the object lookup cache.
objects_cache = "/var/lib/reproto/cache"

[repository]
# Index to use for looking up packages.
index = "file:///home/me/repo/reproto-index"
# Object storage to use for looking up packages.
objects = "file:///home/me/repo/reproto-objects"
```

# Index Configuration

In the root of the index you may place a `config.json`, which may contain the following options:

```json
{
    "objects": "file:///home/me/repo/reproto-objects"
}
```

`objects`, this is the URL that will be used, unless specified in User Configuration, or using
`--objects <url>`.
By storing this in the index, the index can control where objects are being stored.
