default-reproto := $(CURDIR)/target/debug/reproto

ifeq ($(filter all make,$(DEBUG)),)
make-args := --no-print-directory -s
endif

ifneq ($(filter all reproto,$(DEBUG)),)
override REPROTO_FLAGS := --debug
endif

# set IT="<dir>" to limit which modules to build
IT ?= $(wildcard it/test-*)

define \n


endef

define it-target-body
$(1) += $(1)/$(2)
$(1)/$(2): $$(REPROTO)
	$$(MAKE) $$(make-args) -f $$(CURDIR)/tools/Makefile.it -C $(2) $(1)
endef

define it-target-default
$(or $(2),$(1)): $$($(1))
.PHONY: $(1) $$($(1))
endef

define it-target
$(eval \
	$(foreach i,$(IT),\
		$(call it-target-body,$(1),$(i)) $(\n)) \
	$(call it-target-default,$(1),$(2)) $(\n))
endef

test-cmd = $(2) 1> /dev/null 2>&1 && echo $(1) || echo "disabled: $(1)" >&2;

export REPROTO_FLAGS
export PYTHON ?= python
export PYTHON3 ?= python3
export REPROTO ?= $(default-reproto)

define check-deps
$(call test-cmd,java,mvn --version)
$(call test-cmd,python,$(PYTHON) --version)
$(call test-cmd,python3,$(PYTHON3) --version)
$(call test-cmd,rust,cargo --version)
$(call test-cmd,js,node --version \&\& babel --version)
endef

export PROJECTS := $(shell $(call check-deps))

.PHONY: all update tests clean
.PHONY: suites update-suites clean-suites
.PHONY: projects update-projects clean-projects

all: suites projects

update: update-suites update-projects

tests:
	cargo test --all

clean: it-clean
	cargo clean

$(call it-target,clean,it-clean)
$(call it-target,suites)
$(call it-target,update-suites)
$(call it-target,clean-suites)
$(call it-target,projects)
$(call it-target,update-projects)
$(call it-target,clean-projects)

$(default-reproto): $(CURDIR)/cli/Cargo.toml
	@echo "Building: $@"
	cargo build --manifest-path cli/Cargo.toml

help:
	@echo ""
	@echo "Please read 'Suites & Projects' in README.md"
	@echo ""
	@echo "Variables (specified like 'make VARIABLE=VALUE <target>'):"
	@echo "  PROJECTS=foo     - only build the listed kinds of projects"
	@echo "  DEBUG=all        - (very) verbose output"
	@echo "  DEBUG=reproto    - debug ReProto"
	@echo "  DEBUG=mvn        - debug Maven"
	@echo "  IT=it/test-basic - only build the specifiec integration tests"
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
