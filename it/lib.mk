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

suite-builds := $(suites:%=suite-build/%)
suite-diff := $(suites:%=suite-diff/%)
suite-update := $(suites:%=suite-update/%)

project-workdir := $(projects:%=$(workdir)/%)
project-script := $(projects:%=$(workdir)/%/script.sh)
project-run := $(projects:%=project-run/%)
project-output := $(projects:%=$(output)/project-%)
project-diff := $(projects:%=project-diff/%)
project-update := $(projects:%=project-update/%)

reproto-args := $(paths:%=--path %) $(targets:%=--package %)

# how to build suites
java-suite := java $(reproto-args) $(java-args)
js-suite := js $(reproto-args) $(js-args)
python-suite := python $(reproto-args) $(python-args)
rust-suite := rust $(reproto-args) $(rust-args)
doc-suite := doc $(reproto-args) --skip-static $(doc-args)

# how to build projects
java-project := java -m fasterxml $(reproto-args) -o $(workdir)/java/target/generated-sources/reproto
js-project := js $(reproto-args) -o $(workdir)/js/generated
python-project := python $(reproto-args) -o $(workdir)/python/generated
rust-project := rust $(reproto-args) -o $(workdir)/rust/src --package-prefix generated

# base command invocations
reproto-cmd := $(REPROTO) $(REPROTO_ARGS)
reproto-compile := $(reproto-cmd) compile

list-inputs = $(shell ls -1 $(1))
diff-dirs = $(DIFF) -ur $(1) $(2)

define sync-dirs
	$(RM) $(2)
	$(CP) $(1) $(2)
endef

.PHONY: all clean update
.PHONY: projects clean-projects update-projects
.PHONY: suites clean-suites update-suites
.PHONY: $(suite-builds) $(suite-update) $(suite-diff)
.PHONY: $(project-run) $(project-update) $(project-diff)

# treating script as phony will cause them to rebuild
ifeq ($(REBUILD),yes)
.PHONY: $(project-script)
endif

all: suites projects

clean: clean-projects clean-suites

update: update-suites update-projects

suites: $(suite-diff)

clean-suites:
	$(RM) $(output)/suite-*

update-suites: $(suite-update)

projects: $(project-diff)

clean-projects:
	$(RM) $(workdir)
	$(RM) $(output)/project-*

update-projects: $(project-run) $(project-update)

$(suite-builds): $(REPROTO)
	@echo "Suite: $(@F)"
	$(RM) $(output)/suite-$(@F)
	$(reproto-compile) $($(@F)-suite) -o $(output)/suite-$(@F)

$(suite-update): $(suite-builds)
	@echo "Updating Suite: $(@F)"
	$(call sync-dirs,$(output)/suite-$(@F),$(expected)/suite-$(@F))

$(suite-diff): $(suite-builds)
	@echo "Verifying Diff: $(@F)"
	$(call diff-dirs,$(expected)/suite-$(@F),$(output)/suite-$(@F))

$(project-workdir): $(workdir)
	$(call sync-dirs,../$(@F),$@)

$(project-script): $(REPROTO) $(input) $(project-workdir)
	@echo "Building Project: $(notdir $(@D))"
	$(reproto-compile) $($(notdir $(@D))-project)
	$(MAKE) -C $(@D)

$(project-run): $(project-script) $(project-output)
	@echo "Running Project: $(@F)"
	$(run-project) $(workdir)/$(@F)/script.sh \
		$(foreach in,$(call list-inputs,$(input)),$(input)/$(in) $(output)/project-$(@F)/$(in))

$(project-update): $(project-run)
	@echo "Updating Project: $(@F)"
	$(call sync-dirs,$(output)/project-$(@F),$(expected)/project-$(@F))

$(project-diff): $(project-run)
	@echo "Diffing Project: $(@F)"
	$(call diff-dirs,$(expected)/project-$(@F),$(output)/project-$(@F))

$(workdir) $(input) $(project-output):
	mkdir -p $@

$(default-reproto):
	@echo "Building $(default-reproto)"
	cd $(ROOT) && $(CARGO) build
