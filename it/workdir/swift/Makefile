define build
	@mkdir -p Sources/swift
	@cp $1.swift.disabled Sources/swift/main.swift
	@swift build
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec $(CURDIR)/.build/debug/swift" >> script.sh
	@chmod +x script.sh

endef

simple:
	$(call build,simple)

codable:
	$(call build,codable)
