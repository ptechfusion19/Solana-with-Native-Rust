use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::env;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct TokenAccountRaw {
    // little-endian byte arrays for fixed-size fields
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

fn sized() -> usize {
    std::mem::size_of::<TokenAccountRaw>() // should be 165
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let rpc_url = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("Provide RPC URL(mainnet-beta, devnet) as 1st arg");
    let acc_str = args
        .get(2)
        .expect("Provide token account pubkey as 2nd arg");

    let rpc = RpcClient::new(rpc_url.to_string());
    let acc_pubkey = acc_str
        .parse::<Pubkey>()
        .with_context(|| format!("Invalid pubkey: {}", acc_str))?;

    println!(
        "RPC: {}\nToken Account: {}\nExpected token account layout size: {} bytes\n",
        rpc_url,
        acc_pubkey,
        sized()
    );

    // fetching account data
    let data = rpc
        .get_account_data(&acc_pubkey)
        .with_context(|| format!("Failed to fetch account data for {}", acc_pubkey))?;

    if data.len() < sized() {
        anyhow::bail!(
            "Account data too small: {} bytes (need >= {})",
            data.len(),
            sized()
        );
    }

    let slice = &data[0..sized()];

    let token: &TokenAccountRaw = bytemuck::try_from_bytes(slice)
        .map_err(|e| anyhow::anyhow!("Failed to cast bytes into TokenAccountRaw: {}", e))?;

    // converting fields from little-endian byte arrays
    let amount = u64::from_le_bytes(token.amount);
    let delegate_option = u32::from_le_bytes(token.delegate_option);
    let is_native_option = u32::from_le_bytes(token.is_native_option);
    let is_native = u64::from_le_bytes(token.is_native);
    let delegated_amount = u64::from_le_bytes(token.delegated_amount);
    let close_authority_option = u32::from_le_bytes(token.close_authority_option);

    // pubkeys
    let mint = Pubkey::new_from_array(token.mint);
    let owner = Pubkey::new_from_array(token.owner);
    let delegate = Pubkey::new_from_array(token.delegate);
    let close_authority = Pubkey::new_from_array(token.close_authority);

    println!("mint: {}", mint);
    println!("owner: {}", owner);
    println!("amount (raw u64): {}", amount);
    println!("delegate_option: {}", delegate_option);
    if delegate_option != 0 {
        println!("delegate: {}", delegate);
    }
    println!("state: {}", token.state);
    println!("is_native_option: {}", is_native_option);
    if is_native_option != 0 {
        println!("is_native (wrapped lamports): {}", is_native);
    }
    println!("delegated_amount: {}", delegated_amount);
    println!("close_authority_option: {}", close_authority_option);
    if close_authority_option != 0 {
        println!("close_authority: {}", close_authority);
    }

    Ok(())
}
