PYTHON ?= python3

all:
	@echo "#!/usr/bin/env bash\nPYTHONPATH=$(CURDIR)/generated exec env $(PYTHON) $(CURDIR)/script.py" > script.sh
	@chmod +x script.sh
