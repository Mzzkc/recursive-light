// CAM Manager - Coordinates Qdrant + PostgreSQL hybrid operations
// Phase 3: High-level API for Collective Associative Memory

use crate::cam::embeddings::{EmbeddingGenerator, OpenAIEmbeddingGenerator};
use crate::cam::qdrant_storage::QdrantVectorStorage;
use crate::cam::storage::CAMStorage;
use crate::cam::types::{CAMError, Hyperedge, Insight};
use chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::sync::Arc;

/// High-level coordinator for CAM system
/// Orchestrates PostgreSQL (metadata) + Qdrant (vectors) + OpenAI (embeddings)
pub struct CAMManager {
    pg_storage: CAMStorage,
    vector_storage: Arc<QdrantVectorStorage>,
    embedding_gen: Arc<dyn EmbeddingGenerator>,
}

impl CAMManager {
    /// Create new CAM manager with PostgreSQL + Qdrant + OpenAI
    pub async fn new(
        pg_pool: PgPool,
        qdrant_url: &str,
        openai_api_key: String,
    ) -> Result<Self, CAMError> {
        let pg_storage = CAMStorage::new(pg_pool);

        let vector_storage = Arc::new(
            QdrantVectorStorage::new(qdrant_url, "cam_insights_vectors".to_string()).await?,
        );

        let embedding_gen =
            Arc::new(OpenAIEmbeddingGenerator::new(openai_api_key)?) as Arc<dyn EmbeddingGenerator>;

        Ok(Self {
            pg_storage,
            vector_storage,
            embedding_gen,
        })
    }

    /// Insert new insight (generates embedding, stores in both Qdrant + PostgreSQL)
    pub async fn insert_insight(&self, mut insight: Insight) -> Result<(), CAMError> {
        // Generate embedding for insight content
        let embedding = self.embedding_gen.generate(&insight.content).await?;

        // Store vector in Qdrant
        self.vector_storage
            .insert_vector(
                insight.id,
                embedding.clone(),
                insight.created_at,
                &format!("{:?}", insight.lifecycle),
            )
            .await?;

        // Store metadata in PostgreSQL (embedding not needed)
        insight.embedding = None; // PostgreSQL doesn't store embeddings
        self.pg_storage.insert_insight(&insight).await?;

        Ok(())
    }

    /// Batch insert insights (optimized for multiple insights)
    pub async fn batch_insert_insights(&self, insights: Vec<Insight>) -> Result<(), CAMError> {
        if insights.is_empty() {
            return Ok(());
        }

        // Extract content for batch embedding generation
        let contents: Vec<String> = insights.iter().map(|i| i.content.clone()).collect();

        // Generate embeddings in batch (more efficient)
        let embeddings = self.embedding_gen.batch_generate(&contents).await?;

        // Prepare Qdrant batch data
        let qdrant_data: Vec<(Uuid, Vec<f32>, chrono::DateTime<Utc>, String)> = insights
            .iter()
            .zip(embeddings.iter())
            .map(|(insight, embedding)| {
                (
                    insight.id,
                    embedding.clone(),
                    insight.created_at,
                    format!("{:?}", insight.lifecycle),
                )
            })
            .collect();

        // Batch insert to Qdrant
        self.vector_storage
            .batch_insert_vectors(qdrant_data)
            .await?;

        // Batch insert to PostgreSQL
        for mut insight in insights {
            insight.embedding = None; // PostgreSQL doesn't store embeddings
            self.pg_storage.insert_insight(&insight).await?;
        }

        Ok(())
    }

    /// Semantic search: Query text → embedding → Qdrant search → PostgreSQL metadata
    pub async fn semantic_search(
        &self,
        query: &str,
        limit: usize,
        min_score: f32,
    ) -> Result<Vec<(Insight, f32)>, CAMError> {
        // Generate embedding for query
        let query_embedding = self.embedding_gen.generate(query).await?;

        // Search Qdrant for similar vectors
        let similar_ids = self
            .vector_storage
            .search_similar(query_embedding, limit, min_score)
            .await?;

        if similar_ids.is_empty() {
            return Ok(vec![]);
        }

        // Extract IDs and scores
        let (ids, scores): (Vec<Uuid>, Vec<f32>) = similar_ids.into_iter().unzip();

        // Fetch metadata from PostgreSQL (preserves Qdrant ranking)
        let insights = self.pg_storage.get_insights_by_ids(&ids).await?;

        // Combine insights with similarity scores
        let results = insights.into_iter().zip(scores.into_iter()).collect();

        Ok(results)
    }

    /// Get insight by ID (metadata from PostgreSQL)
    pub async fn get_insight(&self, id: Uuid) -> Result<Insight, CAMError> {
        self.pg_storage.get_insight(id).await
    }

    /// Insert hyperedge (relationship between insights)
    pub async fn insert_hyperedge(&self, hyperedge: &Hyperedge) -> Result<(), CAMError> {
        self.pg_storage.insert_hyperedge(hyperedge).await
    }

    /// Get hyperedges for an insight
    pub async fn get_hyperedges_for_insight(
        &self,
        insight_id: Uuid,
    ) -> Result<Vec<Hyperedge>, CAMError> {
        self.pg_storage.get_hyperedges_for_insight(insight_id).await
    }

    /// Increment observation count (insight observed again)
    pub async fn increment_observation(&self, insight_id: Uuid) -> Result<(), CAMError> {
        self.pg_storage.increment_observation(insight_id).await
    }

    /// Update insight confidence (after validation)
    pub async fn update_confidence(
        &self,
        insight_id: Uuid,
        new_confidence: f64,
    ) -> Result<(), CAMError> {
        self.pg_storage
            .update_confidence(insight_id, new_confidence)
            .await
    }

    /// Delete deprecated insight (removes from both Qdrant + PostgreSQL)
    pub async fn delete_insight(&self, insight_id: Uuid) -> Result<(), CAMError> {
        // Delete from Qdrant
        self.vector_storage.delete_vector(insight_id).await?;

        // PostgreSQL deletion would go here (when implemented)
        // For now, we just mark as Deprecated in lifecycle_stage

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cam::types::{Domain, OscillationContext, PhenomenologicalQualities};

    // Tests require: PostgreSQL, Qdrant, OpenAI API key

    #[tokio::test]
    #[ignore] // Requires full infrastructure
    async fn test_cam_manager_integration() {
        // This is an integration test demonstrating the full flow
        // Requires:
        // - PostgreSQL at DATABASE_URL
        // - Qdrant at http://localhost:6334
        // - OPENAI_API_KEY environment variable

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let openai_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let pool = sqlx::PgPool::connect(&database_url).await.unwrap();

        let manager = CAMManager::new(pool, "http://localhost:6334", openai_key)
            .await
            .unwrap();

        // Create test insight
        let oscillation_ctx = OscillationContext {
            boundary: "CD-ED".to_string(),
            frequency: 2.5,
            amplitude: 0.8,
            phase: 1.2,
            permeability: 0.7,
            qualities: PhenomenologicalQualities {
                clarity: 0.8,
                depth: 0.9,
                openness: 0.7,
                precision: 0.85,
                fluidity: 0.75,
                resonance: 0.9,
                coherence: 0.88,
            },
        };

        let insight = Insight::new(
            "Test insight about boundary dissolution".to_string(),
            Domain::Computational,
            vec![Domain::Experiential],
            Uuid::new_v4(), // source_instance_id
            oscillation_ctx,
        );

        // Insert insight (generates embedding, stores in Qdrant + PostgreSQL)
        manager.insert_insight(insight.clone()).await.unwrap();

        // Semantic search should find it
        let results = manager
            .semantic_search("boundary dissolution", 5, 0.5)
            .await
            .unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].0.content, insight.content);
    }
}
