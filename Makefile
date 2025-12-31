# Poker Consensus Engine - Development Makefile

# Rust toolchain
RUST_TOOLCHAIN := $(shell cat rust-toolchain.toml 2>/dev/null | grep channel | cut -d'=' -f2 | tr -d ' ')

# Default target
.PHONY: help
help:
	@echo "Poker Consensus Engine - Development Commands"
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  help             Show this help message"
	@echo "  build            Build the node (release)"
	@echo "  build-debug      Build the node (debug)"
	@echo "  build-wasm       Build runtime WASM"
	@echo "  test             Run all tests"
	@echo "  test-unit        Run unit tests"
	@echo "  test-integration Run integration tests"
	@echo "  lint             Run linting and formatting checks"
	@echo "  lint-fix         Fix linting issues automatically"
	@echo "  fmt              Format code"
	@echo "  check            Run cargo check"
	@echo "  clippy           Run clippy lints"
	@echo "  clean            Clean build artifacts"
	@echo "  purge            Purge local chain data"
	@echo "  start            Start development node"
	@echo "  start-validator  Start validator node"
	@echo "  benchmark        Run benchmarks"
	@echo "  docker-build     Build Docker image"
	@echo "  docker-run       Run Docker container"
	@echo "  install-deps     Install system dependencies"
	@echo "  setup            Full development setup"
	@echo ""

# Build targets
.PHONY: build
build:
	@cargo build --release --bin poker-node

.PHONY: build-debug
build-debug:
	@cargo build --bin poker-node

.PHONY: build-wasm
build-wasm:
	@cargo build --release -p poker-consensus-runtime --features runtime-benchmarks

.PHONY: build-all
build-all: build-wasm build

# Testing targets
.PHONY: test
test: test-unit test-integration

.PHONY: test-unit
test-unit:
	@cargo test --workspace --lib --bins -- --test-threads=4

.PHONY: test-integration
test-integration:
	@cargo test --workspace --test integration -- --test-threads=2

.PHONY: test-verbose
test-verbose:
	@cargo test --workspace --lib --bins -- --nocapture

.PHONY: test-coverage
test-coverage:
	@cargo llvm-cov --workspace --html

# Linting targets
.PHONY: lint
lint: fmt check clippy

.PHONY: lint-fix
lint-fix:
	@cargo fmt --all
	@cargo clippy --all --fix --allow-dirty

.PHONY: fmt
fmt:
	@cargo fmt --all --check

.PHONY: check
check:
	@cargo check --workspace --all-features

.PHONY: clippy
clippy:
	@cargo clippy --workspace --all-features -- -D warnings

# Cleanup targets
.PHONY: clean
clean:
	@cargo clean
	@rm -rf target

.PHONY: purge
purge:
	@rm -rf node/data
	@rm -rf polkadot
	@rm -rf local_testing_state.json

# Node operation targets
.PHONY: start
start:
	@cargo run --bin poker-node -- --chain=dev --alice --validator --unsafe-rpc-external --rpc-cors=all --ws-external

.PHONY: start-validator
start-validator:
	@cargo run --bin poker-node -- --chain=local --validator --unsafe-rpc-external --rpc-cors=all --ws-external --name validator-$(shell whoami)

.PHONY: start-bob
start-bob:
	@cargo run --bin poker-node -- --chain=dev --bob --validator --unsafe-rpc-external --rpc-cors=all --ws-external --name bob

.PHONY: benchmark
benchmark:
	@cargo benchmark --pallet pallet_poker --extrinsic '*' --output=./benchmarking_results

# Docker targets
.PHONY: docker-build
docker-build:
	@docker build -t poker-consensus-node:latest .

.PHONY: docker-run
docker-run:
	@docker run -p 30333:30333 -p 9933:9933 -p 9944:9944 poker-consensus-node:latest --chain=dev --validator

# Dependencies
.PHONY: install-deps
install-deps:
	@echo "Installing system dependencies..."
	@sudo apt-get update
	@sudo apt-get install -y build-essential cmake pkg-config libssl-dev clang
	@echo "Installing Rust..."
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $(RUST_TOOLCHAIN)
	@rustup target add wasm32-unknown-unknown

# Setup target
.PHONY: setup
setup: install-deps build-wasm build
	@echo "Setup complete!"
	@echo "You can now run 'make start' to start a development node"

# Generate keys
.PHONY: generate-keys
generate-keys:
	@cargo run --bin poker-node key generate --scheme Sr25519
	@cargo run --bin poker-node key generate --scheme Ed25519

# Export chain spec
.PHONY: export-spec
export-spec:
	@cargo run --bin poker-node build-spec --chain dev > spec.json
	@cargo run --bin poker-node build-spec --chain dev --raw > spec-raw.json

# Runtime upgrade check
.PHONY: check-runtime
check-runtime:
	@cargo run --bin poker-node export-genesis-state --chain dev > genesis-state
	@cargo run --bin poker-node export-genesis-wasm --chain dev > genesis-wasm

# Help target
help:
	@echo "Available targets:"
	@echo ""
	@echo "Building:"
	@echo "  make build        - Build release binary"
	@echo "  make build-debug  - Build debug binary"
	@echo ""
	@echo "Testing:"
	@echo "  make test         - Run all tests"
	@echo "  make test-unit    - Run unit tests"
	@echo "  make test-verbose - Run tests with output"
	@echo ""
	@echo "Linting:"
	@echo "  make lint         - Check formatting and lints"
	@echo "  make fmt          - Format code"
	@echo "  make clippy       - Run clippy"
	@echo ""
	@echo "Running:"
	@echo "  make start        - Start dev node"
	@echo "  make start-bob    - Start bob node"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker image"
	@echo ""
	@echo "Full setup:"
	@echo "  make setup        - Install deps and build"
