set dotenv-load

# List all just recipes
default:
    @just --list

# Install require tools
requirements:
    rustup toolchain add nightly
    cargo install cargo-watch cargo-audit cargo-deny cargo-msrv cargo-semver-checks
    cargo-sort 
    cargo install cargo-nextest

# Run TDD mode
tdd:
    cargo watch -c -s "just check"

# Help of the application
help:
    cargo run --quiet -- --help

# Launch tests
test:
    @echo "🧪 Testing..."
    cargo nextest run
    cargo test --doc

# Format the code
format:
    cargo +nightly fmt
    cargo sort --workspace --grouped

# Format the code
lint:
    @echo "🎩 Linting..."
    cargo check --all-features
    cargo clippy --all-features

# Check the code (formatting, lint, and tests)
check:
    @echo "🦀 Check formatting..."
    cargo +nightly fmt --all -- --check
    cargo sort --workspace --grouped --check
    @just lint
    @just test

# Audit (security issue, licences)
audit:
    @echo "🚨 Audit CVE..."
    cargo audit

    @echo "🪪 Check licences..."
    cargo deny check

# Detect the msrv
msrv:
    cargo msrv list
    cargo msrv

# Check the MSRV
check-msrv:
    cargo msrv verify

# Check semver
semver:
    cargo semver-checks check-release

# Build in production mode
build:
    cargo build --release

# Build the documentation
doc:
    cargo doc

# Install to the cargo bin path
install:
    cargo install --path .

# Update with template
dev-template:
    ffizer apply --source ../ffizer-templates --rev master --source-subfolder rust-app --destination .
