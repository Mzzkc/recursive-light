// CAM Qdrant Storage - Vector embeddings for semantic search
// Phase 3: Qdrant + PostgreSQL hybrid architecture

use crate::cam::types::CAMError;
use chrono::{DateTime, Utc};
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    with_payload_selector::SelectorOptions, Condition, CreateCollection, Distance, FieldCondition,
    Filter, HnswConfigDiff, Match, PointStruct, SearchPoints, Value, VectorParams, VectorsConfig,
    WithPayloadSelector,
};
use qdrant_client::Qdrant;
use sqlx::types::Uuid;
use std::collections::HashMap;

/// Qdrant vector storage for CAM system
/// Handles 1536-dim OpenAI ada-002 embeddings
pub struct QdrantVectorStorage {
    client: Qdrant,
    collection_name: String,
}

impl QdrantVectorStorage {
    /// Create new Qdrant storage and initialize collection
    pub async fn new(url: &str, collection_name: String) -> Result<Self, CAMError> {
        let client = Qdrant::from_url(url)
            .build()
            .map_err(|e| CAMError::EmbeddingError(format!("Qdrant connection failed: {}", e)))?;

        let storage = Self {
            client,
            collection_name,
        };

        // Create collection if it doesn't exist
        storage.ensure_collection_exists().await?;

        Ok(storage)
    }

    /// Ensure collection exists with proper configuration
    async fn ensure_collection_exists(&self) -> Result<(), CAMError> {
        // Check if collection exists
        let collections =
            self.client.list_collections().await.map_err(|e| {
                CAMError::EmbeddingError(format!("Failed to list collections: {}", e))
            })?;

        let collection_exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.collection_name);

        if collection_exists {
            return Ok(());
        }

        // Create collection with optimized HNSW configuration
        let vectors_config = VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 1536, // OpenAI ada-002 dimensions
                distance: Distance::Cosine.into(),
                hnsw_config: Some(HnswConfigDiff {
                    m: Some(16),                      // Good balance of accuracy/memory
                    ef_construct: Some(100),          // Construction quality
                    full_scan_threshold: Some(10000), // Exact search for small collections
                    on_disk: Some(false),             // Keep in memory for speed
                    ..Default::default()
                }),
                quantization_config: None, // Enable later for >1M vectors
                on_disk: Some(false),
                ..Default::default()
            })),
        };

        self.client
            .create_collection(CreateCollection {
                collection_name: self.collection_name.clone(),
                vectors_config: Some(vectors_config),
                ..Default::default()
            })
            .await
            .map_err(|e| CAMError::EmbeddingError(format!("Failed to create collection: {}", e)))?;

        Ok(())
    }

    /// Insert vector embedding for an insight
    pub async fn insert_vector(
        &self,
        insight_id: Uuid,
        embedding: Vec<f32>,
        created_at: DateTime<Utc>,
        lifecycle_stage: &str,
    ) -> Result<(), CAMError> {
        // Validate embedding dimensions
        if embedding.len() != 1536 {
            return Err(CAMError::ValidationError(format!(
                "Invalid embedding dimension: expected 1536, got {}",
                embedding.len()
            )));
        }

        // Create payload with metadata for filtering
        let mut payload = HashMap::new();
        payload.insert(
            "insight_id".to_string(),
            Value::from(insight_id.to_string()),
        );
        payload.insert(
            "created_at".to_string(),
            Value::from(created_at.to_rfc3339()),
        );
        payload.insert(
            "lifecycle_stage".to_string(),
            Value::from(lifecycle_stage.to_string()),
        );

        let point = PointStruct::new(insight_id.to_string(), embedding, payload);

        use qdrant_client::qdrant::UpsertPointsBuilder;

        self.client
            .upsert_points(UpsertPointsBuilder::new(&self.collection_name, vec![point]))
            .await
            .map_err(|e| CAMError::EmbeddingError(format!("Failed to insert vector: {}", e)))?;

        Ok(())
    }

    /// Batch insert vectors for multiple insights
    pub async fn batch_insert_vectors(
        &self,
        insights: Vec<(Uuid, Vec<f32>, DateTime<Utc>, String)>,
    ) -> Result<(), CAMError> {
        let points: Vec<PointStruct> = insights
            .into_iter()
            .map(|(id, embedding, created_at, lifecycle_stage)| {
                let mut payload = HashMap::new();
                payload.insert("insight_id".to_string(), Value::from(id.to_string()));
                payload.insert(
                    "created_at".to_string(),
                    Value::from(created_at.to_rfc3339()),
                );
                payload.insert("lifecycle_stage".to_string(), Value::from(lifecycle_stage));

                PointStruct::new(id.to_string(), embedding, payload)
            })
            .collect();

        use qdrant_client::qdrant::UpsertPointsBuilder;

        self.client
            .upsert_points(UpsertPointsBuilder::new(&self.collection_name, points))
            .await
            .map_err(|e| CAMError::EmbeddingError(format!("Batch insert failed: {}", e)))?;

        Ok(())
    }

    /// Semantic search returning insight UUIDs + similarity scores
    pub async fn search_similar(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
        min_score: f32,
    ) -> Result<Vec<(Uuid, f32)>, CAMError> {
        // Validate query embedding
        if query_embedding.len() != 1536 {
            return Err(CAMError::ValidationError(format!(
                "Invalid query embedding dimension: expected 1536, got {}",
                query_embedding.len()
            )));
        }

        // Filter out deprecated insights
        let filter = Filter {
            must_not: vec![Condition {
                condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                    FieldCondition {
                        key: "lifecycle_stage".to_string(),
                        r#match: Some(Match {
                            match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keyword(
                                "Deprecated".to_string(),
                            )),
                        }),
                        ..Default::default()
                    },
                )),
            }],
            ..Default::default()
        };

        let search_result = self
            .client
            .search_points(SearchPoints {
                collection_name: self.collection_name.clone(),
                vector: query_embedding,
                limit: limit as u64,
                score_threshold: Some(min_score),
                with_payload: Some(WithPayloadSelector {
                    selector_options: Some(SelectorOptions::Enable(true)),
                }),
                filter: Some(filter),
                ..Default::default()
            })
            .await
            .map_err(|e| CAMError::QueryError(format!("Semantic search failed: {}", e)))?;

        let results = search_result
            .result
            .iter()
            .filter_map(|scored_point| {
                let insight_id_str = scored_point.payload.get("insight_id")?.as_str()?;
                let insight_id = Uuid::parse_str(insight_id_str).ok()?;
                Some((insight_id, scored_point.score))
            })
            .collect();

        Ok(results)
    }

    /// Delete vector for an insight (e.g., when deprecated)
    pub async fn delete_vector(&self, insight_id: Uuid) -> Result<(), CAMError> {
        use qdrant_client::qdrant::{DeletePointsBuilder, PointsIdsList};

        let points_list = PointsIdsList {
            ids: vec![insight_id.to_string().into()],
        };

        self.client
            .delete_points(DeletePointsBuilder::new(&self.collection_name).points(points_list))
            .await
            .map_err(|e| CAMError::EmbeddingError(format!("Failed to delete vector: {}", e)))?;

        Ok(())
    }

    /// Get collection statistics
    pub async fn get_stats(&self) -> Result<CollectionStats, CAMError> {
        let info = self
            .client
            .collection_info(&self.collection_name)
            .await
            .map_err(|e| CAMError::QueryError(format!("Failed to get stats: {}", e)))?;

        let (vectors_count, indexed_vectors_count) = if let Some(result) = info.result {
            (
                result.vectors_count.unwrap_or(0),
                result.indexed_vectors_count.unwrap_or(0),
            )
        } else {
            (0, 0)
        };

        Ok(CollectionStats {
            vectors_count,
            indexed_vectors_count,
        })
    }
}

/// Collection statistics
#[derive(Debug, Clone)]
pub struct CollectionStats {
    pub vectors_count: u64,
    pub indexed_vectors_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests require Qdrant running on localhost:6334
    // Run: docker run -p 6334:6334 qdrant/qdrant

    #[tokio::test]
    #[ignore] // Requires Qdrant instance
    async fn test_qdrant_connection() {
        let storage = QdrantVectorStorage::new("http://localhost:6334", "test_cam".to_string())
            .await
            .unwrap();

        let stats = storage.get_stats().await.unwrap();
        assert_eq!(stats.vectors_count, 0); // New collection
    }

    #[tokio::test]
    #[ignore] // Requires Qdrant instance
    async fn test_insert_and_search() {
        let storage = QdrantVectorStorage::new("http://localhost:6334", "test_cam".to_string())
            .await
            .unwrap();

        let insight_id = Uuid::new_v4();
        let embedding = vec![0.1; 1536]; // Mock embedding

        storage
            .insert_vector(insight_id, embedding.clone(), Utc::now(), "Emerging")
            .await
            .unwrap();

        // Search with same vector should return high similarity
        let results = storage.search_similar(embedding, 10, 0.9).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, insight_id);
        assert!(results[0].1 > 0.99); // Should be nearly 1.0 (identical vectors)
    }
}
