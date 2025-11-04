// CAM Embeddings - Vector generation for semantic search
// Phase 1 MVP: Mock embeddings for development
// Phase 2+: Real embeddings (OpenAI ada-002 or local Sentence Transformers)

use crate::cam::types::CAMError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Embedding generator trait for extensibility
#[async_trait::async_trait]
pub trait EmbeddingGenerator: Send + Sync {
    /// Generate embedding for a single text
    async fn generate(&self, text: &str) -> Result<Vec<f32>, CAMError>;

    /// Batch generate embeddings for multiple texts
    async fn batch_generate(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, CAMError>;
}

/// Mock embedding generator for Phase 1 MVP
/// Generates deterministic 1536-dim embeddings based on text hash
pub struct MockEmbeddingGenerator {
    dimensions: usize,
}

impl MockEmbeddingGenerator {
    pub fn new() -> Self {
        Self {
            dimensions: 1536, // OpenAI ada-002 dimensions
        }
    }

    /// Generate a deterministic embedding from text
    /// Uses hash of text to seed a simple pattern
    fn generate_mock_embedding(&self, text: &str) -> Vec<f32> {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let seed = hasher.finish();

        let mut embedding = Vec::with_capacity(self.dimensions);

        // Generate deterministic pattern based on seed
        for i in 0..self.dimensions {
            let value = ((seed.wrapping_add(i as u64) as f64) / u64::MAX as f64) * 2.0 - 1.0;
            embedding.push(value as f32);
        }

        // Normalize to unit vector (required for cosine similarity)
        let magnitude: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for val in &mut embedding {
                *val /= magnitude;
            }
        }

        embedding
    }
}

impl Default for MockEmbeddingGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl EmbeddingGenerator for MockEmbeddingGenerator {
    async fn generate(&self, text: &str) -> Result<Vec<f32>, CAMError> {
        Ok(self.generate_mock_embedding(text))
    }

    async fn batch_generate(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, CAMError> {
        let embeddings = texts
            .iter()
            .map(|text| self.generate_mock_embedding(text))
            .collect();
        Ok(embeddings)
    }
}

// TODO Phase 2+: Implement real embedding generators
// pub struct OpenAIEmbeddingGenerator { ... }
// pub struct LocalEmbeddingGenerator { ... } // Sentence Transformers

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_embedding_dimensions() {
        let generator = MockEmbeddingGenerator::new();
        let embedding = generator.generate("test").await.unwrap();
        assert_eq!(embedding.len(), 1536);
    }

    #[tokio::test]
    async fn test_mock_embedding_deterministic() {
        let generator = MockEmbeddingGenerator::new();
        let embedding1 = generator.generate("same text").await.unwrap();
        let embedding2 = generator.generate("same text").await.unwrap();
        assert_eq!(embedding1, embedding2);
    }

    #[tokio::test]
    async fn test_mock_embedding_different_for_different_text() {
        let generator = MockEmbeddingGenerator::new();
        let embedding1 = generator.generate("text one").await.unwrap();
        let embedding2 = generator.generate("text two").await.unwrap();
        assert_ne!(embedding1, embedding2);
    }

    #[tokio::test]
    async fn test_mock_embedding_normalized() {
        let generator = MockEmbeddingGenerator::new();
        let embedding = generator.generate("normalize test").await.unwrap();

        // Check unit vector (magnitude ~= 1.0)
        let magnitude: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_batch_generate() {
        let generator = MockEmbeddingGenerator::new();
        let texts = vec!["text1".to_string(), "text2".to_string(), "text3".to_string()];
        let embeddings = generator.batch_generate(&texts).await.unwrap();

        assert_eq!(embeddings.len(), 3);
        assert_eq!(embeddings[0].len(), 1536);
        assert_eq!(embeddings[1].len(), 1536);
        assert_eq!(embeddings[2].len(), 1536);
    }
}
