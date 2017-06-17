MVN ?= mvn -q

all:
	@$(MVN) package
	@$(MVN) dependency:build-classpath -D mdep.outputFile=.classpath
	@echo "#!/usr/bin/env bash\nexec java -cp $$(cat .classpath):$(CURDIR)/target/classes Test" > script.sh
	@chmod +x script.sh
