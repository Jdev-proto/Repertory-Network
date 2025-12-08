use crate::core::fiat_banking::UnitClient;
use sqlx::{PgPool, Row};
use std::error::Error;

pub struct BillingEngine {
    db: PgPool,
    unit: UnitClient,
    my_revenue_account_id: String,
}

impl BillingEngine {
    
    /// Run this on the 1st of the month.
    /// ACCURATE, COMPLIANT BILLING ENGINE
    pub async fn process_monthly_invoice(&self, tenant_id: &str) -> Result<(), Box<dyn Error>> {
        
        // 1. Fetch Tenant Settings & Wholesale Costs
        // We look at the 'subscription_settings' table (The Store)
        let rec = sqlx::query!(
            r#"
            SELECT 
                t.unit_deposit_account_id,
                s.base_fee_retail,
                s.health_active, s.health_cost_wholesale,
                s.retirement_active, s.retirement_cost_wholesale,
                s.crime_active, s.crime_cost_wholesale
            FROM tenants t
            JOIN subscription_settings s ON t.id = s.tenant_id
            WHERE t.id = $1
            "#,
            uuid::Uuid::parse_str(tenant_id)?
        )
        .fetch_one(&self.db)
        .await?;

        // --- THE CALCULATOR ---
        
        // A. Start with Base Fee ($2,500.00)
        let mut total_charge = rec.base_fee_retail.unwrap_or(2500.00); 
        
        // We build a "Legal Receipt" string to store in your logs/email
        let mut detailed_receipt = format!("Base Platform Access: ${:.2}", total_charge);
        
        // We build a "Bank Statement" string (shorter) for Unit
        let mut bank_desc = String::from("Monthly SaaS Bundle");

        // B. Health Insurance (Split: Premium + Tech Fee)
        if rec.health_active.unwrap_or(false) {
            let cost = rec.health_cost_wholesale.unwrap_or_default(); // e.g. $400.00
            let admin_fee = 50.00;
            
            total_charge += cost + admin_fee;
            
            // COMPLIANCE FIX: List the fee separately
            detailed_receipt.push_str(&format!("\n + Health Premium (Pass-through): ${:.2}", cost));
            detailed_receipt.push_str(&format!("\n + Health Integration Fee: ${:.2}", admin_fee));
        }

        // C. 401k (Split: Contribution + Data Fee)
        if rec.retirement_active.unwrap_or(false) {
            let cost = rec.retirement_cost_wholesale.unwrap_or_default(); // e.g. $80.00
            let admin_fee = 50.00;
            
            total_charge += cost + admin_fee;
            
            detailed_receipt.push_str(&format!("\n + 401k Contribution: ${:.2}", cost));
            detailed_receipt.push_str(&format!("\n + 401k Data Connection Fee: ${:.2}", admin_fee));
        }

        // D. Crime Insurance (Split: Premium + Tech Fee)
        if rec.crime_active.unwrap_or(false) {
            let cost = rec.crime_cost_wholesale.unwrap_or_default(); // e.g. $30.00
            let admin_fee = 50.00;
            
            total_charge += cost + admin_fee;
            
            detailed_receipt.push_str(&format!("\n + Crime Ins Premium: ${:.2}", cost));
            detailed_receipt.push_str(&format!("\n + Crime Ins Admin Fee: ${:.2}", admin_fee));
        }

        // --- EXECUTION ---

        println!("🧾 INVOICE GENERATED FOR TENANT {}:", tenant_id);
        println!("{}", detailed_receipt);
        println!("--------------------------------");
        println!("   GRAND TOTAL: ${:.2}", total_charge);

        // Convert to cents for Unit API (e.g. $3080.00 -> 308000)
        let amount_cents = (total_charge * 100.0) as u64; 
        
        // Pull the money instantly
        // Note: The 'description' here appears on their bank statement.
        // We keep it generic but accurate: "Monthly SaaS Bundle"
        self.unit.create_book_payment(
            &rec.unit_deposit_account_id.unwrap(),
            &self.my_revenue_account_id,
            amount_cents,
            &bank_desc 
        ).await?;

        Ok(())
    }
}