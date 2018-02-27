all:
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "dotnet build $$PWD >&2" >> script.sh
	@echo "exec dotnet run --project $$PWD" >> script.sh
	chmod +x script.sh
