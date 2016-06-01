.PHONY : clean doc test build packageall cleanpkgs

.DEFAULT_GOAL := build

clean:
	cargo clean

doc:
	cargo rustdoc -- --no-defaults --passes "collapse-docs" --passes "unindent-comments"

test:
	cargo test

build:
	cargo build

cleanpkgs:
	rm target/pkgs/*

packageall: package packageosx

package: target/pkgs/muxed-x86_64-unknown-linux-gnu.tar.gz
target/pkgs/muxed-x86_64-unknown-linux-gnu.tar.gz:
	mkdir -p target/pkgs/
	cd target/release && tar -cvzf muxed-x86_64-unknown-linux-gnu.tar.gz muxed
	mv target/release/muxed-x86_64-unknown-linux-gnu.tar.gz target/pkgs/

packageosx: target/pkgs/muxed-x86_64-apple-darwin.tar.gz
target/pkgs/muxed-x86_64-apple-darwin.tar.gz:
	mkdir -p target/pkgs/
	cd target/x86_64-apple-darwin/release && tar -cvzf muxed-x86_64-apple-darwin.tar.gz muxed
	mv target/x86_64-apple-darwin/release/muxed-x86_64-apple-darwin.tar.gz target/pkgs/

help:
	@echo doc: create public doc
	@echo clean: remove target folder
	@echo test: run the spec_suite
	@echo build: build the project
	@echo cleanpkgs: clean the target/pkgs/ directory
	@echo packageall: build both default and osx packages
	@echo package: package the default binary into .tar for distribution
	@echo packageosx: package the osx binary into .tar for distribution
