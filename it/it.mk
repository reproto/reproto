script_input=$(CURDIR)/../tools/script-input
diff_projects=$(CURDIR)/../tools/diff-projects
update_projects=$(CURDIR)/../tools/update-projects

EXPECTED = expected
OUTPUT = output
PROTO_PATH = proto

JAVA_OUT = ${OUTPUT}/java
PYTHON_OUT = ${OUTPUT}/python
JS_OUT = ${OUTPUT}/js
RUST_OUT = ${OUTPUT}/rust

SUITES ?= python java js rust
TARGETS ?= test
FILTERED ?=

tool ?= cargo run -q --

python_args ?=
java_args ?= -m builder
js_args ?=
rust_args ?=

PACKAGES := $(TARGETS:%=--package %)
PROJECTS := $(SUITES:%=project-%)
FILTERED_PROJECTS := $(FILTERED_PROJECTS:%=project-%)
TARGET_PROJECTS := $(filter-out $(FILTERED_PROJECTS),$(PROJECTS))

.PHONY: all it update update-projects clean $(SUITES) $(PROJECTS)

all: clean it

it: ${SUITES}
	@echo "Verifying Diffs"
	@diff -ur $(EXPECTED) $(OUTPUT)

projects: ${TARGET_PROJECTS}
	@echo "Verifying Project Diffs"
	@${diff_projects}

update: ${SUITES}
	@rsync -ra $(OUTPUT)/ $(EXPECTED)/

update-projects:
	${update_projects}

clean:
	rm -rf project-*-workdir
	rm -rf project-*-output
	${RM} -rf output

python:
	@echo "Building Python"
	@${tool} compile -b python ${python_args} -o ${PYTHON_OUT} --path ${PROTO_PATH} ${PACKAGES}

project-python:
	@rsync -ra ../$@/ $@-workdir
	${tool} compile -b python -o $@-workdir/generated --path ${PROTO_PATH} ${PACKAGES}
	@cd $@-workdir && make
	@${script_input} $@-workdir/script.sh

rust:
	@echo "Building Rust"
	@${tool} compile -b rust ${python_args} -o ${RUST_OUT} --path ${PROTO_PATH} ${PACKAGES}

project-rust:

js:
	@echo "Building JavaScript"
	@${tool} compile -b js ${js_args} -o ${JS_OUT} --path ${PROTO_PATH} ${PACKAGES}

project-js:
	@rsync -ra ../$@/ $@-workdir
	${tool} compile -b js ${js_args} -o $@-workdir/generated --path ${PROTO_PATH} ${PACKAGES}
	@cd $@-workdir && make
	@${script_input} $@-workdir/script.sh

java:
	@echo "Building Java"
	@${tool} compile -b java ${java_args} -o ${JAVA_OUT} --path ${PROTO_PATH} ${PACKAGES}

project-java:
	@rsync -ra ../$@/ $@-workdir
	${tool} compile -b java -m fasterxml -o $@-workdir/target/generated-sources/reproto --path ${PROTO_PATH} ${PACKAGES}
	@cd $@-workdir && make
	@${script_input} $@-workdir/script.sh
