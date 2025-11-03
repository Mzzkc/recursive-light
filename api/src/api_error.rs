// Wave 2: Comprehensive API Error Types
// Using miette (by Kat Marchán, they/them) for beautiful diagnostics
// Using thiserror for ergonomic error definitions

use miette::Diagnostic;
use thiserror::Error;

/// Main API error type with rich diagnostics
#[derive(Error, Debug, Diagnostic)]
pub enum ApiError {
    /// Database operation failed
    #[error("Database error: {message}")]
    #[diagnostic(
        code(api::database),
        help("Check database connection and schema migrations")
    )]
    Database {
        message: String,
        #[source]
        source: sqlx::Error,
    },

    /// LLM provider error (network, auth, rate limit, etc.)
    #[error("LLM provider error: {message}")]
    #[diagnostic(code(api::llm), help("Check API keys and network connectivity"))]
    LlmProvider {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// JSON parsing/serialization error
    #[error("JSON error: {message}")]
    #[diagnostic(
        code(api::json),
        help("Check that data structures match expected format")
    )]
    Json {
        message: String,
        #[source]
        source: serde_json::Error,
    },

    /// Configuration error (missing env vars, invalid config, etc.)
    #[error("Configuration error: {message}")]
    #[diagnostic(
        code(api::config),
        help("Check environment variables and configuration files")
    )]
    Configuration { message: String },

    /// Domain/boundary calculation error
    #[error("Flow processing error: {stage} - {reason}")]
    #[diagnostic(
        code(api::flow),
        help("Check input message and domain activation logic")
    )]
    FlowProcessing { stage: String, reason: String },

    /// Memory tier operation error
    #[error("Memory operation error: {operation} - {reason}")]
    #[diagnostic(
        code(api::memory),
        help("Check session state and memory tier transitions")
    )]
    Memory { operation: String, reason: String },

    /// Invalid input from user
    #[error("Invalid input: {message}")]
    #[diagnostic(
        code(api::input),
        help("Check that input meets validation requirements")
    )]
    InvalidInput { message: String },

    /// Resource not found (session, user, snapshot, etc.)
    #[error("{resource_type} not found: {identifier}")]
    #[diagnostic(
        code(api::not_found),
        help("Verify the resource exists in the database")
    )]
    NotFound {
        resource_type: String,
        identifier: String,
    },

    /// Internal error (should not happen in normal operation)
    #[error("Internal error: {message}")]
    #[diagnostic(code(api::internal), help("This is a bug - please report it"))]
    Internal { message: String },
}

/// Result type alias for API operations
pub type ApiResult<T> = miette::Result<T, ApiError>;

// Conversion implementations for common error types

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Database {
            message: err.to_string(),
            source: err,
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Json {
            message: err.to_string(),
            source: err,
        }
    }
}

impl From<crate::llm_error::LlmError> for ApiError {
    fn from(err: crate::llm_error::LlmError) -> Self {
        ApiError::LlmProvider {
            message: err.to_string(),
            source: Box::new(err),
        }
    }
}

impl From<crate::flow_process::FlowError> for ApiError {
    fn from(err: crate::flow_process::FlowError) -> Self {
        use crate::flow_process::FlowError;
        match err {
            FlowError::StageProcessingFailed { stage, reason } => {
                ApiError::FlowProcessing { stage, reason }
            }
        }
    }
}

// Helper methods for common error scenarios

impl ApiError {
    /// Create a configuration error
    pub fn config(message: impl Into<String>) -> Self {
        ApiError::Configuration {
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(resource_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        ApiError::NotFound {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        ApiError::InvalidInput {
            message: message.into(),
        }
    }

    /// Create an internal error (for bugs/invariant violations)
    pub fn internal(message: impl Into<String>) -> Self {
        ApiError::Internal {
            message: message.into(),
        }
    }

    /// Create a memory operation error
    pub fn memory(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        ApiError::Memory {
            operation: operation.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = ApiError::config("Missing API key");
        assert!(err.to_string().contains("Configuration error"));

        let err = ApiError::not_found("User", "123");
        assert!(err.to_string().contains("User not found"));
    }

    #[test]
    fn test_error_from_sqlx() {
        // Simulate a database error
        let sql_err = sqlx::Error::RowNotFound;
        let api_err: ApiError = sql_err.into();
        assert!(matches!(api_err, ApiError::Database { .. }));
    }
}
