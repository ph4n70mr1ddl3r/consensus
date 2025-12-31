# Poker Consensus Engine

**A trustless, leaderless async BFT consensus engine for mental poker coordination.**

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-yellowgreen)](https://www.rust-lang.org)
[![Substrate](https://img.shields.io/badge/Substrate-4.0-red)](https://substrate.io/)

## Overview

The Poker Consensus Engine is a decentralized platform for coordinating trustless poker games. It uses:

- **BABE/GRANDPA Consensus** for block production and finality
- **Threshold BLS Signatures** for distributed key management
- **Zero-Knowledge Proofs** for privacy-preserving card verification
- **Hybrid Logical Clocks** for fair timestamping and timeout coordination

### Key Features

- 🃏 **Trustless Card Dealing**: No single party knows all cards
- ⏱️ **Fair Timeouts**: Byzantine-tolerant timeout coordination
- 🔐 **Threshold Signatures**: No validator can manipulate game outcomes
- 🛡️ **Byzantine Fault Tolerance**: Survives f < n/3 malicious nodes
- 🔄 **Dynamic Validator Set**: Supports 4+ validators with rotation

## Architecture

```
┌─────────────────────────────────────────────────────┐
│          POKER CONSENSUS ENGINE (Rust/Substrate)    │
├─────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐│
│  │              BABE/GRANDPA CONSENSUS              ││
│  │  - Rotating leaders (NOT single-point trusted)  ││
│  │  - Immediate finality                           ││
│  │  - Byzantine fault tolerance (f < n/3)          ││
│  └─────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────┐│
│  │              CUSTOM PALLETS                      ││
│  │  ┌─────────┐ ┌─────────┐ ┌─────────────────┐   ││
│  │  │  Poker  │ │Timestamp│ │       DKG       │   ││
│  │  │ Pallet  │ │ Pallet  │ │    Pallet       │   ││
│  │  └─────────┘ └─────────┘ └─────────────────┘   ││
│  └─────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────┐│
│  │              CRYPTOGRAPHIC SERVICES              ││
│  │  - BLS12-381 Threshold Signatures               ││
│  │  - ZK Proofs (Groth16/PLONK)                    ││
│  │  - Card Commitment Schemes                      ││
│  └─────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.75+ with WASM target
- CMake, Clang, and build essentials
- 4GB+ RAM (8GB+ recommended for building)

### Installation

```bash
# Clone the repository
git clone https://github.com/poker-consensus/engine.git
cd engine

# Install dependencies
make install-deps

# Build the node
make build

# Start development node
make start
```

### Using Docker

```bash
# Build Docker image
make docker-build

# Run container
make docker-run
```

## Development

### Setting Up Development Environment

```bash
# Full setup (install deps + build)
make setup

# Run tests
make test

# Run linter
make lint
```

### Available Make Targets

| Target | Description |
|--------|-------------|
| `make build` | Build release binary |
| `make test` | Run all tests |
| `make lint` | Run formatting and lints |
| `make start` | Start dev node |
| `make clean` | Clean build artifacts |
| `make docker-build` | Build Docker image |

See [Makefile](Makefile) for complete target list.

## Project Structure

```
poker-consensus/
├── node/                   # Substrate node implementation
│   ├── src/
│   │   ├── main.rs        # Node entry point
│   │   ├── service.rs     # Node services
│   │   └── rpc.rs         # RPC handlers
│   └── Cargo.toml
├── runtime/               # Runtime module
│   └── src/lib.rs
├── pallets/               # Pallets
│   ├── poker/            # Game logic
│   ├── timestamp/        # Timestamp consensus
│   ├── dkg/              # Distributed key generation
│   └── bls/              # BLS signatures
├── scripts/              # Build and utility scripts
├── Makefile              # Development commands
├── Dockerfile            # Docker build
└── rust-toolchain.toml   # Rust version
```

## Documentation

- [Architecture](_bmad-output/planning-artifacts/architecture/poker-consensus-engine-architecture.md)
- [Technical Specification](_bmad-output/planning-artifacts/tech-spec/poker-consensus-engine-tech-spec.md)
- [Story Backlog](_bmad-output/planning-artifacts/stories/poker-consensus-stories.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Security

For security vulnerabilities, please email security@poker-consensus.engine.

## License

This project is licensed under the AGPL-3.0 License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [Substrate](https://substrate.io/) - Blockchain framework
- [Polkadot](https://polkadot.network/) - Consensus architecture
- [arkworks](https://arkworks.io/) - Rust cryptographic libraries
