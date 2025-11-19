// CAM Embeddings - Vector generation for semantic search
// OpenAI ada-002 embeddings (1536 dimensions)

use crate::cam::types::CAMError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Embedding generator trait for extensibility
#[async_trait::async_trait]
pub trait EmbeddingGenerator: Send + Sync {
    /// Generate embedding for a single text
    async fn generate(&self, text: &str) -> Result<Vec<f32>, CAMError>;

    /// Batch generate embeddings for multiple texts (up to 100)
    async fn batch_generate(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, CAMError>;
}

/// OpenAI embedding generator using ada-002 model
/// Industry standard: 1536 dimensions, proven at scale
pub struct OpenAIEmbeddingGenerator {
    api_key: String,
    client: Client,
    model: String,
}

impl OpenAIEmbeddingGenerator {
    /// Create new OpenAI embedding generator
    pub fn new(api_key: String) -> Result<Self, CAMError> {
        if api_key.is_empty() {
            return Err(CAMError::EmbeddingError(
                "OpenAI API key is required (set OPENAI_API_KEY env var)".to_string(),
            ));
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| CAMError::EmbeddingError(format!("HTTP client creation failed: {}", e)))?;

        Ok(Self {
            api_key,
            client,
            model: "text-embedding-ada-002".to_string(),
        })
    }

    /// Call OpenAI embeddings API
    async fn call_api(&self, input: Vec<String>) -> Result<Vec<Vec<f32>>, CAMError> {
        let request = EmbeddingRequest {
            model: self.model.clone(),
            input,
            encoding_format: Some("float".to_string()),
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| CAMError::EmbeddingError(format!("API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(CAMError::EmbeddingError(format!(
                "OpenAI API error ({}): {}",
                status, error_text
            )));
        }

        let api_response: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| CAMError::EmbeddingError(format!("Failed to parse response: {}", e)))?;

        let embeddings = api_response
            .data
            .into_iter()
            .map(|item| item.embedding)
            .collect();

        Ok(embeddings)
    }
}

#[async_trait::async_trait]
impl EmbeddingGenerator for OpenAIEmbeddingGenerator {
    async fn generate(&self, text: &str) -> Result<Vec<f32>, CAMError> {
        let embeddings = self.call_api(vec![text.to_string()]).await?;
        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| CAMError::EmbeddingError("No embedding returned".to_string()))
    }

    async fn batch_generate(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, CAMError> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        if texts.len() > 100 {
            return Err(CAMError::ValidationError(
                "Batch size limited to 100 texts per request".to_string(),
            ));
        }

        self.call_api(texts.to_vec()).await
    }
}

// ============================================================
// OpenAI API Types
// ============================================================

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests require OPENAI_API_KEY environment variable
    // These are integration tests, not unit tests (real API calls)

    #[tokio::test]
    #[ignore] // Requires API key and incurs costs
    async fn test_openai_single_embedding() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let generator = OpenAIEmbeddingGenerator::new(api_key).unwrap();

        let embedding = generator.generate("test insight").await.unwrap();

        assert_eq!(embedding.len(), 1536); // ada-002 dimensions
    }

    #[tokio::test]
    #[ignore] // Requires API key and incurs costs
    async fn test_openai_batch_embeddings() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let generator = OpenAIEmbeddingGenerator::new(api_key).unwrap();

        let texts = vec![
            "First insight".to_string(),
            "Second insight".to_string(),
            "Third insight".to_string(),
        ];
        let embeddings = generator.batch_generate(&texts).await.unwrap();

        assert_eq!(embeddings.len(), 3);
        assert!(embeddings.iter().all(|e| e.len() == 1536));
    }

    #[test]
    fn test_empty_api_key_rejected() {
        let result = OpenAIEmbeddingGenerator::new("".to_string());
        assert!(result.is_err());
    }
}
