# Makefile for RunPod SDK

ifneq (,$(wildcard ./.env))
	include .env
	export
endif

# Environment variables with defaults
RUNPOD_API_KEY ?=
RUNPOD_BASE_URL ?= https://rest.runpod.io/v1
RUNPOD_TIMEOUT_SECS ?= 30



# Make-level logger (evaluated by make; does not invoke the shell)
define make-log
$(info [$(shell date '+%Y-%m-%d %H:%M:%S')] [MAKE] [$(MAKECMDGOALS)] $(1))
endef

# Shell-level logger (expands to a printf that runs in the shell)
define shell-log
printf "[%s] [MAKE] [$(MAKECMDGOALS)] $(1)\n" "$$(date '+%Y-%m-%d %H:%M:%S')"
endef

.PHONY: help
help: ## Show this help message
	@echo "RunPod SDK Makefile"
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*## "; printf "\n"} /^[a-zA-Z_-]+:.*?## .*$$/ { printf "  %-20s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: setup
setup: install-tools env ## Complete project setup
	$(call make-log,Project setup complete!)

.PHONY: env
env: ## Create .env file from template
	$(call make-log,Setting up environment file...)
	@if [ ! -f .env ]; then \
		$(call shell-log,Creating .env from .env.example...); \
		cp .env.example .env; \
		$(call shell-log,.env file created. Please edit it with your actual values.); \
	else \
		$(call shell-log,.env file already exists.); \
	fi

.PHONY: install-tools
install-tools: ## Install required development tools
	$(call make-log,Installing development tools...)
	@# Check and install cargo-audit
	@if ! command -v cargo-audit >/dev/null 2>&1; then \
		$(call shell-log,Installing cargo-audit...); \
		cargo install cargo-audit --locked; \
		$(call shell-log,cargo-audit installed successfully.); \
	else \
		$(call shell-log,cargo-audit already available: $$(cargo audit --version)); \
	fi
	@# Check and install cargo-deny
	@if ! command -v cargo-deny >/dev/null 2>&1; then \
		$(call shell-log,Installing cargo-deny...); \
		cargo install cargo-deny --locked; \
		$(call shell-log,cargo-deny installed successfully.); \
	else \
		$(call shell-log,cargo-deny already available: $$(cargo deny --version)); \
	fi

.PHONY: check-env
check-env: ## Verify environment variables are set
	$(call make-log,Checking environment configuration...)
	@if [ -z "$(RUNPOD_API_KEY)" ]; then \
		$(call shell-log,ERROR: RUNPOD_API_KEY is not set. Please set it in .env file.); \
		exit 1; \
	else \
		$(call shell-log,RUNPOD_API_KEY is configured.); \
	fi
	@$(call shell-log,Base URL: $(RUNPOD_BASE_URL))
	@$(call shell-log,Timeout: $(RUNPOD_TIMEOUT_SECS) seconds)

.PHONY: test
test: check-env ## Run all tests with environment loaded
	$(call make-log,Running tests with environment loaded...)
	@cargo test

.PHONY: test-lib
test-lib: ## Run unit tests only (no API key required)
	$(call make-log,Running unit tests...)
	@cargo test --lib

.PHONY: test-integration
test-integration: check-env ## Run integration tests (requires API key)
	$(call make-log,Running integration tests...)
	@cargo test --test '*' -- --test-threads=1

.PHONY: examples
examples: check-env ## Run all examples
	$(call make-log,Running examples...)
	@$(call shell-log,Running basic_usage example...)
	@cargo run --example basic_usage
	@$(call shell-log,Running manage_endpoints example...)
	@cargo run --example manage_endpoints

.PHONY: security
security: install-tools ## Run security audits
	$(call make-log,Running security audits...)
	@$(call shell-log,Running cargo audit...)
	@cargo audit
	@$(call shell-log,Running cargo deny...)
	@cargo deny check

.PHONY: clean-env
clean-env: ## Remove .env file
	$(call make-log,Removing .env file...)
	@if [ -f .env ]; then \
		rm .env; \
		$(call shell-log,.env file removed.); \
	else \
		$(call shell-log,.env file does not exist.); \
	fi

.PHONY: doc-test
doc-test: ## Run documentation tests (may require API key)
	$(call make-log,Running documentation tests...)
	@cargo test --doc

.PHONY: verify
verify: ## Run core verification checks (no API key required)
	$(call make-log,Running verification checks...)
	@cargo fmt --check
	@cargo clippy --all-targets --all-features -- -D warnings
	@$(MAKE) test-lib
	@$(MAKE) security
	$(call make-log,All verification checks passed!)

.PHONY: verify-full
verify-full: verify doc-test ## Run all verification checks including doctests
	$(call make-log,All verification checks including doctests passed!)

.PHONY: ci
ci: verify ## Run CI pipeline (no API key required)
	$(call make-log,CI pipeline completed successfully!)

.PHONY: dev
dev: ## Development workflow: format, lint, and test
	$(call make-log,Running development workflow...)
	@cargo fmt
	@cargo clippy --all-targets --all-features -- -D warnings
	@$(MAKE) test
	$(call make-log,Development checks completed!)

# Default target
.DEFAULT_GOAL := help
