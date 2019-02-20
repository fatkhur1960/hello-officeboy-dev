
PROJ_DIR=$(shell pwd)

PUBLIC_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/public-api.html
LIBRARY_DOC_OUTPUT=$(PROJ_DIR)/target/doc/apf/index.html
DATABASE_TEST_URL=postgresql://postgres@localhost/apf_test

docs: prepare lib-docs api-docs

prepare:
	mkdir -p target/api-docs

lib-docs:
	@@echo generating library documentation...
	@@cargo doc --package apf --no-deps --lib
	@@echo generated: $(LIBRARY_DOC_OUTPUT)

api-docs: prepare api-docs/public-api.md
	@@echo generating API documentation...
	@@cd api-docs && aglio -i public-api-gen.md -o $(PUBLIC_API_DOC_OUTPUT)
	@@echo generated: $(PUBLIC_API_DOC_OUTPUT)

fmt:
	cd testkit && cargo fmt
	cd macros/apf_proc_macro && cargo fmt
	cargo fmt

test:
	@@echo Testing...
	@@DATABASE_URL=$(DATABASE_TEST_URL) cargo test

test-dev:
	@@echo Testing...
	@@DATABASE_URL=$(DATABASE_TEST_URL) cargo test -- --nocapture

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

release-linux:
	@@echo ""
	@@echo Ini akan melakukan build menggunakan Docker, 
	@@echo nantinya output binary bisa ditemukan di target/x86_64-unknown-linux-musl/release
	@@echo Building for musl Linux...
	@@docker run -it --rm -v $(PROJ_DIR):/workdir \
					-v /tmp:/root/.cargo/git \
					-v /tmp:/root/.cargo/registry \
					anvie/rust-musl-build:latest \
					cargo build --release --target=x86_64-unknown-linux-musl

test-env:
	diesel database reset --database-url $(DATABASE_TEST_URL)
	diesel migration run --database-url $(DATABASE_TEST_URL)

test-env-redo:
	diesel migration redo --database-url $(DATABASE_TEST_URL)

reset-db:
	diesel database reset

.PHONY: prepare docs lib-docs api-docs fmt \
		test test-dev lint audit commit \
		release test-env test-env-redo release-linux \
		reset-db
