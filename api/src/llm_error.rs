// Production LLM Error Handling
// Comprehensive error types for all LLM provider operations

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
    ConfigError { message: String },

    /// Unsupported provider in factory
    UnsupportedProvider { provider_name: String },

    /// Rate limiting or quota exceeded
    RateLimitError {
        message: String,
        retry_after: Option<u64>,
    },

    /// Authentication/authorization errors
    AuthError { message: String },
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlmError::NetworkError {
                message,
                status_code,
            } => {
                write!(f, "Network error: {} (status: {:?})", message, status_code)
            }
            LlmError::JsonParseError {
                message,
                raw_response,
            } => {
                write!(f, "JSON parse error: {} (raw: {:?})", message, raw_response)
            }
            LlmError::ApiError {
                message,
                error_type,
                status_code,
            } => {
                write!(
                    f,
                    "API error: {} (type: {:?}, status: {:?})",
                    message, error_type, status_code
                )
            }
            LlmError::InvalidResponseFormat {
                field,
                message,
                raw_response,
            } => {
                write!(
                    f,
                    "Invalid response format - field '{}': {} (raw: {:?})",
                    field, message, raw_response
                )
            }
            LlmError::ConfigError { message } => {
                write!(f, "Configuration error: {}", message)
            }
            LlmError::UnsupportedProvider { provider_name } => {
                write!(f, "Unsupported provider: {}", provider_name)
            }
            LlmError::RateLimitError {
                message,
                retry_after,
            } => {
                write!(
                    f,
                    "Rate limit error: {} (retry after: {:?}s)",
                    message, retry_after
                )
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
                Some(code) if (400..500).contains(&code) => LlmError::ApiError {
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
    fn test_error_from_reqwest() {
        // Test reqwest::Error conversion through serde_json error
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let llm_err: LlmError = json_err.into();

        match llm_err {
            LlmError::JsonParseError { message, .. } => {
                assert!(message.contains("JSON parsing failed"));
            }
            _ => panic!("Expected JsonParseError"),
        }
    }

    #[test]
    fn test_unsupported_provider_error() {
        let err = LlmError::UnsupportedProvider {
            provider_name: "unknown".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("Unsupported provider"));
        assert!(display.contains("unknown"));
    }

    #[test]
    fn test_auth_error() {
        let err = LlmError::AuthError {
            message: "Invalid API key".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("Authentication error"));
        assert!(display.contains("Invalid API key"));
    }
}
