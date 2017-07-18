CARGO ?= cargo

BINARY := $(CURDIR)/target/debug/rust

ifeq ($(DEBUG),yes)
CARGO_BUILD_OPTS :=
else
CARGO_BUILD_OPTS := -q
endif

all:
	@$(CARGO) build $(CARGO_BUILD_OPTS)
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec $(BINARY)" >> script.sh
	@chmod +x script.sh
