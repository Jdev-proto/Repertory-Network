pub async fn full_onboarding(
    req: web::Json<SignupRequest>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    
    // Step 1: Create Gusto Account (Triggers your Commission)
    let gusto_data = match data.gusto_client.create_partner_managed_company(
        &req.company_name, 
        &req.email
    ).await {
        Ok(d) => d,
        Err(e) => return HttpResponse::BadRequest().body(format!("Gusto Setup Failed: {}", e)),
    };

    // Step 2: Store their tokens securely (Vault or DB)
    // You need 'gusto_data.access_token' to manage their benefits later.
    save_tokens_to_db(&req.company_name, &gusto_data.access_token);

    // Step 3: Create Iroha 2 Domain (The Blockchain Space)
    // We link the Iroha Domain Name to the Gusto Company UUID for tracking.
    let domain_id = req.company_name.to_lowercase();
    let register_domain = Register::domain(Domain::new(domain_id.parse().unwrap()));
    
    // ... Submit to Iroha ...

    HttpResponse::Ok().json(serde_json::json!({
        "status": "Onboarding Complete", 
        "gusto_uuid": gusto_data.company_uuid,
        "iroha_domain": domain_id
    }))
}

use crate::core::tiers::ServiceTier;

// ... inside your signup handler ...

// 1. User selected "Enterprise" in the UI
let selected_tier = ServiceTier::Enterprise;
let config = selected_tier.get_config();

// 2. Save these settings to your Database
sqlx::query!(
    r#"
    INSERT INTO subscription_settings 
    (tenant_id, base_fee_retail, health_active, retirement_active, crime_active)
    VALUES ($1, $2, $3, $4, $5)
    "#,
    tenant_id,
    config.base_price,        // 2500.00
    config.includes_health,   // true
    config.includes_401k,     // true
    config.includes_crime_ins // true
)
.execute(&pool)
.await?;