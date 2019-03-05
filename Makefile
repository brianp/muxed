.PHONY : test build clippy fmt clean check release osxbuild
.DEFAULT_GOAL := build

VERSION_TAG := $(shell git describe --abbrev=0 --tags)

local_path := $(shell pwd)

docker_image_name := brianp/muxed:dev

osx_image_name := brianp/muxed:osx

docker_dev_cmd := docker run -it -v "${local_path}:/usr/src/" -w "/usr/src/muxed" --rm ${docker_image_name}

clippy:
	${docker_dev_cmd} cargo clippy

test:
	${docker_dev_cmd} cargo test

fmt:
	${docker_dev_cmd} cargo fmt

clean:
	${docker_dev_cmd} cargo clean

check:
	${docker_dev_cmd} cargo check

release:
	${docker_dev_cmd} cargo build --release

osxrelease:
	docker run -it -v "${local_path}:/usr/src/" -w "/usr/src/muxed" --rm ${osx_image_name} cargo build --target x86_64-apple-darwin

dockerbuild:
	docker build -t ${docker_image_name} -f test.dockerfile .

osxdockerbuild:
	docker build -t ${osx_image_name} -f osx.dockerfile .

help:
	@echo test: run the tests
	@echo build: build the docker image
	@echo clippy: run the linter
	@echo fmt: run the rust code formatter
	@echo clean: clean the target directory
	@echo check: run the rust compiler check
