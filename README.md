# Solana with Native Rust

A comprehensive collection of production-ready Solana programs and tools built with native Rust. This repository demonstrates advanced Solana development patterns, DeFi protocols, trading bots, and low-level blockchain interactions without relying heavily on SDKs.

## Table of Contents

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

## Overview

This repository serves as a learning resource and reference implementation for building Solana applications using native Rust. Each branch contains a complete, standalone project demonstrating different aspects of Solana development: DeFi Protocols (AMM pools, atomic swaps, vaults), Trading Bots (volume bots, rebalancers, MEV strategies), Blockchain Tools (transaction monitoring, token decoding, RPC utilities), and Advanced Patterns (versioned transactions, Address Lookup Tables, PDA management).

## Repository Structure

This repository uses a branch-based structure where each branch contains a complete, independent project:

- `AMM-Pool_Swap`: Automated Market Maker implementation
- `atomic-volume-bot`: Anchor-based volume generation bot
- `jito-dex-rebalancer-bot`: MEV-aware DEX rebalancing system
- `native-rust-vault`: Pure Rust vault with PDA management
- `no-sdk-token-decoder`: Manual SPL token account decoder
- `raydium-reverberate`: Raydium atomic round-trip swaps
- `reactive-atomic-swap-engine`: Event-driven swap with auto-rebalancing
- `solana-v0-tx-with-alt`: Version 0 transactions with ALTs
- `usdc-burn-listener`: Real-time USDC burn event monitor

## Projects

### 1. AMM Pool & Swap (Branch: AMM-Pool_Swap)

An Automated Market Maker using Anchor framework. Features: basic AMM pool initialization, token swap mechanisms, Anchor framework integration, TypeScript client. Tech Stack: Anchor, Rust, TypeScript, SPL. Use Case: Understanding AMM fundamentals and Anchor development patterns.

### 2. Atomic Volume Bot (Branch: atomic-volume-bot)

An Anchor-based bot for generating trading volume on Solana DEXs. Features: automated volume generation, Anchor program architecture, TypeScript testing, DEX integration. Tech Stack: Anchor, Rust, TypeScript, Web3.js. Use Case: Market making, liquidity provision, volume simulation.

### 3. Jito DEX Rebalancer Bot (Branch: jito-dex-rebalancer-bot)

A rebalancing bot designed for Jito's MEV infrastructure. Features: native Rust implementation (no Anchor), separate client/program architecture, MEV-aware execution, automated rebalancing. Tech Stack: Native Rust, Solana SDK, Jito MEV. Use Case: Advanced trading strategies, MEV capture, automated portfolio management.

### 4. Native Rust Vault (Branch: native-rust-vault)

A pure Solana program implementing a secure per-user SOL vault system. Features: deterministic PDAs, deposit/withdrawal operations, authorization enforcement, rent-exemption preservation. Tech Stack: Native Rust, SPL (no Anchor). Key Details: Each user gets a unique PDA vault `[b"vault", depositor_pubkey]`, strict authorization, automatic rent-exempt balance preservation.

### 5. No-SDK Token Decoder (Branch: no-sdk-token-decoder)

A Rust utility that manually decodes SPL Token accounts without SDK helpers. Features: manual byte-level deserialization, decodes Mint and Token Accounts, zero-copy parsing with `#[repr(C)]` structs, RPC integration. Tech Stack: Rust, Solana Client, Bytemuck. Key Features: detects account type, manual little-endian conversion, decimal-aware balance formatting.

### 6. Raydium Reverberate (Branch: raydium-reverberate)

An advanced trading bot executing atomic round-trip swaps on Raydium AMM pools. Features: atomic buy-sell transactions, direct Raydium AMM v4 integration, slippage protection, auto ATA creation, volume simulation, TypeScript client. Tech Stack: Anchor, Rust, TypeScript, Raydium SDK v2. Use Case: Market making, volume testing, atomic arbitrage.

### 7. Reactive Atomic Swap Engine (Branch: reactive-atomic-swap-engine)

A reactive swap system with automated rebalancing triggered by deposit events. Features: 1:1 atomic swap (Token X → Token Y), event-driven architecture, automated rebalancing client, admin controls. Tech Stack: Native Rust, Solana SDK. Components: Swap Program (on-chain logic), Pool Initialization Client, Depositor Client, Rebalancer Client.

### 8. Solana V0 Transactions with ALT (Branch: solana-v0-tx-with-alt)

A comprehensive implementation demonstrating Solana's Version 0 transactions with Address Lookup Tables. Features: v0 transaction construction, ALT creation and usage, transaction size optimization, serialization/deserialization examples. Tech Stack: Native Rust, Solana SDK, bincode. Key Concepts: legacy vs. v0 differences, ALT benefits, transaction serialization format, compact account indexing.

### 9. USDC Burn Listener (Branch: usdc-burn-listener)

A real-time monitoring tool detecting USDC token burn events on Solana. Features: real-time blockchain monitoring, SPL token burn detection, duplicate signature tracking, configurable RPC endpoints, async Rust with Tokio. Tech Stack: Rust, Tokio, Reqwest, Solana RPC. Use Case: Token supply monitoring, burn analytics, building notification systems.

## Technologies

Core: Rust (primary language), Solana (Layer 1 blockchain), Anchor Framework (smart contracts), TypeScript (client interaction)

Libraries: Solana Program Library (SPL), Solana SDK, Bytemuck (zero-copy casting), Tokio (async runtime), Web3.js (JavaScript client), Raydium SDK (DEX integration), Jito (MEV infrastructure)

Development Tools: Cargo (Rust package manager), Anchor CLI (framework tooling), Solana CLI (blockchain interaction), solana-test-validator (local development)

## Getting Started

Prerequisites:

1. Rust Toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Solana CLI Tools (v1.17+): `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`
3. Anchor Framework: `cargo install --git https://github.com/coral-xyz/anchor avm --locked --force && avm install latest && avm use latest`
4. Node.js & Yarn/NPM (v18+): `curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash - && sudo apt-get install -y nodejs && npm install -g yarn`

Quick Start:

1. Clone: `git clone https://github.com/ptechfusion19/Solana-with-Native-Rust.git && cd Solana-with-Native-Rust`
2. Checkout branch: `git checkout [branch-name]` (e.g., `native-rust-vault`, `raydium-reverberate`)
3. Each branch contains its own README with project-specific setup instructions

Build Patterns:

For Anchor projects (AMM-Pool_Swap, atomic-volume-bot, raydium-reverberate):
```bash
yarn install && anchor build && anchor test
```

For native Rust programs (native-rust-vault, jito-dex-rebalancer-bot):
```bash
cargo build-sbf && solana program deploy target/deploy/program_name.so
```

For Rust client tools (no-sdk-token-decoder, usdc-burn-listener):
```bash
cargo build --release && cargo run
```

Local Development: `solana-test-validator --reset` and configure with `solana config set --url localhost`

## Learning Path

Recommended exploration order:
1. `native-rust-vault` - Core Solana concepts
2. `AMM-Pool_Swap` - Anchor framework
3. `raydium-reverberate` - Atomic operations and DEX integration
4. `no-sdk-token-decoder` - Low-level blockchain data
5. `reactive-atomic-swap-engine` - Event-driven architectures
6. `solana-v0-tx-with-alt` - Transaction optimization
7. `usdc-burn-listener` - Real-time blockchain monitoring

## License

MIT License


## Resources

- Solana Documentation: https://docs.solana.com/
- Anchor Framework: https://www.anchor-lang.com/
- Solana Cookbook: https://solanacookbook.com/
- Solana Program Library: https://spl.solana.com/
- Raydium SDK: https://raydium.io/developers/

## Author

ProgrammX - Let's build on Solana!
