# Reactive Atomic Swap

A Solana program implementing a reactive atomic swap mechanism with automated rebalancing. The system enables users to deposit Token X and receive Token Y in return, with an admin-controlled pool that automatically rebalances based on deposit events.

## Architecture

### Core Components

- **Swap Program**: On-chain Solana program handling deposits and admin rebalancing
- **Pool Initialization Client**: Sets up new swap pools
- **Depositor Client**: Executes user deposits (Token X → Token Y)
- **Rebalancer Client**: Monitors deposits and automatically tops up Token Y reserves

### Program Features

- **Initialize Pool**: Create new swap pools with admin controls
- **Deposit**: 1:1 atomic swap from Token X to Token Y
- **Admin Rebalance**: Top-up or withdraw Token Y from pool reserves
- **Event Logging**: Machine-parseable deposit events for reactive monitoring

## Quick Start

### Prerequisites

- Rust toolchain
- Solana CLI tools
- Local Solana test validator (for development)

### Build

```bash
# Build the on-chain program
cd programs/swap_program
cargo build-bpf

# Build client tools
cd ../../clients
cargo build --release
```

### Deploy Program

```bash
# Deploy to local validator
solana program deploy programs/swap_program/target/deploy/swap_program.so
```

## Usage

### 1. Initialize Pool

Set environment variables and run the pool initializer:

```bash
export SOLANA_URL="http://127.0.0.1:8899"
export PROGRAM_ID="<your_deployed_program_id>"
export PAYER_KEYPAIR="<path_to_payer_keypair>"
export POOL_STATE_KEYPAIR="<path_to_pool_state_keypair>"
export ADMIN_PUBKEY="<admin_public_key>"
export VAULT_X="<token_x_vault_account>"
export VAULT_Y="<token_y_vault_account>"
export MINT_X="<token_x_mint>"
export MINT_Y="<token_y_mint>"

./target/release/init_pool
```

### 2. Execute Deposits

```bash
export USER_KEYPAIR="<path_to_user_keypair>"
export USER_TOKEN_X="<user_token_x_account>"
export USER_TOKEN_Y="<user_token_y_account>"
export POOL_VAULT_X="<pool_vault_x>"
export POOL_VAULT_Y="<pool_vault_y>"
export POOL_STATE_PUBKEY="<pool_state_account>"
export POOL_AUTHORITY="<derived_pda>"
export DEPOSIT_AMOUNT="1000000"  # Amount in smallest units

./target/release/depositor
```

### 3. Run Reactive Rebalancer

```bash
export SOLANA_WS_URL="ws://127.0.0.1:8900"
export ADMIN_KEYPAIR="<path_to_admin_keypair>"
export ADMIN_TOKEN_Y="<admin_token_y_account>"
export POOL_VAULT_Y="<pool_vault_y>"
export POOL_STATE_PUBKEY="<pool_state_account>"
export TOPUP_AMOUNT="1000000"  # Auto top-up amount

./target/release/rebalancer
```

## Program Instructions

### InitializePool (Tag: 0)
Creates a new swap pool with specified token vaults and admin authority.

### Deposit (Tag: 1)
- Transfers Token X from user to pool
- Transfers Token Y from pool to user (1:1 ratio)
- Emits deposit event for reactive monitoring

### AdminRebalance (Tag: 2)
- **Action 0**: Top-up pool with Token Y (admin → pool)
- **Action 1**: Withdraw Token Y from pool (pool → admin)

## Event System

The program emits structured log events for external monitoring:

```
EVENT:Deposit|user=<pubkey>|amount=<amount>
EVENT:AdminRebalance|action=<topup|withdraw>|admin=<pubkey>|amount=<amount>
```

The rebalancer client subscribes to these events via WebSocket and automatically triggers rebalancing transactions.

## Security Features

- Admin-only rebalancing operations
- PDA-based pool authority for secure token transfers
- Rent-exempt account validation
- Owner verification for all token accounts

## Development

### Project Structure

```
├── programs/
│   └── swap_program/          # On-chain Solana program
├── clients/
│   ├── init_pool/            # Pool initialization client
│   ├── depositor/            # Deposit execution client
│   └── rebalancer/           # Reactive rebalancing service
```

### Testing

Run the local test validator and deploy the program for testing:

```bash
solana-test-validator
# In another terminal, deploy and test
```

## License

MIT
