# reproto repository server

This is the server part of the HTTP repository of reproto.

## Configuration

Configuration for the server is defined in TOML, you can specify the configuration path to load
using the `REPROTO_SERVER_CONFIG` environment variable.

The following is a configuration with all options.

```toml
# The address to bind the server to.
listen_address = "127.0.0.1:1234"

# Path to the objects storage.
objects = "/var/reproto-server/objects"

# Maximum file size to permit during uploads.
max_file_size = 10000000
```

For a complete set of options and implementation details, please see [config.rs][config].

[config]: src/config.rs
