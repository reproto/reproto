PYTHON3 ?= python3

default:
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec env PYTHONPATH=$(CURDIR)/generated $(PYTHON3) $(CURDIR)/script.py" >> script.sh
	@chmod +x script.sh
