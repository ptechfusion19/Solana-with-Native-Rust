# no_sdk_token_decode — Manual SPL Token account decoder in Rust (No SPL helper crates)

## 1. Description

A Rust utility that connects to a Solana RPC node, fetches raw account data, and manually decodes SPL **Mint** and **Token Account** layouts without using the `spl-token` helpers. The tool demonstrates how raw bytes map to Rust `#[repr(C)]` structs and shows the owner, mint, and raw balance. Useful for learning, forensic inspection, and projects that must avoid SDK helper functions.

---

## 2. Features

- Fetch account bytes via Solana RPC (`solana-client`).
- Detect whether an account is a **Mint** (82 bytes) or a **Token Account** (>= 165 bytes).
- Manual zero-copy deserialization using `bytemuck` into tight `#[repr(C)]` structs.
- Convert raw little-endian fields (`u64`, `u32`) and decode `Pubkey` fields.
- Optionally fetch the mint account to print a human-readable token balance (`amount / 10^decimals`).
- No use of `spl-token` crate unpack helpers — full manual decoding.

---

## 3. Prerequisites

- Rust (stable toolchain) — `rustup` recommended.
- Cargo (comes with Rust).
- Network connectivity to a Solana RPC node (e.g., `https://api.mainnet-beta.solana.com` or Helius/QuickNode URL).

---

## 4. Installation

1. Clone or create the project folder:

```bash
git clone <your-repo> no_sdk_token_decode
cd no_sdk_token_decode
```

2. Build the project:

```bash
cargo build
```

---

## 5. Configuration

The program accepts a positional RPC URL and an account pubkey.

- `RPC_URL` (optional): If not provided, defaults to `https://api.mainnet-beta.solana.com`.
- `ACCOUNT_PUBKEY` (required): Pubkey of the account you want to decode.

If you use a provider that requires an API key (e.g., Helius), include the key in the RPC URL:

```
https://rpc.helius.xyz/?api-key=YOUR_KEY_HERE
```

---

## 8. How to Use

Run the binary with the RPC URL (optional) and the account pubkey:

```bash
cargo run -- "https://api.mainnet-beta.solana.com" <ACCOUNT_PUBKEY>
```

Example (test token-account provided by instructor):

```bash
cargo run -- "https://api.mainnet-beta.solana.com" 3emsAVdmGKERbHjmGfQ6oZ1e35dkf5iYcS6U4CPKFVaa
```

If the account is a **Mint**, the tool prints `supply`, `decimals`, `mint_authority`, etc. If it's a **Token Account**, the tool prints `mint`, `owner`, the raw `amount`, and (if possible) a human-readable balance using the mint's `decimals`.

---

## 9. Code Examples

### Key structs (from `src/main.rs`):

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct TokenAccountRaw {
    pub mint: [u8; 32],
    pub owner: [u8; 32],
    pub amount: [u8; 8],
    pub delegate_option: [u8; 4],
    pub delegate: [u8; 32],
    pub state: u8,
    pub is_native_option: [u8; 4],
    pub is_native: [u8; 8],
    pub delegated_amount: [u8; 8],
    pub close_authority_option: [u8; 4],
    pub close_authority: [u8; 32],
}

// zero-copy cast example
let token: &TokenAccountRaw = bytemuck::try_from_bytes(slice)
    .map_err(|e| anyhow::anyhow!("Failed to cast bytes into TokenAccountRaw: {}", e))?;

// convert little-endian arrays to integers
let amount = u64::from_le_bytes(token.amount);
let owner_pubkey = Pubkey::new_from_array(token.owner);
```

---

## 10. Demos

Example CLI tool output might look like:

```
RPC: https://api.mainnet-beta.solana.com
Token Account: 3emsAVdmGKERbHjmGfQ6oZ1e35dkf5iYcS6U4CPKFVaa
Expected token account layout size: 165 bytes
mint: <pubkey>
owner: <pubkey>
amount (raw u64): 1234567890
mint decimals: 6
human-readable balance: 1234.567890
```

---

## 11. Project Structure

```
no_sdk_token_decode/
├─ Cargo.toml
├─ README.md
└─ src/
   └─ main.rs        # RPC client + manual decode logic
```

- `main.rs` contains the `MintRaw` and `TokenAccountRaw` structs, RPC fetching, detection logic (mint vs token account), and formatting/printing logic.

---

## 12. License
MIT
