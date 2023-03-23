.PHONY: build
build:
	cargo build --release
	rm -f ./lua/goto_backend.so
	[[ -e ./target/release/libgoto.dylib ]] && cp ./target/release/libgoto.dylib ./lua/goto_backend.so || cp ./target/release/libgoto.so ./lua/goto_backend.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
