# Solana with Native Rust 🦀

A comprehensive collection of production-ready Solana programs and tools built with native Rust. This repository demonstrates advanced Solana development patterns, DeFi protocols, trading bots, and low-level blockchain interactions without relying heavily on SDKs.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-14F195?style=flat&logo=solana&logoColor=white)](https://solana.com/)

## 📋 Table of Contents

- [Overview](#overview)
- [Repository Structure](#repository-structure)
- [Projects](#projects)
  - [AMM Pool & Swap](#1-amm-pool--swap)
  - [Atomic Volume Bot](#2-atomic-volume-bot)
  - [Jito DEX Rebalancer Bot](#3-jito-dex-rebalancer-bot)
  - [Native Rust Vault](#4-native-rust-vault)
  - [No-SDK Token Decoder](#5-no-sdk-token-decoder)
  - [Raydium Reverberate](#6-raydium-reverberate)
  - [Reactive Atomic Swap Engine](#7-reactive-atomic-swap-engine)
  - [Solana V0 Transactions with ALT](#8-solana-v0-transactions-with-alt)
  - [USDC Burn Listener](#9-usdc-burn-listener)
- [Technologies](#technologies)
- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [License](#license)

## 🎯 Overview

This repository serves as a learning resource and reference implementation for building Solana applications using native Rust. Each branch contains a complete, standalone project demonstrating different aspects of Solana development:

- **DeFi Protocols**: AMM pools, atomic swaps, and vault systems
- **Trading Bots**: Volume bots, rebalancers, and MEV strategies
- **Blockchain Tools**: Transaction monitoring, token decoding, and RPC utilities
- **Advanced Patterns**: Versioned transactions, Address Lookup Tables, and PDA management

All projects emphasize low-level, native Rust implementations to provide deep insights into Solana's architecture and runtime behavior.

## 🗂️ Repository Structure

This repository uses a **branch-based structure** where each branch contains a complete, independent project:

```
main                          # Repository root with overview
├── AMM-Pool_Swap            # Automated Market Maker implementation
├── atomic-volume-bot        # Anchor-based volume generation bot
├── jito-dex-rebalancer-bot  # MEV-aware DEX rebalancing system
├── native-rust-vault        # Pure Rust vault with PDA management
├── no-sdk-token-decoder     # Manual SPL token account decoder
├── raydium-reverberate      # Raydium atomic round-trip swaps
├── reactive-atomic-swap-engine  # Event-driven swap with auto-rebalancing
├── solana-v0-tx-with-alt    # Version 0 transactions with ALTs
└── usdc-burn-listener       # Real-time USDC burn event monitor
```

## 🚀 Projects

### 1. AMM Pool & Swap

**Branch**: `AMM-Pool_Swap`

An Automated Market Maker (AMM) implementation using the Anchor framework.

**Features**:
- Basic AMM pool initialization
- Token swap mechanisms
- Anchor framework integration
- TypeScript client for interaction

**Tech Stack**: Anchor, Rust, TypeScript, Solana Program Library

**Use Case**: Understanding AMM fundamentals and Anchor development patterns

---

### 2. Atomic Volume Bot

**Branch**: `atomic-volume-bot`

An Anchor-based bot for generating trading volume on Solana DEXs.

**Features**:
- Automated volume generation
- Anchor program architecture
- TypeScript testing framework
- Integration with Solana DEXs

**Tech Stack**: Anchor, Rust, TypeScript, Solana Web3.js

**Use Case**: Market making, liquidity provision, and volume simulation

---

### 3. Jito DEX Rebalancer Bot

**Branch**: `jito-dex-rebalancer-bot`

A sophisticated rebalancing bot designed to work with Jito's MEV infrastructure.

**Features**:
- Native Rust implementation (no Anchor)
- Separate client and program architecture
- MEV-aware execution strategies
- Automated portfolio rebalancing

**Tech Stack**: Native Rust, Solana SDK, Jito MEV infrastructure

**Use Case**: Advanced trading strategies, MEV capture, and automated portfolio management

---

### 4. Native Rust Vault

**Branch**: `native-rust-vault`

A pure Solana program (no Anchor) implementing a secure per-user SOL vault system.

**Features**:
- Deterministic Program Derived Addresses (PDAs)
- Deposit and withdrawal operations
- Authorization enforcement (only depositor can withdraw)
- Rent-exemption preservation
- Comprehensive security checks

**Tech Stack**: Native Rust, Solana Program Library (no Anchor)

**Use Case**: 
- Learning native Solana program development
- Understanding PDA derivation and `invoke_signed` patterns
- Building secure custody solutions

**Key Implementation Details**:
- Each user gets a unique PDA vault: `[b"vault", depositor_pubkey]`
- Strict authorization: only the original depositor can withdraw
- Automatic rent-exempt balance preservation
- No SDK helpers - demonstrates raw Solana programming

---

### 5. No-SDK Token Decoder

**Branch**: `no-sdk-token-decoder`

A Rust utility that manually decodes SPL Token accounts without using `spl-token` helper crates.

**Features**:
- Manual byte-level deserialization using `bytemuck`
- Decodes both Mint and Token Account layouts
- Zero-copy parsing with `#[repr(C)]` structs
- RPC integration for fetching account data
- Human-readable balance calculation

**Tech Stack**: Rust, Solana Client, Bytemuck, no SPL helpers

**Use Case**:
- Learning SPL Token account layout internals
- Forensic analysis and debugging
- Projects requiring SDK-free token inspection
- Understanding raw account data structures

**Key Features**:
- Detects account type (82 bytes = Mint, 165+ bytes = Token Account)
- Manual little-endian conversion
- Pubkey extraction from raw bytes
- Decimal-aware balance formatting

---

### 6. Raydium Reverberate

**Branch**: `raydium-reverberate`

An advanced trading bot that executes atomic round-trip swaps on Raydium AMM pools.

**Features**:
- Atomic buy-and-sell transactions (both succeed or both fail)
- Direct Raydium AMM v4 integration
- Built-in slippage protection
- Automatic associated token account creation
- Volume simulation capabilities
- TypeScript client with Raydium SDK v2

**Tech Stack**: Anchor, Rust, TypeScript, Raydium SDK v2

**Use Case**:
- Market making strategies
- Volume testing and simulation
- Atomic arbitrage operations
- MEV-resistant trading

**Architecture**:
- Smart contract implements `atomic_round_trip_swap` instruction
- Client handles pool data fetching and transaction building
- Comprehensive implementation documentation included

---

### 7. Reactive Atomic Swap Engine

**Branch**: `reactive-atomic-swap-engine`

A reactive swap system with automated rebalancing triggered by deposit events.

**Features**:
- 1:1 atomic swap (Token X → Token Y)
- Event-driven architecture with deposit logging
- Automated rebalancing client monitors deposits
- Admin controls for pool management
- Multiple client tools (pool initializer, depositor, rebalancer)

**Tech Stack**: Native Rust, Solana SDK, Event-driven architecture

**Use Case**:
- Building reactive trading systems
- Automated liquidity management
- Event monitoring and response patterns
- Cross-program invocation (CPI) patterns

**Components**:
- **Swap Program**: On-chain deposit and rebalance logic
- **Pool Initialization Client**: Creates new swap pools
- **Depositor Client**: User-facing deposit interface
- **Rebalancer Client**: Monitors events and tops up reserves

---

### 8. Solana V0 Transactions with ALT

**Branch**: `solana-v0-tx-with-alt`

A comprehensive implementation demonstrating Solana's Version 0 transactions with Address Lookup Tables (ALTs).

**Features**:
- Version 0 (v0) transaction construction
- Address Lookup Table (ALT) creation and usage
- Transaction size optimization techniques
- Complete serialization/deserialization examples
- Pure native Rust implementation

**Tech Stack**: Native Rust, Solana SDK, bincode

**Use Case**:
- High-throughput DeFi operations
- MEV strategies requiring many accounts
- Protocol development with complex account requirements
- Understanding transaction anatomy and optimization

**Key Concepts Demonstrated**:
- Legacy vs. v0 transaction differences
- ALT benefits and operations
- Transaction serialization format
- Compact account key indexing
- Base64 encoding for RPC submission

---

### 9. USDC Burn Listener

**Branch**: `usdc-burn-listener`

A real-time monitoring tool that detects USDC token burn events on Solana.

**Features**:
- Real-time blockchain monitoring
- SPL token burn instruction detection
- Duplicate signature tracking
- Configurable RPC endpoints
- Async Rust with Tokio runtime

**Tech Stack**: Rust, Tokio, Reqwest, Solana RPC

**Use Case**:
- Token supply monitoring
- Burn event analytics
- Real-time blockchain event detection
- Building notification systems

**Configuration**:
- Environment variables for RPC URL and mint address
- Default monitoring of mainnet USDC
- Extensible to any SPL token

---

## 🛠️ Technologies

This repository leverages a comprehensive Solana development stack:

### Core Technologies
- **Rust**: Primary language for all programs and most clients
- **Solana**: Layer 1 blockchain platform
- **Anchor Framework**: Solana's Sealevel framework for smart contracts (selected projects)
- **TypeScript**: Client-side interaction and testing

### Libraries & Tools
- **Solana Program Library (SPL)**: Token standards and utilities
- **Solana SDK**: Core blockchain interaction libraries
- **Bytemuck**: Zero-copy type casting
- **Tokio**: Async runtime for Rust
- **Web3.js**: JavaScript Solana client
- **Raydium SDK**: DEX integration
- **Jito**: MEV infrastructure

### Development Tools
- **Cargo**: Rust package manager
- **Anchor CLI**: Framework tooling
- **Solana CLI**: Blockchain interaction
- **solana-test-validator**: Local development blockchain

## 🏁 Getting Started

### Prerequisites

Before working with any project in this repository, ensure you have:

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Solana CLI Tools** (v1.17+)
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

3. **Anchor Framework** (for Anchor projects)
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install latest
   avm use latest
   ```

4. **Node.js & Yarn/NPM** (v18+, for TypeScript clients)
   ```bash
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   npm install -g yarn
   ```

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/ptechfusion19/Solana-with-Native-Rust.git
   cd Solana-with-Native-Rust
   ```

2. **Checkout the project you want to explore**
   ```bash
   # Example: Explore the Native Rust Vault
   git checkout native-rust-vault
   
   # Or explore the Raydium Reverberate bot
   git checkout raydium-reverberate
   ```

3. **Follow project-specific instructions**
   
   Each branch contains its own README with detailed setup and usage instructions specific to that project.

### General Build Patterns

**For Anchor projects** (AMM-Pool_Swap, atomic-volume-bot, raydium-reverberate):
```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test
```

**For native Rust programs** (native-rust-vault, jito-dex-rebalancer-bot):
```bash
# Build the program
cargo build-bpf

# Or for newer Solana versions
cargo build-sbf

# Deploy to local validator
solana program deploy target/deploy/program_name.so
```

**For Rust client tools** (no-sdk-token-decoder, usdc-burn-listener, solana-v0-tx-with-alt):
```bash
# Build
cargo build --release

# Run
cargo run
```

### Local Development

Start a local Solana test validator for development:

```bash
solana-test-validator --reset
```

Configure Solana CLI to use localhost:

```bash
solana config set --url localhost
```

## 📚 Learning Path

Recommended order for exploring the projects:

1. **Start Simple**: `native-rust-vault` - Learn core Solana concepts
2. **Add Complexity**: `AMM-Pool_Swap` - Understand Anchor framework
3. **Advanced Patterns**: `raydium-reverberate` - Atomic operations and DEX integration
4. **Deep Dive**: `no-sdk-token-decoder` - Low-level blockchain data
5. **Production Ready**: `reactive-atomic-swap-engine` - Event-driven architectures
6. **Optimization**: `solana-v0-tx-with-alt` - Transaction optimization
7. **Monitoring**: `usdc-burn-listener` - Real-time blockchain monitoring

## 🤝 Contributing

Contributions are welcome! Each branch represents a standalone project. To contribute:

1. Choose the branch/project you want to improve
2. Create a feature branch from that project branch
3. Make your changes
4. Submit a pull request to the respective project branch

## 📄 License

MIT License

Copyright (c) 2025 ProgrammX

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## 🔗 Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Program Library](https://spl.solana.com/)
- [Raydium SDK](https://raydium.io/developers/)

## 👨‍💻 Author

**ProgrammX**

Let's get rusty! 🦀
