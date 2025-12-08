use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use eyre::Result;

pub async fn init_defi_system(client: &Client) -> Result<()> {
    println!("🏦 Initializing DeFi System...");

    // 1. Register a 'Bank' Domain
    let domain_id: DomainId = "central_bank".parse()?;
    client.submit(Register::domain(Domain::new(domain_id.clone()))).await?;

    // 2. Define USD (Fungible, Numeric)
    // "Numeric" allows for decimal precision (unlike "Store" assets)
    let usd_def_id: AssetDefinitionId = "usd#central_bank".parse()?;
    let register_usd = Register::asset_definition(AssetDefinition::numeric(usd_def_id.clone()).mintable());
    client.submit(register_usd).await?;

    // 3. Mint Money to Alice
    let alice_id: AccountId = "alice@wonderland".parse()?;
    let mint_tx = Mint::asset_numeric(
        1_000_000.0, 
        AssetId::new(usd_def_id, alice_id)
    );
    
    client.submit(mint_tx).await?;
    println!("✅ Minted $1,000,000 USD to Alice");
    
    Ok(())
}