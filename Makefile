default-reproto := $(CURDIR)/target/debug/reproto

ifeq ($(DEBUG),yes)
make-args :=
else
make-args := --no-print-directory -s
endif

# set IT="<dir>" to limit which modules to build
IT ?= $(wildcard it/test-*)
run = $(MAKE) $(make-args) -f tools/Makefile.each dirs="$(IT)" $(IT) target=$1

export ROOT := $(CURDIR)
export PYTHON ?= python3
export PROJECTS := $(shell PYTHON=$(PYTHON) tools/check-project-deps)
export REPROTO ?= $(default-reproto)

.PHONY: all update tests clean
.PHONY: suites update-suites clean-suites
.PHONY: projects update-projects clean-projects

all: suites projects

update: update-suites update-projects

tests:
	cargo test --all

clean:
	cargo clean
	$(call run,clean)

# simplified set of suites
suites: $(REPROTO)
	$(call run,suites)

update-suites: $(REPROTO)
	$(call run,update-suites)

clean-suites: $(REPROTO)
	$(call run,clean-suites)

# extensive project-building test suites
projects: $(REPROTO)
	$(call run,projects)

update-projects: $(REPROTO)
	$(call run,update-projects)

clean-projects: $(REPROTO)
	$(call run,clean-projects)

$(default-reproto):
	cargo build

help:
	@echo ""
	@echo "Please read 'Suites & Projects' in README.md"
	@echo ""
	@echo "Variables (specified like 'make VARIABLE=VALUE <target>'):"
	@echo "  PROJECTS=foo - only build the listed kinds of projects"
	@echo "  DEBUG=yes    - (very) verbose output"
	@echo "  EXCLUDE=rust - exclude the named targets"
	@echo "  INCLUDE=rust - only include the named targets"
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
	@echo "  clean-suites  - clean it suites"
	@echo ""
	@echo "Project Targets:"
	@echo "  projects        - run it projects"
	@echo "  update-projects - update expected output for it projects"
	@echo "  clean-projects  - clean it projects"
	@echo ""
	@echo "Examples:"
	@echo "  Run all tests (very fast):"
	@echo "    make suites"
	@echo "  A single set of suites:"
	@echo "    make IT=it/test-basic clean-suites suites"
	@echo "  A single set of projects:"
	@echo "    make IT=it/test-basic clean-projects projects"
	@echo ""
