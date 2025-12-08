use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{env, fs};

fn read_keypair(path: &str) -> Result<Keypair> {
    let s = fs::read_to_string(path)?;
    let v: Vec<u8> = serde_json::from_str(&s)?;
    Ok(Keypair::from_bytes(&v)?)
}

fn main() -> Result<()> {
    // env vars:
    // SOLANA_URL, PROGRAM_ID, USER_KEYPAIR (signer for deposit), USER_TOKEN_X, USER_TOKEN_Y (destination for Y),
    // POOL_VAULT_X, POOL_VAULT_Y, POOL_STATE_PUBKEY, POOL_AUTHORITY (PDA pubkey)
    let rpc_url = env::var("SOLANA_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let program_id: Pubkey = env::var("PROGRAM_ID")?.parse()?;
    let user_kp = read_keypair(&env::var("USER_KEYPAIR")?)?;
    let user_token_x: Pubkey = env::var("USER_TOKEN_X")?.parse()?;
    let pool_vault_x: Pubkey = env::var("POOL_VAULT_X")?.parse()?;
    let pool_vault_y: Pubkey = env::var("POOL_VAULT_Y")?.parse()?;
    let user_token_y: Pubkey = env::var("USER_TOKEN_Y")?.parse()?;
    let pool_state: Pubkey = env::var("POOL_STATE_PUBKEY")?.parse()?;
    let pool_authority: Pubkey = env::var("POOL_AUTHORITY")?.parse()?;
    let amount: u64 = env::var("DEPOSIT_AMOUNT")?.parse()?; // number of smallest units

    let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

    // Build data: tag 1 + amount LE
    let mut data = Vec::with_capacity(1 + 8);
    data.push(1u8);
    data.extend_from_slice(&amount.to_le_bytes());

    // Accounts (order required by program)
    // 0: user_authority (signer)
    // 1: user_token_x (writable)
    // 2: pool_vault_x (writable)
    // 3: pool_vault_y (writable)
    // 4: user_token_y (writable)
    // 5: pool_state (writable)
    // 6: token_program
    // 7: pool_authority (readonly PDA)
    let token_program = spl_token::id();

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(user_kp.pubkey(), true),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(pool_vault_x, false),
            AccountMeta::new(pool_vault_y, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(pool_authority, false),
        ],
        data,
    };

    let message = Message::new(&[ix], Some(&user_kp.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    let recent_blockhash = client.get_latest_blockhash()?;
    tx.try_sign(&[&user_kp], recent_blockhash)?;
    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("Deposit tx signature: {}", sig);

    Ok(())
}
