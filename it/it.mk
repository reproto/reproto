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
tool ?= cargo run -q --
TARGET ?= test

python_args ?=
java_args ?= -m builder
js_args ?=
rust_args ?=

.PHONY: all it update clean ${SUITES}

all: clean it

it: ${SUITES}
	make $(SUITES:%=project-%)
	make diffcheck

diffcheck:
	@echo "Verifying Diffs"
	@diff -ur $(EXPECTED) $(OUTPUT)
	${diff_projects}

update: ${SUITES}
	@rsync -rav $(OUTPUT)/ $(EXPECTED)/
	${update_projects}

clean:
	rm -rf project-*-workdir
	rm -rf project-*-output
	${RM} -rf output

python:
	@echo "Building Python"
	@${tool} compile -b python ${python_args} -o ${PYTHON_OUT} --path ${PROTO_PATH} --package ${TARGET}

project-python:

rust:
	@echo "Building Rust"
	@${tool} compile -b rust ${python_args} -o ${RUST_OUT} --path ${PROTO_PATH} --package ${TARGET}

project-rust:

js:
	@echo "Building JavaScript"
	@${tool} compile -b js ${js_args} -o ${JS_OUT} --path ${PROTO_PATH} --package ${TARGET}

project-js:
	rsync -rav ../$@/ $@-workdir
	${tool} compile -b js ${js_args} -o $@-workdir/generated --path ${PROTO_PATH} --package ${TARGET}
	@cd $@-workdir && make
	@${script_input} $@-workdir/script.sh

java:
	@echo "Building Java"
	@${tool} compile -b java ${java_args} -o ${JAVA_OUT} --path ${PROTO_PATH} --package ${TARGET}

project-java:
