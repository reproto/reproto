CARGO ?= cargo

BINARY = $(CURDIR)/target/release/rust

all:
	$(CARGO) build --release
	@echo "#!/usr/bin/env bash\nexec $(BINARY)" > script.sh
	@chmod +x script.sh
