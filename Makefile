

.PHONY: build
build: ## builds the project
	cargo build

.PHONY: test
test: ## runs the tests
	cargo test

.PHONY: help
help: ## runs the makefile-help.sh script
	@sh .make/makefile-help.sh $(MAKEFILE_LIST)
