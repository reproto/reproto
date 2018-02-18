release := ../target/wasm32-unknown-unknown/release
sources += src/main.rs

out ?= target

.PHONY: all web

all: $(out) $(out)/reproto-wasm.js $(out)/reproto-wasm.wasm

web:
	cd $(out) && python3 -m http.server

$(out)/reproto-wasm.js: $(release)/reproto-wasm.js
	cp $< $@

$(out)/reproto-wasm.wasm: $(release)/reproto-wasm.wasm
	cp $< $@

$(release)/reproto-wasm.js $(release)/reproto-wasm.wasm: $(sources)
	cargo web build

$(out):
	mkdir -p $(out)
