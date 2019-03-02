.PHONY : clean doc test build

.DEFAULT_GOAL := build

VERSION_TAG := $(shell git describe --abbrev=0 --tags)

clean:
	cargo clean

doc:
	cargo rustdoc -- --no-defaults --passes "collapse-docs" --passes "unindent-comments"

test:
	cargo test

build:
	cargo build

release: target/release/muxednew
target/release/muxednew:
	cargo build --release

releaseosx: target/release/x86_64-apple-darwin/muxednew
target/release/x86_64-apple-darwin/muxednew:
	cargo build --release --target x86_64-apple-darwin

help:
	@echo doc: create public doc
	@echo clean: remove target folder
	@echo test: run the tests
	@echo build: build the project
	@echo release: build the project with --release
