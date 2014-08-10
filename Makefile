LIBNAME := muxed

all: $(LIBNAME)

$(LIBNAME): src/main.rs src/*.rs
	rustc -o ./target/$@ $<

clean:
	rm -rf build/

test: src/main.rs
	@mkdir -p build
	rustc --test --dep-info build/$(notdir $<).d $< -o ./target/$@
