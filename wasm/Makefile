release := ../target/wasm32-unknown-unknown/release
sources += src/main.rs

out ?= target

.PHONY: all web

all: $(out) $(out)/reproto-wasm.js $(out)/reproto-wasm.wasm

web: $(out)/reproto-wasm.js  $(out)/reproto-wasm.wasm $(out)/index.html $(out)/main.js
	cd $(out) && python3 -m http.server

$(out)/reproto-wasm.js: $(release)/reproto-wasm.js
	cp $< $@

$(out)/reproto-wasm.wasm: $(release)/reproto-wasm.wasm
	cp $< $@

$(release)/reproto-wasm.js $(release)/reproto-wasm.wasm: $(sources)
	cargo web build

$(out)/index.html: $(CURDIR)/static/index.html
	cp $(CURDIR)/static/index.html $@

$(out)/main.js: $(CURDIR)/static/main.js
	cp $(CURDIR)/static/main.js $@

$(out):
	mkdir -p $(out)
