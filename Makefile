.POSIX:

.PHONY: default
default: build

.PHONY: dev
dev:
	LEPTOS_SITE_ADDR="0.0.0.0:8080" \
	LEPTOS_OUTPUT_NAME="my-leptos-app-$(shell tr -dc a-z0-9 </dev/urandom | head -c 10)" \
	cargo leptos watch

.PHONY: build
build:
	LEPTOS_SITE_ADDR="0.0.0.0:8080" \
	LEPTOS_OUTPUT_NAME="my-leptos-app-$(shell tr -dc a-z0-9 </dev/urandom | head -c 10)" \
	cargo leptos build --release -P

.PHONY: fmt
fmt:
	cargo fmt
	leptosfmt .
