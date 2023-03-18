.PHONY: build
build:
	cargo build --release
	rm -f ./lua/goto.so
	cp ./target/release/libgoto.dylib ./lua/goto.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
