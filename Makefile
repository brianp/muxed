.PHONY : clean doc test build

clean:
	cargo clean

doc:
	cargo rustdoc -- --no-defaults --passes "collapse-docs" --passes "unindent-comments"

test:
	cargo test

build:
	cargo build

help:
	@echo doc: create public doc
	@echo clean: remove target folder
	@echo test: run the spec_suite
	@echo build: build the project
