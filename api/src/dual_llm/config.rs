// Dual-LLM Configuration System
// Manages feature flags and LLM #1 settings

use serde::{Deserialize, Serialize};
use std::env;

/// Configuration for dual-LLM mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualLlmConfig {
    /// Enable dual-LLM mode (defaults to false for backward compatibility)
    pub enabled: bool,

    /// LLM #1 model (Unconscious processor)
    /// Examples: "openai/gpt-3.5-turbo", "anthropic/claude-3-5-haiku"
    pub unconscious_model: String,

    /// Timeout for LLM #1 calls in milliseconds
    pub llm1_timeout_ms: u64,

    /// Maximum retries for LLM #1 before fallback to Rust
    pub llm1_max_retries: u32,

    /// Enable fallback to Rust calculators on LLM #1 failure
    pub fallback_enabled: bool,
}

impl Default for DualLlmConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Default to classic mode (backward compatible)
            unconscious_model: "openai/gpt-3.5-turbo".to_string(),
            llm1_timeout_ms: 5000,
            llm1_max_retries: 2,
            fallback_enabled: true,
        }
    }
}

impl DualLlmConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // DUAL_LLM_MODE controls feature flag
        if let Ok(mode) = env::var("DUAL_LLM_MODE") {
            config.enabled = mode.to_lowercase() == "true" || mode == "1";
        }

        // UNCONSCIOUS_LLM_MODEL overrides default
        if let Ok(model) = env::var("UNCONSCIOUS_LLM_MODEL") {
            config.unconscious_model = model;
        }

        // LLM1_TIMEOUT_MS overrides default
        if let Ok(timeout) = env::var("LLM1_TIMEOUT_MS") {
            if let Ok(ms) = timeout.parse() {
                config.llm1_timeout_ms = ms;
            }
        }

        // LLM1_MAX_RETRIES overrides default
        if let Ok(retries) = env::var("LLM1_MAX_RETRIES") {
            if let Ok(n) = retries.parse() {
                config.llm1_max_retries = n;
            }
        }

        // DUAL_LLM_FALLBACK overrides default
        if let Ok(fallback) = env::var("DUAL_LLM_FALLBACK") {
            config.fallback_enabled = fallback.to_lowercase() == "true" || fallback == "1";
        }

        config
    }

    /// Create config with dual-LLM enabled (for testing)
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Self::default()
        }
    }

    /// Create config with dual-LLM disabled (classic mode, for testing)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_disabled() {
        let config = DualLlmConfig::default();
        assert!(
            !config.enabled,
            "Default config should have dual-LLM disabled"
        );
        assert!(
            config.fallback_enabled,
            "Fallback should be enabled by default"
        );
    }

    #[test]
    fn test_enabled_config() {
        let config = DualLlmConfig::enabled();
        assert!(config.enabled);
    }

    #[test]
    fn test_disabled_config() {
        let config = DualLlmConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_from_env_defaults_when_no_vars() {
        // Clear any existing env vars (in case they're set)
        std::env::remove_var("DUAL_LLM_MODE");
        std::env::remove_var("UNCONSCIOUS_LLM_MODEL");

        let config = DualLlmConfig::from_env();
        assert!(
            !config.enabled,
            "Should default to disabled when no env vars"
        );
        assert_eq!(config.unconscious_model, "openai/gpt-3.5-turbo");
        assert_eq!(config.llm1_timeout_ms, 5000);
        assert_eq!(config.llm1_max_retries, 2);
    }
}
