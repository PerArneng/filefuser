

.PHONY: build
build: ## builds the project
	cargo build

.PHONY: test
test: ## runs the tests
	cargo test

.PHONY: build-release
build-release: ## builds the project in release mode
	cargo build --release

.PHONY: help
help: ## runs the makefile-help.sh script
	@sh .make/makefile-help.sh $(MAKEFILE_LIST)
