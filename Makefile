.PHONY: build clean

build:
	cargo build --release --all-targets

clean:
	cargo clean
