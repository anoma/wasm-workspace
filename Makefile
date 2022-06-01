cargo = $(env) cargo

debug:
	$(cargo) -Z unstable-options \
		build \
			--target wasm32-unknown-unknown \
			--out-dir build/debug/

release:
	$(cargo) -Z unstable-options \
		build \
			--release \
			--target wasm32-unknown-unknown \
			--out-dir build/release/

.PHONY : debug release
