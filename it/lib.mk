M := $(notdir $(CURDIR))
M := $(M:test-%=%)

ifeq ($(ROOT),)
$(error "ROOT: missing variable")
endif

ifeq ($(PROJECTS),)
$(error "PROJECTS: missing variable")
endif

ifeq ($(M),)
$(error "M: missing variable")
endif

reproto-args := $(REPROTO_ARGS)

ifeq ($(DEBUG),yes)
reproto-args := $(reproto-args) --debug
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

compile-args := $(paths:%=--path %) $(targets:%=--package %)

# how to build suites
java-suite := java $(compile-args) $(java-args)
js-suite := js $(compile-args) $(js-args)
python-suite := python $(compile-args) $(python-args)
rust-suite := rust $(compile-args) $(rust-args)
doc-suite := doc $(compile-args) --skip-static $(doc-args)

# how to build projects
java-project := java -m fasterxml $(compile-args) -o $(workdir)/java/target/generated-sources/reproto
js-project := js $(compile-args) -o $(workdir)/js/generated
python-project := python $(compile-args) -o $(workdir)/python/generated
rust-project := rust $(compile-args) -o $(workdir)/rust/src --package-prefix generated

# base command invocations
reproto-cmd := $(REPROTO) $(reproto-args)
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
	@echo "$(M): Suite: $(@F)"
	$(RM) $(output)/suite-$(@F)
	$(reproto-compile) $($(@F)-suite) -o $(output)/suite-$(@F)

$(suite-update): $(suite-builds)
	@echo "$(M): Updating Suite: $(@F)"
	$(call sync-dirs,$(output)/suite-$(@F),$(expected)/suite-$(@F))

$(suite-diff): $(suite-builds)
	@echo "$(M): Verifying Diff: $(@F)"
	$(call diff-dirs,$(expected)/suite-$(@F),$(output)/suite-$(@F))

$(project-workdir): $(workdir)
	$(call sync-dirs,../$(@F),$@)

$(project-script): $(REPROTO) $(input) $(project-workdir)
	@echo "$(M): Building Project: $(notdir $(@D))"
	$(reproto-compile) $($(notdir $(@D))-project)
	$(MAKE) -C $(@D)

$(project-run): $(project-script) $(project-output)
	@echo "$(M): Running Project: $(@F)"
	$(run-project) $(workdir)/$(@F)/script.sh \
		$(foreach in, \
			$(call list-inputs,$(input)),$(input)/$(in) $(output)/project-$(@F)/$(in))

$(project-update): $(project-run)
	@echo "$(M): Updating Project: $(@F)"
	$(call sync-dirs,$(output)/project-$(@F),$(expected)/project-$(@F))

$(project-diff): $(project-run)
	@echo "$(M): Diffing Project: $(@F)"
	$(call diff-dirs,$(expected)/project-$(@F),$(output)/project-$(@F))

$(workdir) $(input) $(project-output):
	mkdir -p $@

$(default-reproto):
	@echo "Building $(default-reproto)"
	cd $(ROOT) && $(CARGO) build
