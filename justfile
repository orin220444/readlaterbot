default:
	cargo build
release:
	cargo build --release
raspberry:
	cross build --release --target aarch64-unknown-linux-gnu
bump_deps:
	cargo update
