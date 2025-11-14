# Makefile for RunPod SDK

ifneq (,$(wildcard ./.env))
	include .env
	export
endif

# Environment variables with defaults
RUNPOD_API_KEY ?=
RUNPOD_BASE_URL ?= https://rest.runpod.io/v1
RUNPOD_GRAPHQL_URL ?= https://api.runpod.io/graphql
RUNPOD_TIMEOUT_SECS ?= 30

# Make-level logger
define make-log
$(info [$(shell date '+%Y-%m-%d %H:%M:%S')] [MAKE] [$(MAKECMDGOALS)] $(1))
endef

# Shell-level logger
define shell-log
printf "[%s] [MAKE] [$(MAKECMDGOALS)] $(1)\n" "$$(date '+%Y-%m-%d %H:%M:%S')"
endef

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
	@if ! command -v cargo-audit >/dev/null 2>&1; then \
		$(call shell-log,Installing cargo-audit...); \
		cargo install cargo-audit --locked; \
		$(call shell-log,cargo-audit installed successfully.); \
	else \
		$(call shell-log,cargo-audit already available: $$(cargo audit --version)); \
	fi
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
	@$(call shell-log,GraphQL URL: $(RUNPOD_GRAPHQL_URL))
	@$(call shell-log,Timeout: $(RUNPOD_TIMEOUT_SECS) seconds)

.PHONY: build
build: ## Build the project in release mode
	$(call make-log,Building project in release mode...)
	@cargo build --release

.PHONY: test
test: ## Run all tests (unit, integration, and doc tests) with environment loaded
	$(call make-log,Running all tests with environment loaded...)
	@RUNPOD_API_KEY="$(RUNPOD_API_KEY)" \
	RUNPOD_BASE_URL="$(RUNPOD_BASE_URL)" \
	RUNPOD_GRAPHQL_URL="$(RUNPOD_GRAPHQL_URL)" \
	RUNPOD_TIMEOUT_SECS="$(RUNPOD_TIMEOUT_SECS)" \
	cargo test

.PHONY: examples
examples: check-env ## Run all examples
	$(call make-log,Running examples...)
	@for example in basic_usage manage_endpoints manage_pods; do \
		$(call shell-log,Running $$example example...); \
		RUNPOD_API_KEY="$(RUNPOD_API_KEY)" \
		RUNPOD_BASE_URL="$(RUNPOD_BASE_URL)" \
		RUNPOD_GRAPHQL_URL="$(RUNPOD_GRAPHQL_URL)" \
		RUNPOD_TIMEOUT_SECS="$(RUNPOD_TIMEOUT_SECS)" \
		cargo run --example $$example || exit 1; \
	done

.PHONY: fmt
fmt: ## Format code
	$(call make-log,Formatting code...)
	@cargo fmt

.PHONY: clippy
clippy: ## Run clippy lints
	$(call make-log,Running clippy...)
	@RUNPOD_API_KEY="$(RUNPOD_API_KEY)" \
	RUNPOD_BASE_URL="$(RUNPOD_BASE_URL)" \
	RUNPOD_GRAPHQL_URL="$(RUNPOD_GRAPHQL_URL)" \
	RUNPOD_TIMEOUT_SECS="$(RUNPOD_TIMEOUT_SECS)" \
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: run-tools
run-tools: install-tools ## Run security audits and other tools
	$(call make-log,Running security audits and tools...)
	@$(call shell-log,Running cargo audit...)
	@cargo audit
	@$(call shell-log,Running cargo deny...)
	@cargo deny check

.PHONY: doc
doc: ## Generate and open documentation
	$(call make-log,Generating and opening documentation...)
	@RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open

.PHONY: dev
dev: fmt clippy test ## Development workflow: format, lint, and test
	$(call make-log,Development checks completed!)
