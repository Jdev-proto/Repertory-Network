use actix_web::{get, post, web, HttpResponse, Responder};
use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState; // Import from your main.rs

// --- Request/Response Structs ---

#[derive(Deserialize)]
pub struct DefineUnitRequest {
    pub tenant_id: String,      // e.g., "tesla_supply_chain"
    pub unit_name: String,      // e.g., "battery_pack"
    pub unit_type: String,      // "Numeric" (Currency) or "Store" (NFT/Item)
    pub decimals: Option<u32>,  // Only for Numeric
}

#[derive(Deserialize)]
pub struct MintUnitRequest {
    pub tenant_id: String,
    pub unit_name: String,
    pub quantity: f64,          // e.g., 100.0
    pub recipient: String,      // e.g., "elon"
}

// --- API Endpoints ---

/// 1. Define a new Unit type (AssetDefinition)
#[post("/unit/define")]
pub async fn define_unit(
    req: web::Json<DefineUnitRequest>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let client = &data.iroha_client;

    // Construct the AssetDefinitionId (e.g., "battery_pack#tesla_supply_chain")
    let asset_def_str = format!("{}#{}", req.unit_name, req.tenant_id);
    let asset_def_id: AssetDefinitionId = match asset_def_str.parse() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid name format"),
    };

    // Determine type: Numeric (Currency) or Store (NFT/Data)
    let definition = if req.unit_type.to_lowercase() == "numeric" {
        // Numeric assets have decimal precision
        AssetDefinition::numeric(asset_def_id).mintable()
    } else {
        // Store assets are for things like specific containers or metadata
        AssetDefinition::store(asset_def_id).mintable()
    };

    let instruction = Register::asset_definition(definition);

    match client.submit(instruction).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "Unit defined", "id": asset_def_str})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Iroha Error: {}", e)),
    }
}

/// 2. Mint Units (Create new supply)
#[post("/unit/mint")]
pub async fn mint_unit(
    req: web::Json<MintUnitRequest>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let client = &data.iroha_client;

    // Target Asset: "battery_pack#tesla_supply_chain"
    let asset_def_str = format!("{}#{}", req.unit_name, req.tenant_id);
    
    // Target Account: "elon@tesla_supply_chain"
    // Note: In a real app, ensure this account exists first!
    let recipient_account = format!("{}@{}", req.recipient, req.tenant_id);
    let account_id: AccountId = recipient_account.parse().unwrap();
    let asset_def_id: AssetDefinitionId = asset_def_str.parse().unwrap();

    // Construct the Mint Instruction
    let asset_id = AssetId::new(asset_def_id, account_id);
    let mint_instruction = Mint::asset_numeric(req.quantity, asset_id);

    match client.submit(mint_instruction).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "Minted", "amount": req.quantity})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Mint Failed: {}", e)),
    }
}