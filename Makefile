.PHONY: all suites projects clean

PYTHON ?= python3
PROJECTS ?= $(shell PYTHON=$(PYTHON) tools/check-it-dependencies)
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

$(TOOL):
	cargo build --release
