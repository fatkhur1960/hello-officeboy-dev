
PROJ_DIR=$(shell pwd)

PUBLIC_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/public-api.html
LIBRARY_DOC_OUTPUT=$(PROJ_DIR)/target/doc/payment/index.html

docs: prepare lib-docs api-docs

prepare:
	mkdir -p target/api-docs

lib-docs:
	@@echo generating library documentation...
	@@cargo doc --package payment --no-deps --lib
	@@echo generated: $(LIBRARY_DOC_OUTPUT)

api-docs: api-docs/public-api.md
	@@echo generating API documentation...
	@@cd api-docs && aglio -i public-api.md -o $(PUBLIC_API_DOC_OUTPUT)
	@@echo generated: $(PUBLIC_API_DOC_OUTPUT)

fmt:
	cd testkit && cargo fmt
	cargo fmt

test:
	@@echo Testing...
	@@cargo test

.PHONY: prepare docs lib-docs api-docs fmt test
