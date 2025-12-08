use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct ExplorerIndexer {
    db: PgPool,
    iroha: Arc<Client>,
}

impl ExplorerIndexer {
    /// Starts the indexing loop
    pub async fn start_syncing(&self) {
        println!("🔍 Explorer Indexer Started...");
        
        let mut last_height = self.get_last_indexed_height().await;

        loop {
            // Poll for the next block
            let next_height = last_height + 1;
            
            // Note: In real Iroha 2, you use the Event Stream. 
            // For simplicity, we use a mock "get_block" query here.
            if let Ok(Some(block)) = self.fetch_block_from_iroha(next_height).await {
                
                self.save_block_to_db(&block).await;
                last_height = next_height;
                println!("📦 Indexed Block #{}", last_height);
                
            } else {
                // No new block, wait 2 seconds
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    async fn save_block_to_db(&self, block: &Block) {
        // 1. Insert Block
        sqlx::query!(
            "INSERT INTO chain_blocks (block_height, block_hash) VALUES ($1, $2)",
            block.header.height as i64,
            block.hash().to_string()
        )
        .execute(&self.db).await.unwrap();

        // 2. Insert Transactions
        for tx in &block.transactions {
            sqlx::query!(
                "INSERT INTO chain_transactions (tx_hash, block_height, sender_account_id, command_type, payload) VALUES ($1, $2, $3, $4, $5)",
                tx.hash().to_string(),
                block.header.height as i64,
                tx.payload.account_id.to_string(),
                "Instruction", // Simplify for demo
                serde_json::json!(format!("{:?}", tx.payload.instructions))
            )
            .execute(&self.db).await.unwrap();
        }
    }

    async fn get_last_indexed_height(&self) -> u64 {
        // Query DB for max(block_height)
        0 // Placeholder
    }
    
    // Mock wrapper for Iroha Client
    async fn fetch_block_from_iroha(&self, _height: u64) -> Result<Option<Block>, ()> { Ok(None) }
}