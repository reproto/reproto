Rust.reproto_wasm.then((module) => {
  console.log(module.derive({content: "{\"foo\": \"bar\"}"}));
});
