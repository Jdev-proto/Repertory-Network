use eyre::Result;
use iroha_client::client::{Client, ClientConfig};
use iroha_data_model::prelude::*;
use std::str::FromStr;

// Import our use case modules (defined below)
mod finance;
mod supply_chain;
mod identity;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Configure the Client
    // In a real app, load these from env vars or a config file.
    // Default "alice" admin credentials usually provided by the iroha2 Docker container:
    let api_url = "http://127.0.0.1:8080".parse()?;
    let account_id = AccountId::from_str("alice@wonderland")?;
    let key_pair = KeyPair::new(
        PublicKey::from_str("ed0120... (ALICE_PUBLIC_KEY_HERE)")?, 
        PrivateKey::from_str("... (ALICE_PRIVATE_KEY_HERE)")?
    )?;

    let config = ClientConfig::new(api_url, account_id, key_pair);
    let client = Client::new(config);

    println!("✅ Connected to Iroha 2 Network");

    // 2. Select Your Use Case
    // Uncomment the one you want to run:
    
    // finance::init_defi_system(&client).await?;
    // supply_chain::track_shipment(&client).await?;
    identity::issue_digital_id(&client).await?;

    Ok(())
}