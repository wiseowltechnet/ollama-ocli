# Rust QA Tools (Gradle Equivalent)

## Core Tools (Built into Cargo)

### 1. **cargo fmt** (like Spotless/Google Java Format)
```bash
cargo fmt              # Format code
cargo fmt -- --check   # Check formatting (CI)
```

### 2. **cargo clippy** (like SpotBugs/PMD/Checkstyle)
```bash
cargo clippy                    # Lint code
cargo clippy -- -D warnings     # Fail on warnings
cargo clippy --fix              # Auto-fix issues
```

### 3. **cargo test** (like JUnit)
```bash
cargo test              # Run all tests
cargo test --release    # Test optimized build
cargo test -- --nocapture  # Show println output
```

### 4. **cargo bench** (like JMH)
```bash
cargo bench             # Run benchmarks
```

## Additional QA Tools

### 5. **cargo-audit** (like OWASP Dependency Check)
```bash
cargo install cargo-audit
cargo audit             # Check for security vulnerabilities
```

### 6. **cargo-tarpaulin** (like JaCoCo)
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html  # Code coverage
```

### 7. **cargo-watch** (like Gradle continuous build)
```bash
cargo install cargo-watch
cargo watch -x test     # Auto-run tests on change
cargo watch -x run      # Auto-run on change
```

### 8. **cargo-deny** (like Gradle dependency verification)
```bash
cargo install cargo-deny
cargo deny check        # Check licenses, bans, advisories
```

### 9. **cargo-outdated** (like Gradle dependency updates)
```bash
cargo install cargo-outdated
cargo outdated          # Check for outdated dependencies
```

### 10. **cargo-bloat** (like Gradle build scan)
```bash
cargo install cargo-bloat
cargo bloat --release   # Analyze binary size
```

## Build Tools

### **Make** (Traditional)
```bash
make qa                 # Run QA pipeline
make ci                 # Run CI pipeline
make build              # Build release
```

### **Just** (Modern Alternative)
```bash
cargo install just
just qa                 # Run QA pipeline
just watch              # Watch and test
just coverage           # Code coverage
```

## CI/CD Integration

### GitHub Actions (Already configured)
```yaml
# .github/workflows/ci.yml
- run: cargo fmt -- --check
- run: cargo clippy -- -D warnings
- run: cargo test
- run: cargo build --release
```

### Pre-commit Hooks
```bash
# Install
cargo install cargo-husky

# .cargo/husky/pre-commit
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Quick Commands

```bash
# Format everything
make fmt

# Run full QA
make qa

# CI pipeline
make ci

# Or with just
just qa
just ci
```

## Comparison to Gradle

| Gradle Plugin | Rust Tool | Command |
|---------------|-----------|---------|
| Spotless | cargo fmt | `cargo fmt` |
| Checkstyle | cargo clippy | `cargo clippy` |
| SpotBugs | cargo clippy | `cargo clippy` |
| JUnit | cargo test | `cargo test` |
| JaCoCo | cargo-tarpaulin | `cargo tarpaulin` |
| OWASP Check | cargo-audit | `cargo audit` |
| JMH | cargo bench | `cargo bench` |
| Gradle Scan | cargo-bloat | `cargo bloat` |

## Recommended Workflow

1. **Before commit**:
   ```bash
   make qa
   ```

2. **During development**:
   ```bash
   just watch
   ```

3. **Before release**:
   ```bash
   make ci
   cargo audit
   cargo tarpaulin
   ```

## Install All Tools

```bash
# Core (included with Rust)
rustup component add rustfmt clippy

# Additional
cargo install cargo-audit
cargo install cargo-tarpaulin
cargo install cargo-watch
cargo install cargo-deny
cargo install cargo-outdated
cargo install cargo-bloat
cargo install just
```

---

**OCLI uses**: Makefile + justfile + GitHub Actions for comprehensive QA! 🚀
