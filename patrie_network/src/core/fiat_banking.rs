// In src/core/fiat_banking.rs

impl UnitClient {
    /// Generates a Barcode so the user can deposit cash at Walmart/CVS
    pub async fn generate_cash_deposit_barcode(
        &self, 
        user_id: &str, 
        account_id: &str
    ) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/cash-deposits/barcode", self.base_url);
        
        let payload = serde_json::json!({
            "data": {
                "type": "cashDepositBarcode",
                "attributes": {
                    "store": "GreenDotNetwork" // or specific retailer
                },
                "relationships": {
                    "customer": {
                        "data": { "type": "customer", "id": user_id }
                    },
                    "account": {
                        "data": { "type": "depositAccount", "id": account_id }
                    }
                }
            }
        });

        let resp = self.http.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send()
            .await?;

        // Extract the barcode image URL or numeric code
        let json: serde_json::Value = resp.json().await?;
        Ok(json["data"]["attributes"]["barcodeUrl"].as_str().unwrap().to_string())
    }
}