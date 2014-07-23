LIBNAME := $(shell rustc --print-file-name src/main.rs)

all: muxed

muxed: $(LIBNAME)

$(LIBNAME): src/main.rs src/*.rs
	rustc -o $@ $<
