all:
	@swift build
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec $(CURDIR)/.build/debug/swift" >> script.sh
	@chmod +x script.sh
