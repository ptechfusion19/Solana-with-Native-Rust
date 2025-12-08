use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::{env, fs};

/// PoolState size must match the on-chain program PoolState::LEN
/// (1 bool) + (5 * Pubkey(32)) + (1 u8) = 162 bytes
const POOL_STATE_SPACE: usize = 162;

fn read_keypair(path: &str) -> Result<Keypair> {
    let s = fs::read_to_string(path)?;
    let v: Vec<u8> = serde_json::from_str(&s)?;
    Ok(Keypair::from_bytes(&v)?)
}

fn main() -> Result<()> {
    // args via env or CLI
    // Required env vars:
    // - SOLANA_URL (e.g. http://127.0.0.1:8899)
    // - PROGRAM_ID (your deployed program id)
    // - PAYER_KEYPAIR (path)
    // - POOL_STATE_KEYPAIR (path to new or existing keypair)
    // - ADMIN_PUBKEY (pubkey string)
    // - VAULT_X, VAULT_Y (token account pubkeys)
    // - MINT_X, MINT_Y (mint pubkeys)
    let rpc_url = env::var("SOLANA_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let program_id: Pubkey = env::var("PROGRAM_ID")?.parse()?;
    let payer_path = env::var("PAYER_KEYPAIR")?;
    let pool_state_path = env::var("POOL_STATE_KEYPAIR")?;
    let admin_pubkey: Pubkey = env::var("ADMIN_PUBKEY")?.parse()?;
    let vault_x: Pubkey = env::var("VAULT_X")?.parse()?;
    let vault_y: Pubkey = env::var("VAULT_Y")?.parse()?;
    let mint_x: Pubkey = env::var("MINT_X")?.parse()?;
    let mint_y: Pubkey = env::var("MINT_Y")?.parse()?;

    let payer = read_keypair(&payer_path)?;
    let pool_state_kp = read_keypair(&pool_state_path)?;

    let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

    // compute lamports for rent-exempt
    let lamports = client.get_minimum_balance_for_rent_exemption(POOL_STATE_SPACE)?;

    // Build create_account instruction for pool_state (owner will be changed to program_id)
    let create_acc_ix = system_instruction::create_account(
        &payer.pubkey(),
        &pool_state_kp.pubkey(),
        lamports,
        POOL_STATE_SPACE as u64,
        &program_id,
    );

    // Build InitializePool instruction data: tag 0 (no extra fields)
    let ix_data = vec![0u8];

    // Accounts required by program per design:
    // 0. payer (signer)
    // 1. pool_state (writable)
    // 2. admin (readonly)
    // 3. vault_x (writable)
    // 4. vault_y (writable)
    // 5. mint_x
    // 6. mint_y
    // 7. token_program
    let token_program = spl_token::id();

    let initialize_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new(pool_state_kp.pubkey(), false),
            AccountMeta::new_readonly(admin_pubkey, false),
            AccountMeta::new(vault_x, false),
            AccountMeta::new(vault_y, false),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new_readonly(token_program, false),
        ],
        data: ix_data,
    };

    // Build transaction (create account + initialize)
    let message = Message::new(&[create_acc_ix, initialize_ix], Some(&payer.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    let recent_blockhash = client.get_latest_blockhash()?;
    tx.try_sign(&[&payer, &pool_state_kp], recent_blockhash)?;

    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("InitializePool tx signature: {}", sig);
    println!("Pool state pubkey: {}", pool_state_kp.pubkey());

    // Derive PDA (informative)
    let (pda, bump) = Pubkey::find_program_address(&[b"pool", pool_state_kp.pubkey().as_ref()], &program_id);
    println!("Derived pool PDA: {} bump: {}", pda, bump);

    Ok(())
}
