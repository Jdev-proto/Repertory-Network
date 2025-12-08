use actix_web::web;
use crate::api::handlers::{tenant, unit}; // Add 'unit' here

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Tenant Endpoints
            .service(tenant::create_tenant)
            
            // Unit (Asset) Endpoints
            .service(unit::define_unit)
            .service(unit::mint_unit)
    );
}