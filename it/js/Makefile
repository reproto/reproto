BABEL ?= babel --presets es2015
OUT := $(CURDIR)/target

all:
	@cd src && $(BABEL) **/*.js --out-dir $(OUT)
	@cd generated && $(BABEL) **/*.js --out-dir $(OUT)
	@echo "#!/usr/bin/env bash\nexec env node $(CURDIR)/target/script.js" > script.sh
	@chmod +x script.sh
