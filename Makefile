build:
	RUSTFLAGS=-Clinker=musl-gcc \
		rustc src/main.rs --target x86_64-unknown-linux-musl -o ripsaw
.PHONY: build

.DEFAULT_GOAL = build
