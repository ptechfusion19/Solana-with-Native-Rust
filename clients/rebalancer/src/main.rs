use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{collections::HashSet, env, fs};
use tokio_tungstenite::connect_async;
use url::Url;

/// Reads a keypair file (solana-keygen JSON array)
fn read_keypair(path: &str) -> Result<Keypair> {
    let s = fs::read_to_string(path)?;
    let v: Vec<u8> = serde_json::from_str(&s)?;
    Ok(Keypair::from_bytes(&v)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Required env:
    // SOLANA_WS_URL (ws://127.0.0.1:8900)
    // SOLANA_URL (http://127.0.0.1:8899)
    // PROGRAM_ID
    // ADMIN_KEYPAIR (path)
    // ADMIN_TOKEN_Y (admin's token account for Y)
    // POOL_VAULT_Y, POOL_STATE_PUBKEY
    // TOPUP_AMOUNT (u64) - amount to top-up on each detected deposit (in smallest units)
    let ws_url = env::var("SOLANA_WS_URL").unwrap_or_else(|_| "ws://127.0.0.1:8900".to_string());
    let rpc_url = env::var("SOLANA_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let program_id: Pubkey = env::var("PROGRAM_ID")?.parse()?;
    let admin_kp = read_keypair(&env::var("ADMIN_KEYPAIR")?)?;
    let admin_token_y: Pubkey = env::var("ADMIN_TOKEN_Y")?.parse()?;
    let pool_vault_y: Pubkey = env::var("POOL_VAULT_Y")?.parse()?;
    let pool_state: Pubkey = env::var("POOL_STATE_PUBKEY")?.parse()?;
    let topup_amount: u64 = env::var("TOPUP_AMOUNT")?.parse()?;

    let rpc_client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

    // connect websocket
    let ws_url = Url::parse(&ws_url)?;
    let (ws_stream, _) = connect_async(ws_url).await?;
    println!("Connected to websocket");

    let (mut write, mut read) = ws_stream.split();

    // subscribe to logs mentioning the program id (confirmed commitment)
    let subscribe_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            { "mentions": [ program_id.to_string() ] },
            { "commitment": "confirmed" }
        ]
    });

    write
        .send(tokio_tungstenite::tungstenite::Message::Text(
            subscribe_req.to_string(),
        ))
        .await?;

    // in-memory dedupe set for deposit tx signatures
    let mut processed: HashSet<String> = HashSet::new();

    // read incoming messages
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
            // Parse JSON-RPC pubsub message
            let v: serde_json::Value = serde_json::from_str(&txt)?;
            // The logsSubscribe returns notifications with method "logsNotification" and params.result
            if v.get("method").and_then(|m| m.as_str()) == Some("logsNotification") {
                if let Some(params) = v.get("params") {
                    if let Some(result) = params.get("result") {
                        // extract signature (if available) and logs
                        let signature = result
                            .get("value")
                            .and_then(|val| val.get("signature"))
                            .and_then(|s| s.as_str())
                            .map(|s| s.to_string());
                        let logs = result
                            .get("value")
                            .and_then(|val| val.get("logs"))
                            .and_then(|l| l.as_array())
                            .cloned()
                            .unwrap_or_default();

                        // find our event
                        for log in logs {
                            if let Some(slog) = log.as_str() {
                                // matches pattern "EVENT:Deposit|user=... |amount=..."
                                if slog.starts_with("EVENT:Deposit") {
                                    println!("Detected event log: {}", slog);
                                    if let Some(sig) = signature.clone() {
                                        if processed.contains(&sig) {
                                            println!(
                                                "Already processed signature {}, skipping",
                                                sig
                                            );
                                            continue;
                                        }
                                        processed.insert(sig.clone());
                                    }

                                    // parse key=value pairs in log
                                    // e.g. "EVENT:Deposit|user=Pubkey |amount=123"
                                    let parts: Vec<_> = slog.split('|').collect();
                                    let mut user_opt = None;
                                    let mut amount_opt: Option<u64> = None;
                                    for p in parts.iter().skip(1) {
                                        let p = p.trim();
                                        if let Some(rest) = p.strip_prefix("user=") {
                                            user_opt = Some(rest.trim().to_string());
                                        } else if let Some(rest) = p.strip_prefix("amount=") {
                                            if let Ok(a) = rest.trim().parse::<u64>() {
                                                amount_opt = Some(a);
                                            }
                                        }
                                    }

                                    println!(
                                        "Parsed event: user={:?} amount={:?} sig={:?}",
                                        user_opt, amount_opt, signature
                                    );

                                    // React immediately: send AdminRebalance action=0 (top-up) signed by admin
                                    // Build instruction data: tag=2, action=0, amount=topup_amount
                                    let mut data = Vec::with_capacity(1 + 1 + 8);
                                    data.push(2u8); // AdminRebalance
                                    data.push(0u8); // action = top-up
                                    data.extend_from_slice(&topup_amount.to_le_bytes());

                                    // Accounts for AdminRebalance top-up:
                                    // 0. admin (signer)
                                    // 1. admin_token_y (writable)
                                    // 2. pool_vault_y (writable)
                                    // 3. pool_state (writable)
                                    // 4. token_program
                                    let token_program = spl_token::id();

                                    let ix = Instruction {
                                        program_id,
                                        accounts: vec![
                                            AccountMeta::new_readonly(admin_kp.pubkey(), true),
                                            AccountMeta::new(admin_token_y, false),
                                            AccountMeta::new(pool_vault_y, false),
                                            AccountMeta::new(pool_state, false),
                                            AccountMeta::new_readonly(token_program, false),
                                        ],
                                        data,
                                    };

                                    // send transaction
                                    let message = Message::new(&[ix], Some(&admin_kp.pubkey()));
                                    let mut tx = Transaction::new_unsigned(message);
                                    let recent_blockhash = rpc_client.get_latest_blockhash()?;
                                    tx.try_sign(&[&admin_kp], recent_blockhash)?;
                                    let sig = rpc_client.send_and_confirm_transaction(&tx)?;
                                    println!("Sent AdminRebalance tx sig: {}", sig);

                                    // optionally: you could also subscribe to signatureSubscribe for the deposit sig (not implemented here)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
