ifeq ($(ROOT),)
$(error "ROOT: missing variable")
endif

ifeq ($(PROJECTS),)
$(error "PROJECTS: missing variable")
endif

target-file ?= Makefile

script-input := $(ROOT)/tools/script-input
default-reproto := $(ROOT)/target/debug/reproto

DIFF ?= diff
RSYNC ?= rsync
REPROTO ?= $(default-reproto)
CARGO ?= cargo
# projects that are excluded
EXCLUDE ?=

reproto-cmd := $(REPROTO) $(REPROTO_ARGS)

expected := expected
output := output

targets := test

java-out := $(output)/java
python-out := $(output)/python
js-out := $(output)/js
rust-out := $(output)/rust
doc-out := $(output)/doc

java-expected := $(expected)/java
python-expected := $(expected)/python
js-expected := $(expected)/js
rust-expected := $(expected)/rust
doc-expected := $(expected)/doc

python-extra :=
java-extra := -m builder
js-extra :=
rust-extra :=
doc-extra :=
suites := python java js rust doc

paths := proto
exclude :=

include $(target-file)

exclude := $(exclude) $(EXCLUDE)

suites := $(filter-out $(exclude), $(suites))
projects := $(filter-out $(exclude), $(PROJECTS))

packages-args := $(targets:%=--package %)

suite-diffs := $(suites:%=suite-diff/%)
suite-updates := $(suites:%=suite-update/%)

project-diffs := $(projects:%=project-diff/%)
project-updates := $(projects:%=project-update/%)

java-suite := java $(java-extra)
js-suite := js $(js-extra)
python-suite := python $(python-extra)
rust-suite := rust $(rust-extra)
doc-suite := doc --skip-static $(doc-extra)

java-project := java -m fasterxml -o workdir-java/target/generated-sources/reproto
js-project := js -o workdir-js/generated
python-project := python -o workdir-python/generated
rust-project := rust -o workdir-rust/src --package-prefix generated

paths-args := $(paths:%=--path %)

.PHONY: all clean update
.PHONY: projects clean-projects update-projects
.PHONY: suites clean-suites update-suites

all: suites projects

clean: clean-projects clean-suites

update: update-suites update-projects

suites: $(suite-diffs)

clean-suites:
	rm -rf output

update-suites: $(suite-updates)

projects: $(project-diffs)

clean-projects:
	rm -rf workdir-*
	rm -rf output-*

update-projects: $(project-updates)

suite-build/%: $(REPROTO)
	@echo "Suite: $*"
	$(reproto-cmd) compile $($*-suite) -o $($*-out) $(paths-args) $(packages-args)

suite-update/%: suite-build/%
	@echo "Updating Suite: $*"
	$(RSYNC) --delete -ra $($*-out)/ $($*-expected)/

suite-diff/%: suite-build/%
	@echo "Verifying Diffs: $*"
	$(DIFF) -ur $($*-expected) $($*-out)

project-build/%: $(REPROTO) output-% expected-%
	@echo "Building Project: $*"
	$(RSYNC) --delete -ra ../$*/ workdir-$*
	$(reproto-cmd) compile $($*-project) $(paths-args) $(packages-args)
	cd workdir-$* && make
	$(script-input) workdir-$*/script.sh

# rule to diff a projects expected output, with actual.
project-diff/%: project-build/%
	@echo "Diffing Project: $*"
	$(DIFF) -ur expected-$* output-$*

# rule to update a projects expected output, with its actual
project-update/%: project-build/%
	@echo "Updating Project: $*"
	$(RSYNC) --delete -ra output-$*/ expected-$*/

$(default-reproto):
	@echo "Building $(default-reproto)"
	cd $(ROOT) && $(CARGO) build

expected-% output-%:
	mkdir -p $@
