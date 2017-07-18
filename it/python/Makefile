PYTHON ?= python3

all:
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec env PYTHONPATH=$(CURDIR)/generated $(PYTHON) $(CURDIR)/script.py" >> script.sh
	@chmod +x script.sh
