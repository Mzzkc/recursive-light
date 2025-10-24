// Mock LLM Provider for Testing
// This allows testing without real API keys or costs

use crate::llm_error::LlmError;
use crate::LlmProvider;
use async_trait::async_trait;

/// Mock LLM that returns deterministic responses for testing
pub struct MockLlm {
    responses: Vec<String>,
    call_count: std::sync::Arc<std::sync::Mutex<usize>>,
}

impl MockLlm {
    /// Create a mock LLM with predetermined responses
    pub fn new(responses: Vec<String>) -> Self {
        Self {
            responses,
            call_count: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }

    /// Create a simple mock that echoes the prompt
    pub fn echo() -> Self {
        Self::new(vec![])
    }

    /// Get number of times the mock was called
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// Get the next response (cycles through responses)
    fn next_response(&self, prompt: &str) -> String {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;

        if self.responses.is_empty() {
            // Echo mode: return simplified version of prompt
            format!(
                "Mock response to: {}",
                prompt.chars().take(100).collect::<String>()
            )
        } else {
            // Use predetermined responses
            let index = (*count - 1) % self.responses.len();
            self.responses[index].clone()
        }
    }
}

/// Mock LLM that returns errors for testing error propagation
pub struct MockErrorLlm {
    error: LlmError,
}

impl MockErrorLlm {
    /// Create a mock that always returns the specified error
    pub fn new(error: LlmError) -> Self {
        Self { error }
    }

    /// Create a mock that simulates authentication errors
    pub fn auth_error() -> Self {
        Self::new(LlmError::AuthError {
            message: "Invalid API key".to_string(),
        })
    }

    /// Create a mock that simulates network errors
    pub fn network_error() -> Self {
        Self::new(LlmError::NetworkError {
            message: "Connection timeout".to_string(),
            status_code: None,
        })
    }

    /// Create a mock that simulates rate limiting
    pub fn rate_limit_error() -> Self {
        Self::new(LlmError::RateLimitError {
            message: "Rate limit exceeded".to_string(),
            retry_after: Some(60),
        })
    }
}

#[async_trait]
impl LlmProvider for MockErrorLlm {
    fn get_api_key(&self) -> String {
        "mock-error-api-key".to_string()
    }

    fn get_provider_name(&self) -> String {
        "mock-error".to_string()
    }

    fn get_model_name(&self) -> String {
        "mock-error-model".to_string()
    }

    async fn send_request(&self, _prompt: &str) -> Result<String, LlmError> {
        // Always return the configured error
        Err(self.error.clone())
    }
}

#[async_trait]
impl LlmProvider for MockLlm {
    fn get_api_key(&self) -> String {
        "mock-api-key".to_string()
    }

    fn get_provider_name(&self) -> String {
        "mock".to_string()
    }

    fn get_model_name(&self) -> String {
        "mock-model".to_string()
    }

    async fn send_request(&self, prompt: &str) -> Result<String, LlmError> {
        // Simulate slight delay (optional, for more realistic testing)
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        Ok(self.next_response(prompt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_echo() {
        let mock = MockLlm::echo();
        let response = mock.send_request("Hello, world!").await.unwrap();

        assert!(response.contains("Mock response to:"));
        assert_eq!(mock.call_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_predetermined() {
        let responses = vec!["First response".to_string(), "Second response".to_string()];
        let mock = MockLlm::new(responses);

        let r1 = mock.send_request("test1").await.unwrap();
        let r2 = mock.send_request("test2").await.unwrap();
        let r3 = mock.send_request("test3").await.unwrap(); // Cycles back

        assert_eq!(r1, "First response");
        assert_eq!(r2, "Second response");
        assert_eq!(r3, "First response"); // Cycled
        assert_eq!(mock.call_count(), 3);
    }
}
