#[post("/insurance/buy")]
async fn buy_policy(
    req: web::Json<BuyPolicyRequest>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    // 1. Fixed Configuration
    let commission_rate = 0.15; // You take 15%
    let my_wallet = "admin@my_ecosystem".parse().unwrap();
    let insurer_wallet = "finance@blue_cross".parse().unwrap();

    // 2. Execute the Atomic Purchase
    let result = InsuranceBroker::purchase_policy_with_commission(
        &data.iroha_client,
        req.user_id.parse().unwrap(),
        insurer_wallet,
        my_wallet,
        req.price,
        commission_rate
    ).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "Success", "commission_earned": req.price * commission_rate})),
        Err(e) => HttpResponse::BadRequest().body(format!("Transaction failed: {}", e))
    }
}