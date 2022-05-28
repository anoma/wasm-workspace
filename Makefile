cargo = $(env) cargo

build:
	$(cargo) -Z unstable-options \
		build \
			--target wasm32-unknown-unknown \
			--out-dir build/debug/

build-release:
	$(cargo) -Z unstable-options \
		build \
			--release \
			--target wasm32-unknown-unknown \
			--out-dir build/release/

.PHONY : build build-release
