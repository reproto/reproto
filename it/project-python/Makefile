all:
	@./python-check
	@echo "#!/usr/bin/env bash\nPYTHONPATH=$(CURDIR)/generated exec env python3 $(CURDIR)/script.py" > script.sh
	@chmod +x script.sh
