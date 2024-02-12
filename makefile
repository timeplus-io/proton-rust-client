# Make will use bash instead of sh
SHELL := /usr/bin/env bash

.PHONY: help
help:
	@echo '    make example     	Run the example code.'
	@echo '    make build   	Builds the code base incrementally (fast) for dev.'
	@echo '    make check   	Checks the code base for security vulnerabilities.'
	@echo '    make clean   	Cleans generated files and folders.'
	@echo '    make doc   		Builds, tests, and opens api docs in a browser.'
	@echo '    make fix   		Fixes linting issues as reported by clippy.'
	@echo '    make format   	Formats call code according to cargo fmt style.'
	@echo '    make install   	Tests and installs all make script dependencies.'
	@echo '    make release   	Builds the code base for release.'
	@echo '    make test   	Tests across all crates.'
	@echo '    make run   		Runs the default binary.'
	@echo '    make update   	Update rust, pull git, and build the project.'

# "---------------------------------------------------------"
# Development make targets
# "---------------------------------------------------------"

.PHONY: build
build:
	@source scripts/build.sh


.PHONY: check
check:
	@source scripts/check.sh


.PHONY: clean
clean:
	@source scripts/clean.sh


.PHONY: doc
doc:
	@source scripts/doc.sh


.PHONY: example
example:
	@source scripts/example.sh


.PHONY: fix
fix:
	@source scripts/fix.sh


.PHONY: format
format:
	@source scripts/format.sh


.PHONY: install
install:
	@source scripts/install_deps.sh


.PHONY: release
release:
	@source scripts/release.sh


.PHONY: run
run:
	@source scripts/run.sh



.PHONY: test
test:
	@source scripts/test.sh


.PHONY: update
update:
	@source scripts/update.sh


