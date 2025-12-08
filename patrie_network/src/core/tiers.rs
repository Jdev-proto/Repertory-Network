use serde::{Deserialize, Serialize};
use std::fmt;

// --- CONSTANTS (The Price List) ---
pub const BASE_PLATFORM_FEE: f64 = 2500.00;
pub const ADMIN_FEE_HEALTH: f64 = 50.00;
pub const ADMIN_FEE_401K: f64 = 50.00;
pub const ADMIN_FEE_CRIME: f64 = 50.00;
pub const BRIDGE_EXIT_FEE_USD: f64 = 10.00;

/// The Official Tiers of the Platform
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ServiceTier {
    Starter,        // The OS
    Professional,   // The Employer
    Enterprise,     // The Compliance Stack
}

impl fmt::Display for ServiceTier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceTier::Starter => write!(f, "Starter (OS)"),
            ServiceTier::Professional => write!(f, "Professional (Employer)"),
            ServiceTier::Enterprise => write!(f, "Enterprise (Compliance)"),
        }
    }
}

/// A "Blueprint" of what features are enabled for each tier
#[derive(Debug, Serialize)]
pub struct TierConfiguration {
    pub tier_name: String,
    pub base_price: f64,
    pub includes_health: bool,
    pub includes_401k: bool,
    pub includes_crime_ins: bool,
    pub description: String,
}

impl ServiceTier {
    /// Returns the configuration for a specific tier
    pub fn get_config(&self) -> TierConfiguration {
        match self {
            ServiceTier::Starter => TierConfiguration {
                tier_name: self.to_string(),
                base_price: BASE_PLATFORM_FEE,
                includes_health: false,
                includes_401k: false,
                includes_crime_ins: false,
                description: "Blockchain Ledger + Banking Rails".to_string(),
            },
            ServiceTier::Professional => TierConfiguration {
                tier_name: self.to_string(),
                base_price: BASE_PLATFORM_FEE,
                includes_health: true, // Auto-enables Health Store Item
                includes_401k: false,
                includes_crime_ins: false,
                description: "Includes Managed Health Insurance".to_string(),
            },
            ServiceTier::Enterprise => TierConfiguration {
                tier_name: self.to_string(),
                base_price: BASE_PLATFORM_FEE,
                includes_health: true,
                includes_401k: true,
                includes_crime_ins: true,
                description: "Full Compliance Stack: Health, 401k, Fraud Protection".to_string(),
            },
        }
    }

    /// Helper: Calculate estimated revenue for YOU (excluding pass-through costs)
    pub fn estimated_profit(&self) -> f64 {
        match self {
            ServiceTier::Starter => BASE_PLATFORM_FEE,
            ServiceTier::Professional => BASE_PLATFORM_FEE + ADMIN_FEE_HEALTH,
            ServiceTier::Enterprise => BASE_PLATFORM_FEE + ADMIN_FEE_HEALTH + ADMIN_FEE_401K + ADMIN_FEE_CRIME,
        }
    }
}