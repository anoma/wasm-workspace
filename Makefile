cargo = $(env) cargo

debug:
	RUSTUP_TOOLCHAIN="nightly-2022-05-20" $(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--exclude 'e2e_tests' \
			--target wasm32-unknown-unknown \
			--target-dir build/cache/wasm32-unknown-unknown \
			--out-dir build/debug/

release:
	RUSTUP_TOOLCHAIN="nightly-2022-05-20" $(cargo) -Z unstable-options \
		build \
			--workspace \
			--exclude 'shared' \
			--exclude 'e2e_tests' \
			--release \
			--target wasm32-unknown-unknown \
			--target-dir build/cache/wasm32-unknown-unknown \
			--out-dir build/release/

e2e-test-binaries:
	RUSTUP_TOOLCHAIN="nightly-2022-05-20" $(cargo) -Z unstable-options \
		build \
			--target x86_64-unknown-linux-musl \
			--target-dir build/cache/x86_64-unknown-linux-musl \
			--package 'e2e_tests' \
			--out-dir build/tests

docker: debug e2e-test-binaries
	docker compose build

.PHONY : debug release docker
