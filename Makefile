LIBNAME := muxed

all: src/main.rs src/*.rs
	rustc -o $@ $<

clean:
	rm -rf build/

test: src/main.rs
	@mkdir -p build
	rustc --test --dep-info build/$(notdir $<).d $< -o $@
