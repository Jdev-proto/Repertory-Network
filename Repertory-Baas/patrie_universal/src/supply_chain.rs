use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use eyre::Result;
use std::collections::BTreeMap;

pub async fn track_shipment(client: &Client) -> Result<()> {
    println!("📦 Initializing Supply Chain...");

    // 1. Create a 'Logistics' Domain
    let domain_id: DomainId = "logistics".parse()?;
    client.submit(Register::domain(Domain::new(domain_id))).await?;

    // 2. Define a Container (Store/NFT type)
    // "Store" assets hold metadata but have no quantity math
    let container_def: AssetDefinitionId = "container#logistics".parse()?;
    client.submit(Register::asset_definition(AssetDefinition::store(container_def.clone()))).await?;

    // 3. Register a specific Container instance
    let alice_id: AccountId = "alice@wonderland".parse()?;
    let container_id = AssetId::new(container_def, alice_id.clone());
    
    // 4. Update Tracking Data (Metadata)
    // We store location and temp directly on the asset
    let mut metadata = BTreeMap::new();
    metadata.insert("location".parse()?, "Port of Tokyo".into());
    metadata.insert("temperature".parse()?, "-4C".into());

    let set_metadata = SetKeyValue::asset(container_id, "status".parse()?, metadata);
    client.submit(set_metadata).await?;
    
    println!("✅ Container Registered at Port of Tokyo");

    Ok(())
}