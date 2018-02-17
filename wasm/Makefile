release := ../target/wasm32-unknown-unknown/release
sources += src/main.rs

.PHONY: all web

all: static/reproto-wasm.js static/reproto-wasm.wasm

web:
	cd static && python3 -m http.server

static/reproto-wasm.js: $(release)/reproto-wasm.js
	cp $< $@

static/reproto-wasm.wasm: $(release)/reproto-wasm.wasm
	cp $< $@

$(release)/reproto-wasm.js $(release)/reproto-wasm.wasm: $(sources)
	cargo web build
