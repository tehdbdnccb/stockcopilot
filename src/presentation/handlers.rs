use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::domain::strategy::StrategyPolicy;
use crate::application::validate_strategy::{validate_strategy, ValidationResult};

#[derive(Deserialize)]
pub struct CompileRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct CompileResponse {
    pub policy: Option<StrategyPolicy>,
    pub validation: ValidationResult,
}

pub async fn compile_handler(
    Json(_payload): Json<CompileRequest>,
) -> Result<Json<CompileResponse>, (StatusCode, String)> {
    // 1. Fetch allowed assets from database/registry
    let allowed_assets = vec![
        "NVDAc".to_string(), "MSFTc".to_string(), "GOOGLc".to_string(),
        "AMZNc".to_string(), "METAc".to_string(), "AAPLc".to_string(),
    ];

    // 2. Call OpenAI/Gemini API using system prompt (Mocked for direct structure)
    // In production: parse natural language prompt with reqwest to LLM endpoint
    let parsed_policy = StrategyPolicy {
        name: "AI Infrastructure Basket".to_string(),
        capital_usdc: 500.0,
        reserve: 0.10,
        rebalance_days: 7,
        assets: vec![
            crate::domain::strategy::AssetAllocation { symbol: "NVDAc".to_string(), weight: 0.20 },
            crate::domain::strategy::AssetAllocation { symbol: "MSFTc".to_string(), weight: 0.20 },
            crate::domain::strategy::AssetAllocation { symbol: "GOOGLc".to_string(), weight: 0.20 },
            crate::domain::strategy::AssetAllocation { symbol: "AMZNc".to_string(), weight: 0.20 },
            crate::domain::strategy::AssetAllocation { symbol: "METAc".to_string(), weight: 0.10 },
        ],
        rules: vec![],
    };

    // 3. Deterministic Validation Step
    let validation = validate_strategy(&parsed_policy, &allowed_assets);

    Ok(Json(CompileResponse {
        policy: if validation.is_valid { Some(parsed_policy) } else { None },
        validation,
    }))
}

#[derive(Deserialize)]
pub struct ValidateRequest {
    pub policy: StrategyPolicy,
    pub allowed_symbols: Vec<String>,
}

pub async fn validate_handler(
    Json(payload): Json<ValidateRequest>,
) -> Json<ValidationResult> {
    let validation = validate_strategy(&payload.policy, &payload.allowed_symbols);
    Json(validation)
}
