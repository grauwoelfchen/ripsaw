build:
	cargo build
.PHONY: build

build\:release:
	cargo build --release --bin ripsaw \
	  --target x86_64-unknown-linux-musl
.PHONY: build\:release

.DEFAULT_GOAL = build
