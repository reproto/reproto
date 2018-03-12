.PHONY: all update tests dumps all-tests clean
.PHONY: suites update-suites
.PHONY: projects update-projects

FILTER ?=

all: suites projects

update: update-suites update-projects

tests: dumps
	cargo test --all

dumps: lib/backend-doc/dumps/syntaxdump lib/backend-doc/dumps/themedump

dumps-cmd := cargo run --bin reproto-pack --manifest-path=$(CURDIR)/tools/pack/Cargo.toml --
it-cmd := cargo run --manifest-path=$(CURDIR)/tools/it/Cargo.toml -- --root $(CURDIR)/it

lib/backend-doc/dumps/syntaxdump:
	$(dumps-cmd) --build-syntax=$(@)

lib/backend-doc/dumps/themedump:
	$(dumps-cmd) --build-themes=$(@)

all-tests: tests projects suites

clean: it-clean
	cargo clean

suites:
	$(it-cmd) --suite $(FILTER)

projects:
	$(it-cmd) --project $(FILTER)

update-suites:
	$(it-cmd) --update --suite $(FILTER)

update-projects:
	$(it-cmd) --update --project $(FILTER)

help:
	@echo ""
	@echo "Please read 'Suites & Projects' in README.md"
	@echo ""
	@echo "Variables (specified like 'make VARIABLE=VALUE <target>'):"
	@echo "  PROJECTS=foo     - only build the listed kinds of projects"
	@echo "  DEBUG=all        - (very) verbose output"
	@echo "  DEBUG=reproto    - debug reproto"
	@echo "  DEBUG=mvn        - debug Maven"
	@echo "  IT=basic - only build the specifiec integration tests"
	@echo ""
	@echo "Targets:"
	@echo "  all    - default target (suites projects)"
	@echo "  tests  - run unit tests for project"
	@echo "  clean  - clean build and tests"
	@echo "  update - update everything (update-suites update-projects)"
	@echo ""
	@echo "Suite Targets:"
	@echo "  suites        - run it suites"
	@echo "  update-suites - update expected output for it suites"
	@echo ""
	@echo "Project Targets:"
	@echo "  projects        - run it projects"
	@echo "  update-projects - update expected output for it projects"
	@echo ""
	@echo "Examples:"
	@echo "  Run all tests (very fast):"
	@echo "    make suites"
	@echo "  A single set of suites:"
	@echo "    make suites FILTER=basic"
	@echo "  A single set of projects:"
	@echo "    make projects FILTER=basic"
	@echo ""
