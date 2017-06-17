.PHONY: all suites projects clean

EACH := tools/for-each-it

PYTHON ?= python3
PROJECTS ?= $(shell PYTHON=$(PYTHON) tools/check-it-dependencies)
TOOL := $(CURDIR)/target/release/reproto
ENVIRONMENT = SUPPORTED_PROJECTS="$(PROJECTS)" PYTHON="$(PYTHON)" TOOL="$(TOOL)"

all: suites

tests:
	cargo test
	cd reproto_core && cargo test
	cd reproto_parser && cargo test

clean:
	$(EACH) clean

# simplified set of suites
suites: $(TOOL)
	$(EACH) TOOL="$(TOOL)" suites

update: $(TOOL)
	$(EACH) TOOL="$(TOOL)" update

# extensive project-building test suites
projects: $(TOOL)
	@echo "Building and testing: $(PROJECTS)"
	$(EACH) $(ENVIRONMENT) --no-print-directory projects

update-projects: $(TOOL)
	@echo "Updating: $(PROJECTS)"
	$(EACH) $(ENVIRONMENT) --no-print-directory update-projects

$(TOOL):
	cargo build --release
