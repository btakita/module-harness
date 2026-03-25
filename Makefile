.PHONY: build release test clippy check install install-hooks clean

# Build debug binary
build:
	cargo build

# Build release binary and symlink to .bin/
release:
	cargo build --release
	@mkdir -p .bin
	@ln -sf ../target/release/module-harness .bin/module-harness
	@echo "Installed .bin/module-harness -> target/release/module-harness"

# Run tests
test:
	cargo test

# Lint
clippy:
	cargo clippy -- -D warnings

# clippy + test
check: clippy test

# Install to ~/.cargo/bin
install:
	cargo install --path .

# Install git hooks
install-hooks:
	@mkdir -p .git/hooks
	@printf '#!/bin/sh\nmake check\n' > .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Installed .git/hooks/pre-commit"

# Publish to crates.io
publish:
	cargo publish

# Remove build artifacts
clean:
	cargo clean
	rm -f .bin/module-harness
