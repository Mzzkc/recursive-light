// PROTOTYPE: LLM Error Handling
// This file demonstrates the proposed error handling pattern for all LLM providers
// Status: Validation phase - DO NOT USE IN PRODUCTION

use serde::{Deserialize, Serialize};
use std::fmt;

/// Comprehensive error type for all LLM provider operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmError {
    /// Network/HTTP errors from reqwest
    NetworkError {
        message: String,
        status_code: Option<u16>,
    },

    /// JSON parsing or deserialization errors
    JsonParseError {
        message: String,
        raw_response: Option<String>,
    },

    /// API returned error response (4xx/5xx with error message)
    ApiError {
        message: String,
        error_type: Option<String>,
        status_code: Option<u16>,
    },

    /// Missing or invalid field in API response
    InvalidResponseFormat {
        field: String,
        message: String,
        raw_response: Option<String>,
    },

    /// Provider configuration error (missing API key, invalid model, etc.)
    ConfigError {
        message: String,
    },

    /// Unsupported provider in factory
    UnsupportedProvider {
        provider_name: String,
    },

    /// Rate limiting or quota exceeded
    RateLimitError {
        message: String,
        retry_after: Option<u64>,
    },

    /// Authentication/authorization errors
    AuthError {
        message: String,
    },
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlmError::NetworkError { message, status_code } => {
                write!(f, "Network error: {} (status: {:?})", message, status_code)
            }
            LlmError::JsonParseError { message, raw_response } => {
                write!(f, "JSON parse error: {} (raw: {:?})", message, raw_response)
            }
            LlmError::ApiError { message, error_type, status_code } => {
                write!(f, "API error: {} (type: {:?}, status: {:?})", message, error_type, status_code)
            }
            LlmError::InvalidResponseFormat { field, message, raw_response } => {
                write!(f, "Invalid response format - field '{}': {} (raw: {:?})", field, message, raw_response)
            }
            LlmError::ConfigError { message } => {
                write!(f, "Configuration error: {}", message)
            }
            LlmError::UnsupportedProvider { provider_name } => {
                write!(f, "Unsupported provider: {}", provider_name)
            }
            LlmError::RateLimitError { message, retry_after } => {
                write!(f, "Rate limit error: {} (retry after: {:?}s)", message, retry_after)
            }
            LlmError::AuthError { message } => {
                write!(f, "Authentication error: {}", message)
            }
        }
    }
}

impl std::error::Error for LlmError {}

// Automatic conversion from reqwest::Error to LlmError
impl From<reqwest::Error> for LlmError {
    fn from(err: reqwest::Error) -> Self {
        let status_code = err.status().map(|s| s.as_u16());

        // Classify reqwest errors into appropriate LlmError variants
        if err.is_timeout() || err.is_connect() {
            LlmError::NetworkError {
                message: format!("Network error: {}", err),
                status_code,
            }
        } else if err.is_status() {
            // Check for specific status codes
            match status_code {
                Some(401) | Some(403) => LlmError::AuthError {
                    message: format!("Authentication failed: {}", err),
                },
                Some(429) => LlmError::RateLimitError {
                    message: format!("Rate limit exceeded: {}", err),
                    retry_after: None,
                },
                Some(code) if code >= 400 && code < 500 => LlmError::ApiError {
                    message: format!("Client error: {}", err),
                    error_type: Some("client_error".to_string()),
                    status_code,
                },
                Some(code) if code >= 500 => LlmError::ApiError {
                    message: format!("Server error: {}", err),
                    error_type: Some("server_error".to_string()),
                    status_code,
                },
                _ => LlmError::NetworkError {
                    message: format!("HTTP error: {}", err),
                    status_code,
                },
            }
        } else if err.is_decode() {
            LlmError::JsonParseError {
                message: format!("Failed to decode response: {}", err),
                raw_response: None,
            }
        } else {
            LlmError::NetworkError {
                message: format!("Request error: {}", err),
                status_code,
            }
        }
    }
}

// Conversion from serde_json::Error to LlmError
impl From<serde_json::Error> for LlmError {
    fn from(err: serde_json::Error) -> Self {
        LlmError::JsonParseError {
            message: format!("JSON parsing failed: {}", err),
            raw_response: None,
        }
    }
}

// PROTOTYPE IMPLEMENTATION: Updated trait definition
// Note: This changes the trait signature - see compatibility analysis below

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[async_trait]
pub trait LlmProviderPrototype {
    fn get_api_key(&self) -> String;
    fn get_provider_name(&self) -> String;
    fn get_model_name(&self) -> String;

    // CHANGED: Result<String, LlmError> instead of Result<String, reqwest::Error>
    async fn send_request(&self, prompt: &str) -> Result<String, LlmError>;
}

// PROTOTYPE: Refactored OpenRouter implementation
pub struct OpenRouterLlmPrototype {
    api_key: String,
    model_name: String,
    client: Client,
}

impl OpenRouterLlmPrototype {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            api_key,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProviderPrototype for OpenRouterLlmPrototype {
    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    fn get_provider_name(&self) -> String {
        "openrouter".to_string()
    }

    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }

    async fn send_request(&self, prompt: &str) -> Result<String, LlmError> {
        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": self.model_name,
                "messages": [{"role": "user", "content": prompt}],
            }))
            .send()
            .await?; // Automatically converts reqwest::Error to LlmError

        let response_json: serde_json::Value = response.json().await?;

        // FIXED: Proper error handling instead of unwrap()
        response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| LlmError::InvalidResponseFormat {
                field: "choices[0].message.content".to_string(),
                message: "Expected string content in response".to_string(),
                raw_response: Some(response_json.to_string()),
            })
            .map(|s| s.to_string())
    }
}

// PROTOTYPE: Refactored OpenAI implementation
pub struct OpenAiLlmPrototype {
    api_key: String,
    model_name: String,
    client: Client,
}

impl OpenAiLlmPrototype {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            api_key,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProviderPrototype for OpenAiLlmPrototype {
    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    fn get_provider_name(&self) -> String {
        "openai".to_string()
    }

    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }

    async fn send_request(&self, prompt: &str) -> Result<String, LlmError> {
        let response = self
            .client
            .post("https://api.openai.com/v1/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model_name,
                "prompt": prompt,
                "max_tokens": 1024,
            }))
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;

        // FIXED: Proper error handling instead of fallback to "Invalid response format"
        response_json["choices"][0]["text"]
            .as_str()
            .ok_or_else(|| LlmError::InvalidResponseFormat {
                field: "choices[0].text".to_string(),
                message: "Expected text field in response".to_string(),
                raw_response: Some(response_json.to_string()),
            })
            .map(|s| s.to_string())
    }
}

// PROTOTYPE: Refactored Anthropic implementation
pub struct AnthropicLlmPrototype {
    api_key: String,
    model_name: String,
    client: Client,
}

impl AnthropicLlmPrototype {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            api_key,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProviderPrototype for AnthropicLlmPrototype {
    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    fn get_provider_name(&self) -> String {
        "anthropic".to_string()
    }

    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }

    async fn send_request(&self, prompt: &str) -> Result<String, LlmError> {
        let response = self
            .client
            .post("https://api.anthropic.com/v1/complete")
            .header("X-Api-Key", self.api_key.clone())
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": self.model_name,
                "prompt": format!("Human: {}\n\nAssistant:", prompt),
                "max_tokens_to_sample": 1024,
            }))
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;

        // FIXED: Proper error handling instead of unwrap()
        response_json["completion"]
            .as_str()
            .ok_or_else(|| LlmError::InvalidResponseFormat {
                field: "completion".to_string(),
                message: "Expected completion field in response".to_string(),
                raw_response: Some(response_json.to_string()),
            })
            .map(|s| s.to_string())
    }
}

// PROTOTYPE: Updated factory with proper error handling
pub struct LlmFactoryPrototype;

impl LlmFactoryPrototype {
    pub fn create_llm(
        provider_name: &str,
        api_key: String,
        model_name: String,
    ) -> Result<Box<dyn LlmProviderPrototype>, LlmError> {
        match provider_name {
            "openai" => Ok(Box::new(OpenAiLlmPrototype::new(api_key, model_name))),
            "anthropic" => Ok(Box::new(AnthropicLlmPrototype::new(api_key, model_name))),
            "openrouter" => Ok(Box::new(OpenRouterLlmPrototype::new(api_key, model_name))),
            _ => Err(LlmError::UnsupportedProvider {
                provider_name: provider_name.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LlmError::InvalidResponseFormat {
            field: "test".to_string(),
            message: "Missing field".to_string(),
            raw_response: None,
        };

        let display = format!("{}", err);
        assert!(display.contains("Invalid response format"));
        assert!(display.contains("test"));
    }

    #[test]
    fn test_unsupported_provider() {
        let result = LlmFactoryPrototype::create_llm(
            "unsupported",
            "key".to_string(),
            "model".to_string(),
        );

        assert!(result.is_err());
        match result {
            Err(LlmError::UnsupportedProvider { provider_name }) => {
                assert_eq!(provider_name, "unsupported");
            }
            _ => panic!("Expected UnsupportedProvider error"),
        }
    }

    #[test]
    fn test_error_from_serde_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json")
            .unwrap_err();
        let llm_err: LlmError = json_err.into();

        match llm_err {
            LlmError::JsonParseError { message, .. } => {
                assert!(message.contains("JSON parsing failed"));
            }
            _ => panic!("Expected JsonParseError"),
        }
    }
}
