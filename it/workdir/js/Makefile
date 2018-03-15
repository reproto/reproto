BABEL ?= babel --presets es2015
OUT := $(CURDIR)/target

default:
	@$(BABEL) src generated --out-dir $(OUT)
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec env NODE_PATH="$(OUT)" node $(CURDIR)/target/script.js" >> script.sh
	@chmod +x script.sh
