.PHONY: all suites projects clean

default-reproto := $(CURDIR)/target/debug/reproto

ROOT ?= $(CURDIR)
PYTHON ?= python3
REPROTO ?= $(default-reproto)
EACH := tools/for-each-it

ifneq ($(M),)
RUN := $(MAKE) -C $(M) -f $(ROOT)/it/lib.mk
else
RUN := $(EACH)
endif

ifeq ($(DEBUG),yes)
REPROTO_ARGS := --debug
else
RUN := $(RUN) --no-print-directory -s
REPROTO_ARGS :=
endif

export ROOT
export PROJECTS := $(shell PYTHON=$(PYTHON) $(ROOT)/tools/check-project-deps)
export PYTHON
export MAKE
export REPROTO
export REPROTO_ARGS

all: suites projects

update: update-suites update-projects

tests:
	cargo test --all

clean:
	cargo clean
	+$(RUN) clean

# simplified set of suites
suites: $(REPROTO)
	+$(RUN) suites

update-suites: $(REPROTO)
	+$(RUN) update-suites

clean-suites: $(REPROTO)
	+$(RUN) clean-suites

# extensive project-building test suites
projects: $(REPROTO)
	+$(RUN) projects

update-projects: $(REPROTO)
	+$(RUN) update-projects

clean-projects: $(REPROTO)
	+$(RUN) clean-projects

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
	@echo "  Run all suites (very fast):"
	@echo "    make clean-suites suites"
	@echo "  A single set of suites:"
	@echo "    make -C it/test-match clean-suites suites"
	@echo "  A single set of projects:"
	@echo "    make -C it/test-match clean-projects projects"
	@echo ""
