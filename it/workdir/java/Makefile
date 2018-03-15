MVN ?= mvn

ifneq ($(filter all mvn,$(DEBUG)),)
override MVN_FLAGS += -X
else
override MVN_FLAGS += -q
endif

default:
	@$(MVN) $(MVN_FLAGS) package
	@$(MVN) $(MVN_FLAGS) dependency:build-classpath -D mdep.outputFile=.classpath
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec java -cp $$(cat .classpath):$(CURDIR)/target/classes Test" >> script.sh
	@chmod +x script.sh
