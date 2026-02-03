# Just is a modern alternative to Make
# Install: cargo install just

# Default recipe
default: fmt lint test

# Format code
fmt:
    @echo "✨ Formatting..."
    cargo fmt

# Check formatting
fmt-check:
    @echo "📋 Checking format..."
    cargo fmt -- --check

# Lint with clippy
lint:
    @echo "🔍 Linting..."
    cargo clippy -- -D warnings

# Run tests
test:
    @echo "🧪 Testing..."
    cargo test

# Build release
build:
    @echo "🔨 Building..."
    cargo build --release

# Security audit
audit:
    @echo "🔒 Auditing..."
    cargo audit || echo "Install: cargo install cargo-audit"

# Code coverage
coverage:
    @echo "📊 Coverage..."
    cargo tarpaulin --out Html || echo "Install: cargo install cargo-tarpaulin"

# Benchmark
bench:
    @echo "⚡ Benchmarking..."
    cargo bench

# Full QA pipeline
qa: fmt-check lint test audit
    @echo "✅ QA complete!"

# CI pipeline
ci: fmt-check lint test build
    @echo "✅ CI complete!"

# Watch and test
watch:
    @echo "👀 Watching..."
    cargo watch -x test || echo "Install: cargo install cargo-watch"

# Clean
clean:
    @echo "🧹 Cleaning..."
    cargo clean

# Install
install:
    @echo "📦 Installing..."
    cargo install --path .
