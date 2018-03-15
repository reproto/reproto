default:
	@dotnet build $(CURDIR)
	@echo "#!/usr/bin/env bash" > script.sh
	@echo "exec dotnet run --project $$PWD" >> script.sh
	@chmod +x script.sh
