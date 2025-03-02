

.PHONY: build
build: ## builds the project
	cargo build

.PHONY: test
test: ## runs the tests
	cargo test

.PHONY: build-release
build-release: ## builds the project in release mode
	cargo build --release

.PHONY: tag-and-push
tag-and-push: ## creates and pushes a tag based on the version in Cargo.toml
	@version=$$(grep '^version =' Cargo.toml | cut -d '"' -f 2); \
	echo "Tagging and pushing version $$version"; \
	git tag -a v$$version -m "Version based on Cargo.toml"; \
	git push origin v$$version

.PHONY: help
help: ## runs the makefile-help.sh script
	@sh .make/makefile-help.sh $(MAKEFILE_LIST)
