
PROJ_DIR=$(shell pwd)

PUBLIC_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/public-api.html
LIBRARY_DOC_OUTPUT=$(PROJ_DIR)/target/doc/apf/index.html

docs: prepare lib-docs api-docs

prepare:
	mkdir -p target/api-docs

lib-docs:
	@@echo generating library documentation...
	@@cargo doc --package apf --no-deps --lib
	@@echo generated: $(LIBRARY_DOC_OUTPUT)

api-docs: prepare api-docs/public-api.md
	@@echo generating API documentation...
	@@cd api-docs && aglio -i public-api.md -o $(PUBLIC_API_DOC_OUTPUT)
	@@echo generated: $(PUBLIC_API_DOC_OUTPUT)

fmt:
	cd testkit && cargo fmt
	cd macros/apf_proc_macro && cargo fmt
	cargo fmt

test:
	@@echo Testing...
	@@source .env && cargo test

test-dev:
	@@echo Testing...
	@@cargo test -- --nocapture

lint:
	@@echo Linting...
	@@cargo clippy

audit:
	@@echo Auditing...
	@@cargo audit

commit:
	@@echo Committing...
	@@make fmt
	@@cargo check
	@@git ci -a

release:
	@@echo Build release mode...
	@@cargo build --release

test-env:
	diesel setup --database-url postgresql://localhost/apf_test?sslmode=disable

.PHONY: prepare docs lib-docs api-docs fmt \
		test test-dev lint audit commit \
		release test-env
