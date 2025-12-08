use actix_web::{get, post, web, HttpResponse, Responder};
use crate::ledger::client::IrohaClient;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct TransferRequest {
    pub sender_id: String,
    pub private_key: String, // In prod, use session-based signing!
    pub recipient_id: String,
    pub asset_id: String,    // e.g. "usd#patrie"
    pub amount: f64,
}

/// 1. Get Wallet Balance
#[get("/wallet/{account_id}/balance")]
pub async fn get_balance(
    path: web::Path<String>,
    iroha: web::Data<Arc<IrohaClient>>,
) -> impl Responder {
    let account_id = path.into_inner();
    
    // Query Iroha for all assets owned by this account
    match iroha.query_all_balances(&account_id).await {
        Ok(balances) => HttpResponse::Ok().json(balances),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// 2. Send Tokens
#[post("/wallet/transfer")]
pub async fn send_tokens(
    req: web::Json<TransferRequest>,
    iroha: web::Data<Arc<IrohaClient>>,
) -> impl Responder {
    
    match iroha.transfer_asset(
        &req.sender_id, 
        &req.private_key, 
        &req.recipient_id, 
        &req.asset_id, 
        req.amount
    ).await {
        Ok(tx_hash) => HttpResponse::Ok().json(serde_json::json!({"status": "Sent", "tx_hash": tx_hash})),
        Err(e) => HttpResponse::BadRequest().body(format!("Transfer Failed: {}", e)),
    }
}