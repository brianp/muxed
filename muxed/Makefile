.PHONY : clean doc test build packageall cleanpkgs

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

release: target/release/muxed
target/release/muxed:
	cargo build --release

releaseosx: target/x86_64-apple-darwin/release/muxed
target/x86_64-apple-darwin/release/muxed:
	cargo build --release --target x86_64-apple-darwin

cleanpkgs:
	rm target/pkgs/*

cpmuxednew: target/release/muxednew
target/release/muxednew:
	cp ../muxednew/target/release/muxednew ./target/release/

cpmuxednewosx: target/x86_64-apple-darwin/release/muxednew
target/x86_64-apple-darwin/release/muxednew:
	cp ../muxednew/target/x86_64-apple-darwin/release/muxednew ./target/x86_64-apple-darwin/release/

packageall: package packageosx

package: release cpmuxednew target/pkgs/muxed-$(VERSION_TAG)-x86_64-unknown-linux-gnu.tar.gz
target/pkgs/muxed-$(VERSION_TAG)-x86_64-unknown-linux-gnu.tar.gz:
	mkdir -p target/pkgs/
	cd target/release && tar -cvzf muxed-$(VERSION_TAG)-x86_64-unknown-linux-gnu.tar.gz muxed
	mv target/release/muxed-$(VERSION_TAG)-x86_64-unknown-linux-gnu.tar.gz target/pkgs/
	cd target/release && tar -cvzf muxed-complete-$(VERSION_TAG)-x86_64-unknown-linux-gnu.tar.gz muxed muxednew
	mv target/release/muxed-complete-$(VERSION_TAG)-x86_64-unknown-linux-gnu.tar.gz target/pkgs/

packageosx: releaseosx cpmuxednewosx target/pkgs/muxed-$(VERSION_TAG)-x86_64-apple-darwin.tar.gz
target/pkgs/muxed-$(VERSION_TAG)-x86_64-apple-darwin.tar.gz:
	mkdir -p target/pkgs/
	cd target/x86_64-apple-darwin/release && tar -cvzf muxed-$(VERSION_TAG)-x86_64-apple-darwin.tar.gz muxed
	mv target/x86_64-apple-darwin/release/muxed-$(VERSION_TAG)-x86_64-apple-darwin.tar.gz target/pkgs/
	cd target/x86_64-apple-darwin/release && tar -cvzf muxed-complete-$(VERSION_TAG)-x86_64-apple-darwin.tar.gz muxed muxednew
	mv target/x86_64-apple-darwin/release/muxed-complete-$(VERSION_TAG)-x86_64-apple-darwin.tar.gz target/pkgs/

help:
	@echo doc: create public doc
	@echo clean: remove target folder
	@echo test: run the tests
	@echo build: build the project
	@echo release: build the project with --release
	@echo cleanpkgs: clean the target/pkgs/ directory
	@echo packageall: build both default and osx packages
	@echo package: package the default binary into .tar for distribution
	@echo packageosx: package the osx binary into .tar for distribution
