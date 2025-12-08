use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/explorer/transactions")]
pub async fn get_recent_transactions(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query!(
        "SELECT tx_hash, sender_account_id, command_type, timestamp FROM chain_transactions ORDER BY timestamp DESC LIMIT 20"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    let txs: Vec<_> = rows.iter().map(|r| serde_json::json!({
        "hash": r.tx_hash,
        "from": r.sender_account_id,
        "type": r.command_type,
        "time": r.timestamp.to_string()
    })).collect();

    HttpResponse::Ok().json(txs)
}