use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use eyre::Result;

pub struct InsuranceBroker;

impl InsuranceBroker {
    /// This function builds the "Atomic Swap" transaction.
    /// It ensures the policy is ONLY issued if you get your commission.
    pub async fn purchase_policy_with_commission(
        client: &Client,
        user_id: AccountId,
        insurer_id: AccountId,
        platform_commission_account: AccountId, // Your wallet
        premium_total: f64,
        commission_rate: f64, // e.g., 0.10 for 10%
    ) -> Result<()> {
        
        // 1. Calculate the Split
        let commission_amount = premium_total * commission_rate;
        let insurer_amount = premium_total - commission_amount;

        // Define the assets
        // Assuming "usd#bank" is the currency
        let currency_def: AssetDefinitionId = "usd#bank".parse()?;
        let policy_def: AssetDefinitionId = "health_policy#insurer_a".parse()?;

        // 2. Build the Instructions
        
        // A. User pays the Insurer (The Net Premium)
        let pay_insurer = Transfer::asset_numeric(
            AssetId::new(currency_def.clone(), user_id.clone()),
            insurer_amount,
            insurer_id.clone(),
        );

        // B. User pays YOU (The Commission)
        let pay_commission = Transfer::asset_numeric(
            AssetId::new(currency_def.clone(), user_id.clone()),
            commission_amount,
            platform_commission_account.clone(),
        );

        // C. Insurer issues the Policy to the User
        // Note: In Iroha 2, the Insurer must have previously granted permission 
        // for this specific brokerage app to mint/transfer on their behalf, 
        // OR the Insurer signs this transaction as a co-signer.
        // For simplicity, we assume the broker has "Mint" rights for this specific asset.
        let mint_policy = Mint::asset_store(
            Metadata::new(), // You can add policy details (expiry, coverage) here
            AssetId::new(policy_def, user_id.clone()),
        );

        // 3. Bundle into ONE Atomic Transaction
        // If the user lacks funds for EITHER payment, the Policy is never minted.
        let transaction = client.build_transaction(
            vec![
                pay_insurer.into(),
                pay_commission.into(),
                mint_policy.into(),
            ],
            None,
        );

        // 4. Submit
        client.submit_transaction(transaction).await?;
        
        println!("✅ Policy Sold. Commission of ${} earned.", commission_amount);
        Ok(())
    }
}