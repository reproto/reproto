CARGO ?= cargo

BINARY := $(CURDIR)/target/debug/rust

ifeq ($(filter all cargo,$(DEBUG)),)
override CARGO_FLAGS += -q
endif

all:
	@$(CARGO) build $(CARGO_FLAGS)
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec $(BINARY)" >> script.sh
	@chmod +x script.sh
