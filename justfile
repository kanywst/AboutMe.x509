# AboutMe.x509 Justfile

# Run all checks (fmt, lint, test)
check: fmt lint test

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Generate coverage report (requires cargo-llvm-cov)
coverage:
    cargo llvm-cov --html

# Build release binary
build:
    cargo build --release
