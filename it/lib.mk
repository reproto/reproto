SCRIPT_INPUT=$(CURDIR)/../tools/script-input

ROOT ?= ../..

DEFAULT_TOOL := $(ROOT)/target/release/reproto

DIFF ?= diff
RSYNC ?= rsync
TOOL ?= $(DEFAULT_TOOL)
CARGO ?= cargo

EXPECTED = expected
OUTPUT = output
PATHS := proto $(PATHS)

SUITES ?= python java js rust
TARGETS ?= test
FILTERED ?=

java_out = $(OUTPUT)/java
python_out = $(OUTPUT)/python
js_out = $(OUTPUT)/js
rust_out = $(OUTPUT)/rust

PYTHON_EXTRA ?=
JAVA_EXTRA ?= -m builder
JS_EXTRA ?=
RUST_EXTRA ?=

# projects that are filtered
FILTERED_PROJECTS ?=
# projects that are supported after checking that necessary tools are available
SUPPORTED_PROJECTS ?= %

SUITES := $(filter-out $(FILTERED),$(SUITES))
PROJECTS ?= $(filter $(SUPPORTED_PROJECTS), $(filter-out $(FILTERED_PROJECTS),$(SUITES)))

PACKAGES := $(TARGETS:%=--package %)

project_targets := $(PROJECTS:%=project-%)
suite_targets := $(SUITES:%=suite-%)
project_diffs := $(PROJECTS:%=projectdiff-%)
project_updates := $(PROJECTS:%=projectupdate-%)
paths := $(PATHS:%=--path %)

DEBUG ?= no

ifeq ($(DEBUG),yes)
O :=
reproto := $(TOOL) --debug
else
O := @
reproto := $(TOOL)
endif

java_suite := -b java $(JAVA_EXTRA)
java_project := -b java -m fasterxml -o workdir-java/target/generated-sources/reproto

js_suite := -b js $(JS_EXTRA)
js_project := -b js -o workdir-js/generated

python_suite := -b python $(PYTHON_EXTRA)
python_project := -b python -o workdir-python/generated

rust_suite := -b rust $(RUST_EXTRA)
rust_project := -b rust -o workdir-rust/src --package-prefix generated

.PHONY: all clean suites projects update update-projects

all: suites projects

update: update-suites update-projects

clean-projects:
	$Orm -rf workdir-*
	$Orm -rf output-*

clean-suites:
	$Orm -rf output

clean: clean-projects clean-suites

suites: $(suite_targets) diff

projects: $(project_targets) $(project_diffs)

update-projects: $(project_targets) $(project_updates)

update-suites: $(suite_targets)
	$Oecho "Updating Suites"
	$O$(RSYNC) --delete -ra $(OUTPUT)/ $(EXPECTED)/

diff:
	$Oecho "Verifying Diffs"
	$O$(DIFF) -ur $(EXPECTED) $(OUTPUT)

# rule to diff a projects expected output, with actual.
projectdiff-%:
	$Oecho "Diffing Project: $*"
	$O$(DIFF) -ur expected-$* output-$*

# rule to update a projects expected output, with its actual
projectupdate-%:
	$Oecho "Updating Project: $*"
	$O$(RSYNC) --delete -ra output-$*/ expected-$*/

# rule to build output for a project
project-%: $(TOOL)
	$O$(RSYNC) --delete -ra ../$*/ workdir-$*
	$O$(reproto) compile $($*_project) $(paths) $(PACKAGES)
	$Ocd workdir-$* && make
	$O$(SCRIPT_INPUT) workdir-$*/script.sh

# rule to build suite output
suite-%: $(TOOL)
	$Oecho "Suite: $*"
	$O$(reproto) compile $($*_suite) -o $($*_out) $(paths) $(PACKAGES)

$(DEFAULT_TOOL):
	$Oecho "Building $(DEFAULT_TOOL)"
	$O$(CARGO) build --release
