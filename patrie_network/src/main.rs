mod cron; // Register the new module

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Setup Database & Clients
    let db_pool = PgPoolOptions::new().connect("postgres://...").await.unwrap();
    let unit_client = UnitClient::new("...".to_string());
    
    // 2. Create the Billing Engine
    let billing_engine = Arc::new(BillingEngine::new(
        db_pool.clone(),
        unit_client,
        "YOUR_LLC_REVENUE_ACCOUNT_ID".to_string()
    ));

    // 3. Start the Cron Service
    // We handle the error here so the app doesn't crash if the scheduler fails
    match cron::start_cron_service(billing_engine.clone()).await {
        Ok(_) => println!("✅ Background jobs running..."),
        Err(e) => eprintln!("❌ Failed to start cron: {}", e),
    }

    // 4. Start Web Server
    HttpServer::new(move || {
        App::new()
            // ... your routes ...
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}