# Feature Flags

This is a repository of all feature flags available for reproto.

Feature flags are active if:
 * A flag _is not_ stable, and is explicitly activated with `#[feature(name_of_flag)]`.
 * A flag _is_ stable, and the schema version is equal to or greater than the version that it was
   introduced.

Feature flags can also be deprecated, in which case they are no longer activated after a specific
schema version.
