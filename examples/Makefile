.PHONY: heroic heroic_python heroic_java

reproto=${CURDIR}/../target/debug/reproto

PROTO_PATH=${CURDIR}
JAVA_PATH=${CURDIR}/target/java
JAVAC_OPTS=-target 1.8 -source 1.8 -cp ${JAVA_PATH} -d ${JAVA_PATH}
PYTHONPATH=${CURDIR}/target/python

PYTHON:=python3
JAVA:=java
JAVAC:=javac

all: heroic

heroic: heroic_python heroic_java

heroic_python: ${reproto}
	${reproto} --debug compile -b python -o ${PYTHONPATH} --path ${PROTO_PATH} heroic.v1
	PYTHONPATH=${PYTHONPATH} ${PYTHON} ${CURDIR}/python/example_heroic.py

heroic_java: ${reproto}
	${reproto} --debug compile -b java -m builder -o ${JAVA_PATH} --path ${PROTO_PATH} heroic.v1
	${JAVAC} ${JAVAC_OPTS} ${JAVA_PATH}/heroic/v1/Query.java ${CURDIR}/java/TestHeroic.java
	${JAVA} -cp ${JAVA_PATH} TestHeroic

${reproto}:
	(cd ${CURDIR}/.. && cargo build)
