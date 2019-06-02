.PHONY : build dockerup dockerdown cargo explain osxrelease dockerbuild osxdockerbuild
.DEFAULT_GOAL := build

VERSION_TAG := $(shell git describe --abbrev=0 --tags)

local_path := $(shell pwd)

docker_image_name := brianp/muxed:dev

docker_instance_name := muxed.dev

osx_image_name := brianp/muxed:osx

docker_dev_cmd := docker exec ${docker_instance_name}

build:
	${docker_dev_cmd} cargo build

dockerup:
	docker run -d -it -v "${local_path}:/usr/src/" -w "/usr/src/muxed" --name ${docker_instance_name} --rm ${docker_image_name}

dockerdown:
	docker stop ${docker_instance_name}

cargo:
	${docker_dev_cmd} cargo $(filter-out $@,$(MAKECMDGOALS))

explain:
	${docker_dev_cmd} rustc --explain $(filter-out $@,$(MAKECMDGOALS))

osxrelease:
	docker run -it -v "${local_path}:/usr/src/" -w "/usr/src/muxed" --rm ${osx_image_name} cargo build --target x86_64-apple-darwin

dockerbuild:
	docker build -t ${docker_image_name} -f dev.dockerfile .

osxdockerbuild:
	docker build -t ${osx_image_name} -f osx.dockerfile .

help:
	@echo build: build muxed debug binary
	@echo dockerup: run the docker development instance
	@echo dockerdown: stop the running development instance
	@echo cargo: run cargo commands inside development instance
	@echo explain: use the rustc --explain command
	@echo osxrelease: build the release binary for osx
	@echo dockerbuild: build the development environment
	@echo osxdockerbuild: build the osx release environment
