ifeq ($(ROOT),)
$(error "ROOT: missing variable")
endif

ifeq ($(PROJECTS),)
$(error "PROJECTS: missing variable")
endif

target-file ?= Makefile

run-project := $(ROOT)/tools/run-project
default-reproto := $(ROOT)/target/debug/reproto

RM := rm -rf
CP := cp -ra
DIFF ?= diff
RSYNC ?= rsync
REPROTO ?= $(default-reproto)
CARGO ?= cargo
# projects that are excluded
EXCLUDE ?=

expected := expected
output := output
workdir := workdir
input := input
targets := test

python-args :=
java-args := -m builder
js-args :=
rust-args :=
doc-args :=
suites := python java js rust doc

paths := proto
exclude-projects :=
exclude-suites :=

include $(target-file)

suites := $(filter-out $(EXCLUDE) $(exclude-suites), $(suites))
projects := $(filter-out $(EXCLUDE) $(exclude-projects), $(PROJECTS))

suite-diffs := $(suites:%=suite-diff/%)
suite-updates := $(suites:%=suite-update/%)

project-diffs := $(projects:%=project-diff/%)
project-updates := $(projects:%=project-update/%)

reproto-args := $(paths:%=--path %) $(targets:%=--package %)

# how to build suites
java-suite := java $(reproto-args) $(java-args)
js-suite := js $(reproto-args) $(js-args)
python-suite := python $(reproto-args) $(python-args)
rust-suite := rust $(reproto-args) $(rust-args)
doc-suite := doc $(reproto-args) --skip-static $(doc-args)

# how to build projects
java-project := java -m fasterxml -o $(workdir)/java/target/generated-sources/reproto
js-project := js -o $(workdir)/js/generated
python-project := python -o $(workdir)/python/generated
rust-project := rust -o $(workdir)/rust/src --package-prefix generated

# base command invocations
reproto-cmd := $(REPROTO) $(REPROTO_ARGS)
reproto-compile := $(reproto-cmd) compile

list-inputs = $(shell ls -1 $(1))
diff-dirs = $(DIFF) -ur $(1) $(2)

define sync-dirs
	$(RM) $(2)
	$(CP) $(1) $(2)
endef

PHONY += all clean update
PHONY += projects clean-projects update-projects
PHONY += suites clean-suites update-suites

all: suites projects

clean: clean-projects clean-suites

update: update-suites update-projects

suites: $(suite-diffs)

clean-suites:
	$(RM) $(output)/suite-*

update-suites: $(suite-updates)

projects: $(project-diffs)

clean-projects:
	$(RM) $(workdir)
	$(RM) $(output)/project-*

update-projects: $(project-updates)

suite-build/%: $(REPROTO) $(output)/suite-%
	@echo "Suite: $*"
	$(reproto-compile) $($*-suite) -o $(output)/suite-$*

suite-update/%: $(expected)/suite-% suite-build/%
	@echo "Updating Suite: $*"
	$(call sync-dirs,$(output)/suite-$*,$(expected)/suite-$*)

suite-diff/%: $(expected)/suite-% suite-build/%
	@echo "Verifying Diffs: $*"
	$(call diff-dirs,$(expected)/suite-$*,$(output)/suite-$*)

project-build/%: $(REPROTO) $(input) $(output)/project-% $(workdir)/%
	@echo "Building Project: $*"
	$(reproto-compile) $($*-project)
	$(MAKE) -C $(workdir)/$*
	$(run-project) $(workdir)/$*/script.sh \
		$(foreach in,$(call list-inputs,$(input)),$(input)/$(in) $(output)/project-$*/$(in))

project-diff/%: $(expected)/project-% project-build/%
	@echo "Diffing Project: $*"
	$(call diff-dirs,$(expected)/project-$*,$(output)/project-$*)

project-update/%: $(expected)/project-% project-build/%
	@echo "Updating Project: $*"
	$(call sync-dirs,$(output)/project-$*,$(expected)/project-$*)

$(default-reproto):
	@echo "Building $(default-reproto)"
	cd $(ROOT) && $(CARGO) build

$(workdir)/%:
	$(call sync-dirs,../$*,$(workdir)/$*)

$(input):
	mkdir -p $@

$(expected)/suite-% $(expected)/project-% $(output)/suite-% $(output)/project-%:
	mkdir -p $@

.PHONY: $(PHONY)
