set dotenv-load

# List all just recipes
default:
    @just --list

# Install require tools
requirements:
    rustup toolchain add nightly
    cargo binstall cargo-watch cargo-audit cargo-deny cargo-msrv cargo-semver-checks cargo-llvm-cov
    cargo binstall cargo-nextest
    cargo binstall cocogitto

# Run TDD mode
tdd:
    cargo watch -c -s "just check"

# Help of the application
help:
    cargo run --quiet -- --help

# Launch tests
test:
    @echo "🧪 Testing..."
    @just dev-kafka up -d --wait
    cargo nextest run
    cargo test --doc
    @just dev-kafka down

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

# Check code coverage
coverage:
    cargo llvm-cov --open

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

# Release {major, minor, patch, release, rc, beta, alpha} or version
changelog:
    cargo changelog --write schema-registry-api

# Release {major, minor, patch, release, rc, beta, alpha} or version
release:
    cargo smart-release --update-crates-index schema-registry-api

# DO Release {major, minor, patch, release, rc, beta, alpha} or version
release-execute: 
    cargo smart-release --update-crates-index schema-registry-api --execute

# Update with template
dev-template:
    ffizer apply --source ../ffizer-templates --rev master --source-subfolder rust-app --destination .

# Start/stop a kafka+schema-registry with docker-compose
dev-kafka *ARGS="up -d":
    docker-compose --file ./docker/docker-compose.yaml {{ARGS}}
