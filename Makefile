.PHONY: build
build:
	cargo build --release
	rm -f ./lua/libmy_module.so
	cp ./target/release/libmy_module.dylib ./lua/libmy_module.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
