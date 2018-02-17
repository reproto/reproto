# WebAssembly support for reproto

This builds the reproto compiler as a WASM module.

#### Building

```bash
cargo web build
cp ../target/wasm32-unknown-unknown/release/reproto_wasm.* static/
```
