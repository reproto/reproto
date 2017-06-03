EXPECTED = ./expected
OUTPUT = ./output

PROTO_PATH = ${CURDIR}/proto
JAVA_OUT = ${OUTPUT}/java
PYTHON_OUT = ${OUTPUT}/python

SUITES = python java

TOOL ?= cargo run -q --
TARGET ?= test

.PHONY: all it update clean ${SUITES}

all: clean it

it: ${SUITES}
	diff -ur $(EXPECTED) $(OUTPUT)

update: ${SUITES}
	@rsync -rav $(OUTPUT)/ $(EXPECTED)/
	git add $(EXPECTED)

clean:
	${RM} -rf output

python:
	${TOOL} compile -b python -o ${PYTHON_OUT} --path ${PROTO_PATH} --package ${TARGET}

java:
	${TOOL} compile -b java -m builder -o ${JAVA_OUT} --path ${PROTO_PATH} --package ${TARGET}
