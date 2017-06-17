.PHONY: all suites projects clean

EACH := tools/for-each-it

PYTHON ?= python3
PROJECTS = $(shell PYTHON=$(PYTHON) tools/check-it-dependencies)
ENVIRONMENT = SUPPORTED_PROJECTS="$(PROJECTS)" PYTHON="$(PYTHON)"

all: suites

clean:
	$(EACH) clean

# simplified set of suites
suites:
	$(EACH) suites

update:
	$(EACH) update

# extensive project-building test suites
projects:
	@echo "Building and testing: $(PROJECTS)"
	$(EACH) $(ENVIRONMENT) --no-print-directory projects

update-projects:
	@echo "Updating: $(PROJECTS)"
	$(EACH) $(ENVIRONMENT) --no-print-directory update-projects
