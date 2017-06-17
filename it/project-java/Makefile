all:
	@./mvn-check
	@mvn -q package
	@mvn -q dependency:build-classpath -D mdep.outputFile=.classpath
	@echo "#!/usr/bin/env bash\nexec java -cp $$(cat .classpath):$(CURDIR)/target/classes Test" > script.sh
	@chmod +x script.sh
