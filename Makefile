.PHONY: clean
default: clean

clean:
	cargo fmt && cargo clippy