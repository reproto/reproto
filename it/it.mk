project_js = $(CURDIR)/../project-js

EXPECTED = expected
OUTPUT = output
PROTO_PATH = proto

JAVA_OUT = ${OUTPUT}/java
PYTHON_OUT = ${OUTPUT}/python
JS_OUT = ${OUTPUT}/js
RUST_OUT = ${OUTPUT}/rust

SUITES ?= python java js rust
TOOL ?= cargo run -q --
TARGET ?= test

python_args ?=
java_args ?= -m builder
js_args ?=
rust_args ?=

.PHONY: all it update clean ${SUITES}

all: clean it

it: ${SUITES}
	make diffcheck
	make $(SUITES:%=%-input)

diffcheck:
	@echo "Verifying Diffs"
	@diff -ur $(EXPECTED) $(OUTPUT)

update: ${SUITES}
	@rsync -rav $(OUTPUT)/ $(EXPECTED)/
	git add $(EXPECTED)

clean:
	rm -rf project-*
	@${RM} -rf output

python:
	@echo "Building Python"
	@${TOOL} compile -b python ${python_args} -o ${PYTHON_OUT} --path ${PROTO_PATH} --package ${TARGET}

python-input:

rust:
	@echo "Building Rust"
	@${TOOL} compile -b rust ${python_args} -o ${RUST_OUT} --path ${PROTO_PATH} --package ${TARGET}

rust-input:

js:
	@echo "Building JavaScript"
	@${TOOL} compile -b js ${js_args} -o ${JS_OUT} --path ${PROTO_PATH} --package ${TARGET}

js-input:
	@rsync -rav ${project_js}/ ./project-js/
	cd ./js-project && make

java:
	@echo "Building Java"
	@${TOOL} compile -b java ${java_args} -o ${JAVA_OUT} --path ${PROTO_PATH} --package ${TARGET}

java-input:
