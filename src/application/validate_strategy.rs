use crate::domain::strategy::StrategyPolicy;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate_strategy(policy: &StrategyPolicy, allowed_symbols: &[String]) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 1. Capital check
    if policy.capital_usdc < 10.0 {
        errors.push("Minimum capital requirement is $10 USDC.".to_string());
    }

    // 2. Total Weight Validation
    let total_weight: f64 = policy.assets.iter().map(|a| a.weight).sum();
    let total_allocation = total_weight + policy.reserve;
    
    // Allow small floating point variances
    if (total_allocation - 1.0).abs() > 0.001 {
        errors.push(format!("Total allocation (including reserve) must equal 100%. Current: {:.1}%", total_allocation * 100.0));
    }

    // 3. Asset Registry Verification
    for asset in &policy.assets {
        if !allowed_symbols.contains(&asset.symbol) {
            errors.push(format!("Asset {} is not a verified Base tokenized stock.", asset.symbol));
        }
        
        if asset.weight < 0.0 {
            errors.push(format!("Negative allocation found for {}. Shorting not supported in MVP.", asset.symbol));
        }
        
        if asset.weight > 0.50 {
            warnings.push(format!("High concentration risk: {} is > 50% of portfolio.", asset.symbol));
        }
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}