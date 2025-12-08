use crate::core::billing_engine::BillingEngine;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use std::error::Error;

pub async fn start_cron_service(
    billing_engine: Arc<BillingEngine>
) -> Result<JobScheduler, Box<dyn Error>> {

    let mut sched = JobScheduler::new().await?;

    // CRON EXPRESSION: "sec min hour day_of_month month day_of_week"
    // "0 0 9 1 * *" = At 09:00:00 AM, on the 1st day of every month.
    let cron_schedule = "0 0 9 1 * *"; 

    sched.add(
        Job::new_async(cron_schedule, move |_uuid, _l| {
            // Clone the Arc so we can move it into the async block
            let engine = billing_engine.clone();
            
            Box::pin(async move {
                if let Err(e) = engine.process_all_tenants().await {
                    eprintln!("CRITICAL: Billing Job Failed: {}", e);
                }
            })
        })?
    ).await?;

    // Start the scheduler in the background
    sched.start().await?;

    println!("⏰ Cron Scheduler Started. Next Billing: 1st of Month at 9am.");

    Ok(sched)
}