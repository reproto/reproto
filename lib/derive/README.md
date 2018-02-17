# reproto schema deriver

Component that drives `reproto derive`, a tool for generating a schema from an existing document.

For a more thorough guide, see [the documentation].

This component uses an intermediate representation called SIR (Structured Intermediate
Representation), which permits it to support multiple input formats.

 * [JSON], through `serde_json` (default, or `--format json`).
 * [YAML], through `serde_yaml` (`--format yaml`).

[the documentation]: /doc/derive.md
[JSON]: json.rs
[YAML]: yaml.rs
