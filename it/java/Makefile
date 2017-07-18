MVN ?= mvn -q

ifeq ($(DEBUG),yes)
MVN := $(MVN) -X
else
MVN := $(MVN) -q
endif

all:
	@$(MVN) package
	@$(MVN) dependency:build-classpath -D mdep.outputFile=.classpath
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec java -cp $$(cat .classpath):$(CURDIR)/target/classes Test" >> script.sh
	@chmod +x script.sh
