# Native Rust Vault

**Short description**

A native Solana program (no Anchor) that implements a per-user SOL Vault. Each depositor gets a deterministic Program Derived Address (PDA) that securely holds their SOL. The program enforces strict checks so **only the original depositor** can withdraw funds and the vault account always keeps enough lamports for rent-exemption.

---

## Features

- Deposit SOL into a program-owned PDA (created on first deposit).
- Withdraw SOL only by the original depositor (authorization enforced by comparing stored depositor pubkey).
- Rent-exemption preservation: the program prevents draining the PDA below the rent-exempt minimum for the stored vault state.
- Small, audit-friendly instruction format (explicit tag + amount layout).

---

## Repository layout (what to look at)

- `src/lib.rs` — program entry and exports.
- `src/instruction.rs` — instruction enum and helper(s) to parse incoming bytes.
- `src/processor.rs` — core deposit/withdraw processing, PDA derivation, `invoke_signed` calls.
- `src/state.rs` — the `VaultState` layout that is stored in the PDA account data.
- `Cargo.toml` — crate configuration (cdylib / solana-program dependency).

---

## Requirements

- Rust toolchain (nightly not required unless the project uses unstable features).
- `solana` CLI and `solana-test-validator` for local testing / deployment.
- `cargo build-bpf` (or your normal `cargo build` depending on your on-chain toolchain) — follow your usual Solana program build steps.

---

## Accounts (order & flags)

The program expects accounts in this exact order (client must follow):

1. **Depositor** — signer, **writable** (payer / target to receive refunds on withdraw).
2. **Vault PDA** — writable; program-owned PDA where SOL is stored.
3. **System Program** — read-only.
4. **Rent Sysvar** — read-only.

Make sure the depositor account is signed and marked writable in the transaction's `Message`.

---

## PDA derivation

The vault PDA is derived deterministically using a seed prefix plus the depositor's pubkey. For example (Rust-style):

```rust
let (vault_pda, bump) = Pubkey::find_program_address(&[b"vault", depositor_pubkey.as_ref()], program_id);
```

The program stores the `bump` and the depositor pubkey inside the vault account `VaultState` to allow validation on withdraw and to perform `invoke_signed` for transfers.

---

## Vault state layout

The on-chain state stored in the PDA (as a compact struct) typically contains:

- `depositor: Pubkey` (32 bytes)
- `bump: u8` (1 byte)
- `initialized: bool` or `u8` (1 byte)

Total size should be computed and used to reserve rent-exempt lamports when creating the PDA account. The program uses this size to calculate `rent.minimum_balance(VaultState::size())`.

---

## Deposit flow (high-level)

- Client builds a `Deposit` instruction with tag `0` and the deposit `amount`.
- Program derives the PDA with seeds `[b"vault", depositor_pubkey]`.
- If the PDA is uninitialized (owned by System Program), program uses `system_instruction::create_account` (via `invoke_signed`) to create the PDA account and funds it with `rent_exempt_min + amount` so it becomes rent-exempt and holds the initial deposit.
- If PDA already exists and is owned by the program, program accepts a `system_instruction::transfer` (depositor -> PDA) for the deposit amount.

**Important for clients:** On first deposit the client must fund both the rent-exempt lamports and the deposit amount in the same transaction so `create_account` succeeds.

---

## Withdraw flow (high-level & security)

- Client builds a `Withdraw` instruction with tag `1` and the requested `amount` (or `0` to withdraw all available while preserving rent).
- Program verifies the depositor is a signer.
- Program checks `vault_info.owner == program_id`.
- Program unpacks `VaultState` stored in PDA and checks `state.depositor == depositor_pubkey` — only the original depositor can withdraw.
- Program computes the maximum withdrawable amount as `vault_balance.checked_sub(rent_minimum)`.
- Program transfers lamports from PDA → depositor using `system_instruction::transfer` with `invoke_signed`, using seeds `[b"vault", depositor_pubkey]` + `bump` to sign as the PDA.

If the requested amount is greater than the available amount, the program returns an error (custom `VaultError::InsufficientFunds` or similar).

---

## Error codes

The program exposes a small custom error enum (translated into `ProgramError::Custom`), examples include:

- `InvalidInstruction` — instruction tag unknown or malformed data.
- `Unauthorized` — requester is not the original depositor.
- `NotRentExempt` — PDA isn't rent-exempt when required.
- `AlreadyInitialized` — trying to initialize an already initialized PDA.

---

## Testing locally

1. Start a local validator:

```bash
solana-test-validator --reset
```

2. Build the program and deploy (use your normal build/deploy steps — `cargo build-bpf` or `cargo build --release` + `solana program deploy <so>`).

---

## License

MIT
