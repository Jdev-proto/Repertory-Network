use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value; // Added this missing import
use std::error::Error;

// --- Data Models ---

#[derive(Serialize)]
struct CreateCompanyRequest {
    user: UserInfo,
    company: CompanyInfo,
}

#[derive(Serialize)]
struct UserInfo {
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Serialize)]
struct CompanyInfo {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateCompanyResponse {
    pub company_uuid: String,
    pub access_token: String, // SAVE THIS! This is the key to their specific account
    pub refresh_token: String,
}

// --- The Client ---

pub struct GustoClient {
    http: Client,
    api_token: String, // Your "System" Token (Organization Level)
    base_url: String,
}

impl GustoClient {
    pub fn new(api_token: String) -> Self {
        Self {
            http: Client::new(),
            api_token,
            // Switch to "https://api.gusto.com" for PRODUCTION
            base_url: "https://api.gusto-demo.com".to_string(), 
        }
    }

    /// 1. CREATE THE CLIENT (The "Sign Up")
    /// This is the Money Maker. Once this succeeds, this company is permanently 
    /// linked to your ecosystem, and you begin earning revenue share.
    pub async fn create_partner_managed_company(
        &self, 
        company_name: &str, 
        admin_email: &str
    ) -> Result<CreateCompanyResponse, Box<dyn Error>> {
        
        let url = format!("{}/v1/partner_managed_companies", self.base_url);
        
        let payload = CreateCompanyRequest {
            user: UserInfo {
                first_name: "Admin".to_string(), // In real app, pass these in
                last_name: "User".to_string(),
                email: admin_email.to_string(),
            },
            company: CompanyInfo {
                name: company_name.to_string(),
            },
        };

        let resp = self.http.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let error_text = resp.text().await?;
            return Err(format!("Gusto Error: {}", error_text).into());
        }

        let data: CreateCompanyResponse = resp.json().await?;
        Ok(data)
    }

    /// 2. ADD HEALTH INSURANCE (The Upsell)
    /// This adds a benefit to the company. Gusto handles the carrier selection 
    /// (Blue Cross, United, etc.) via their UI Flows.
    pub async fn add_health_benefit(
        &self, 
        company_uuid: &str, 
        company_access_token: &str // Note: Use the COMPANY token, not your System token
    ) -> Result<(), Box<dyn Error>> {
        
        let url = format!("{}/v1/companies/{}/company_benefits", self.base_url, company_uuid);
        
        // ID '1' is usually Medical in Gusto's system.
        // Best practice: Query /v1/benefits first to get the exact ID for "Medical"
        let payload = serde_json::json!({
            "benefit_type": "1", 
            "active": true,
            "description": "Company Health Plan"
        });

        let resp = self.http.post(&url)
            .header("Authorization", format!("Bearer {}", company_access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let error_text = resp.text().await?;
            return Err(format!("Failed to add benefit: {}", error_text).into());
        }

        Ok(())
    }

    /// 3. GENERATE MAGIC LINK (Insurance Flow)
    /// This generates the URL you redirect the user to so they can select
    /// their specific insurance plan or crime insurance.
    pub async fn get_insurance_flow_url(
        &self, 
        company_uuid: &str,
        company_access_token: &str
    ) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/v1/companies/{}/flows", self.base_url, company_uuid);
        
        let payload = serde_json::json!({
            "flow_type": "benefits_setup" // checks if they need to pick a plan
        });

        let resp = self.http.post(&url)
            .header("Authorization", format!("Bearer {}", company_access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let error_text = resp.text().await?;
            return Err(format!("Failed to get flow URL: {}", error_text).into());
        }

        let json: Value = resp.json().await?;
        
        // Extract the URL safely
        match json["url"].as_str() {
            Some(link) => Ok(link.to_string()),
            None => Err("No URL found in Gusto response".into()),
        }
    }
}

// Helper: Update the wholesale cost in DB so the billing engine is accurate
pub async fn update_wholesale_cost(
    pool: &PgPool, 
    tenant_id: &str, 
    new_health_cost: f64, 
    new_401k_cost: f64
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE subscription_tiers 
        SET health_cost_wholesale = $1, retirement_cost_wholesale = $2
        WHERE tenant_id = $3
        "#,
        new_health_cost,
        new_401k_cost,
        uuid::Uuid::parse_str(tenant_id).unwrap()
    )
    .execute(pool)
    .await?;
    
    Ok(())
}