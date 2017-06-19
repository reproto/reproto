.PHONY: all suites projects clean

PYTHON ?= python3
PROJECTS ?= $(shell PYTHON=$(PYTHON) tools/check-project-deps)
TOOL := $(CURDIR)/target/release/reproto
ENVIRONMENT = SUPPORTED_PROJECTS="$(PROJECTS)" PYTHON="$(PYTHON)" TOOL="$(TOOL)"
EACH := tools/for-each-it

all: suites projects

tests:
	cargo test
	cd reproto_core && cargo test
	cd reproto_parser && cargo test

clean:
	cargo clean
	$(EACH) clean

# simplified set of suites
suites: $(TOOL)
	$(EACH) TOOL="$(TOOL)" suites

update-suites: $(TOOL)
	$(EACH) TOOL="$(TOOL)" update-suites

clean-suites: $(TOOL)
	$(EACH) TOOL="$(TOOL)" clean-suites

# extensive project-building test suites
projects: $(TOOL)
	$(EACH) $(ENVIRONMENT) --no-print-directory projects

update-projects: $(TOOL)
	$(EACH) $(ENVIRONMENT) --no-print-directory update-projects

clean-projects: $(TOOL)
	$(EACH) $(ENVIRONMENT) --no-print-directory clean-projects

help:
	@echo ""
	@echo "Please read 'Suites & Projects' in README.md"
	@echo ""
	@echo "Variables (specified like 'make VARIABLE=VALUE <target>'):"
	@echo "  PROJECTS=\"foo\" - only build the listed kinds of projects"
	@echo "  DEBUG=yes        - verbose output (very)"
	@echo ""
	@echo "Targets:"
	@echo "  tests           - run unit tests for project"
	@echo "  clean           - clean build and tests"
	@echo ""
	@echo "Suite Targets:"
	@echo "  suites          - run it suites"
	@echo "  update-suites   - update expected output for it suites"
	@echo "  clean-suites    - clean it suites"
	@echo ""
	@echo "Project Targets:"
	@echo "  projects        - run it projects"
	@echo "  update-projects - update expected output for it projects"
	@echo "  clean-projects  - clean it projects"
	@echo ""
	@echo "Examples:"
	@echo "  Run all suites (very fast):"
	@echo "    make clean-suites suites"
	@echo "  A single set of suites:"
	@echo "    make -C it/test-match clean-suites suites"
	@echo "  A single set of projects:"
	@echo "    make -C it/test-match clean-projects projects"
	@echo ""

$(TOOL):
	cargo build --release
