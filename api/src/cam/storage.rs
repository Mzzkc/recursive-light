// CAM Storage - PostgreSQL + pgvector operations
// Phase 3: Hypergraph storage implementation

use crate::cam::types::{CAMError, Domain, Hyperedge, Insight, LifecycleStage};
use sqlx::types::Uuid;
use sqlx::{PgPool, Row};

/// PostgreSQL storage for CAM system
pub struct CAMStorage {
    pool: PgPool,
}

impl CAMStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert a new insight into the database
    pub async fn insert_insight(&self, insight: &Insight) -> Result<(), CAMError> {
        let embedding_vec = insight.embedding.as_ref().ok_or_else(|| {
            CAMError::ValidationError("Embedding required for storage".to_string())
        })?;

        sqlx::query(
            r#"
            INSERT INTO cam_insights (
                id, content, embedding, primary_domain, secondary_domains,
                confidence, lifecycle_stage, source_instance_id,
                source_user_id, source_flow_id, oscillation_context,
                observation_count, metadata, created_at, last_validated
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#,
        )
        .bind(insight.id)
        .bind(&insight.content)
        .bind(embedding_vec.as_slice())
        .bind(format!("{:?}", insight.primary_domain))
        .bind(
            insight
                .secondary_domains
                .iter()
                .map(|d| format!("{:?}", d))
                .collect::<Vec<_>>(),
        )
        .bind(insight.confidence)
        .bind(format!("{:?}", insight.lifecycle))
        .bind(insight.source_instance_id)
        .bind(insight.source_user_id)
        .bind(insight.source_flow_id)
        .bind(serde_json::to_value(&insight.oscillation_context)?)
        .bind(insight.observation_count as i32)
        .bind(&insight.metadata)
        .bind(insight.created_at)
        .bind(insight.last_validated)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a single insight by ID
    pub async fn get_insight(&self, id: Uuid) -> Result<Insight, CAMError> {
        let row = sqlx::query(
            r#"
            SELECT
                id, content, embedding, primary_domain, secondary_domains,
                confidence, lifecycle_stage, source_instance_id,
                source_user_id, source_flow_id, oscillation_context,
                observation_count, created_at, last_validated, metadata
            FROM cam_insights
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(CAMError::InsightNotFound(id))?;

        self.row_to_insight(row)
    }

    /// Semantic search using vector similarity (pgvector)
    pub async fn semantic_search(
        &self,
        query_embedding: &[f32],
        min_similarity: f64,
        limit: usize,
    ) -> Result<Vec<Insight>, CAMError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, content, embedding, primary_domain, secondary_domains,
                confidence, lifecycle_stage, source_instance_id,
                source_user_id, source_flow_id, oscillation_context,
                observation_count, created_at, last_validated, metadata,
                1 - (embedding <=> $1) AS similarity
            FROM cam_insights
            WHERE lifecycle_stage != 'Deprecated'
              AND (1 - (embedding <=> $1)) >= $2
            ORDER BY similarity DESC
            LIMIT $3
            "#,
        )
        .bind(query_embedding)
        .bind(min_similarity)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| self.row_to_insight(row))
            .collect()
    }

    /// Increment observation count for an insight (uses database function)
    pub async fn increment_observation(&self, insight_id: Uuid) -> Result<(), CAMError> {
        sqlx::query("SELECT cam_increment_observation($1)")
            .bind(insight_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Update insight confidence (uses database function)
    pub async fn update_confidence(
        &self,
        insight_id: Uuid,
        new_confidence: f64,
    ) -> Result<(), CAMError> {
        sqlx::query("SELECT cam_update_confidence($1, $2)")
            .bind(insight_id)
            .bind(new_confidence)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Insert a hyperedge connecting insights
    pub async fn insert_hyperedge(&self, hyperedge: &Hyperedge) -> Result<(), CAMError> {
        sqlx::query(
            r#"
            INSERT INTO cam_hyperedges (
                id, insight_ids, relationship_type, strength, spanning_domains,
                discovery_method, discovered_by, observation_count, metadata, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(hyperedge.id)
        .bind(&hyperedge.insight_ids)
        .bind(format!("{:?}", hyperedge.relationship))
        .bind(hyperedge.strength)
        .bind(
            hyperedge
                .spanning_domains
                .iter()
                .map(|d| format!("{:?}", d))
                .collect::<Vec<_>>(),
        )
        .bind(format!("{:?}", hyperedge.discovery_method))
        .bind(hyperedge.discovered_by)
        .bind(hyperedge.observation_count as i32)
        .bind(&hyperedge.metadata)
        .bind(hyperedge.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get hyperedges for a specific insight
    pub async fn get_hyperedges_for_insight(
        &self,
        insight_id: Uuid,
    ) -> Result<Vec<Hyperedge>, CAMError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, insight_ids, relationship_type, strength, spanning_domains,
                discovery_method, discovered_by, observation_count, metadata, created_at
            FROM cam_hyperedges
            WHERE $1 = ANY(insight_ids)
            ORDER BY strength DESC
            "#,
        )
        .bind(insight_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| self.row_to_hyperedge(row))
            .collect()
    }

    /// Get hyperedges for multiple insights (batch operation)
    pub async fn get_hyperedges_for_insights(
        &self,
        insight_ids: &[Uuid],
    ) -> Result<Vec<Hyperedge>, CAMError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, insight_ids, relationship_type, strength, spanning_domains,
                discovery_method, discovered_by, observation_count, metadata, created_at
            FROM cam_hyperedges
            WHERE insight_ids && $1
            ORDER BY strength DESC
            "#,
        )
        .bind(insight_ids)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| self.row_to_hyperedge(row))
            .collect()
    }

    // ============================================================
    // Helper Methods - Row Conversion
    // ============================================================

    fn row_to_insight(&self, row: sqlx::postgres::PgRow) -> Result<Insight, CAMError> {
        use crate::cam::types::OscillationContext;
        use chrono::{DateTime, Utc};

        let id: Uuid = row.try_get("id")?;
        let content: String = row.try_get("content")?;
        let embedding: Option<Vec<f32>> = row.try_get("embedding").ok();
        let primary_domain_str: String = row.try_get("primary_domain")?;
        let secondary_domains_str: Vec<String> = row.try_get("secondary_domains")?;
        let confidence: f64 = row.try_get::<f32, _>("confidence")? as f64;
        let lifecycle_str: String = row.try_get("lifecycle_stage")?;
        let source_instance_id: Uuid = row.try_get("source_instance_id")?;
        let source_user_id: Option<Uuid> = row.try_get("source_user_id")?;
        let source_flow_id: Option<Uuid> = row.try_get("source_flow_id")?;
        let oscillation_context_json: serde_json::Value = row.try_get("oscillation_context")?;
        let observation_count: i32 = row.try_get("observation_count")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let last_validated: DateTime<Utc> = row.try_get("last_validated")?;
        let metadata: serde_json::Value = row.try_get("metadata")?;

        Ok(Insight {
            id,
            content,
            embedding,
            primary_domain: self.parse_domain(&primary_domain_str)?,
            secondary_domains: secondary_domains_str
                .iter()
                .map(|s| self.parse_domain(s))
                .collect::<Result<Vec<_>, _>>()?,
            confidence,
            lifecycle: self.parse_lifecycle(&lifecycle_str)?,
            source_instance_id,
            source_user_id,
            source_flow_id,
            oscillation_context: serde_json::from_value::<OscillationContext>(
                oscillation_context_json,
            )?,
            observation_count: observation_count as u32,
            last_validated,
            created_at,
            metadata,
        })
    }

    fn row_to_hyperedge(&self, row: sqlx::postgres::PgRow) -> Result<Hyperedge, CAMError> {
        use chrono::{DateTime, Utc};

        let id: Uuid = row.try_get("id")?;
        let insight_ids: Vec<Uuid> = row.try_get("insight_ids")?;
        let relationship_str: String = row.try_get("relationship_type")?;
        let strength: f64 = row.try_get::<f32, _>("strength")? as f64;
        let spanning_domains_str: Vec<String> = row.try_get("spanning_domains")?;
        let discovery_method_str: String = row.try_get("discovery_method")?;
        let discovered_by: Uuid = row.try_get("discovered_by")?;
        let observation_count: i32 = row.try_get("observation_count")?;
        let metadata: serde_json::Value = row.try_get("metadata")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;

        Ok(Hyperedge {
            id,
            insight_ids,
            relationship: self.parse_relationship(&relationship_str)?,
            strength,
            spanning_domains: spanning_domains_str
                .iter()
                .map(|s| self.parse_domain(s))
                .collect::<Result<Vec<_>, _>>()?,
            discovery_method: self.parse_discovery_method(&discovery_method_str)?,
            discovered_by,
            created_at,
            observation_count: observation_count as u32,
            metadata,
        })
    }

    // ============================================================
    // Helper Methods - Parsing
    // ============================================================

    fn parse_domain(&self, s: &str) -> Result<Domain, CAMError> {
        match s {
            "Computational" => Ok(Domain::Computational),
            "Scientific" => Ok(Domain::Scientific),
            "Cultural" => Ok(Domain::Cultural),
            "Experiential" => Ok(Domain::Experiential),
            "ComputationalAlgorithms" => Ok(Domain::ComputationalAlgorithms),
            "ComputationalDataStructures" => Ok(Domain::ComputationalDataStructures),
            "ScientificPhysics" => Ok(Domain::ScientificPhysics),
            "ScientificBiology" => Ok(Domain::ScientificBiology),
            "CulturalLanguage" => Ok(Domain::CulturalLanguage),
            "CulturalEthics" => Ok(Domain::CulturalEthics),
            "ExperientialAesthetic" => Ok(Domain::ExperientialAesthetic),
            "ExperientialEmbodied" => Ok(Domain::ExperientialEmbodied),
            _ => Err(CAMError::ValidationError(format!("Unknown domain: {}", s))),
        }
    }

    fn parse_lifecycle(&self, s: &str) -> Result<LifecycleStage, CAMError> {
        match s {
            "Emerging" => Ok(LifecycleStage::Emerging),
            "Validated" => Ok(LifecycleStage::Validated),
            "Established" => Ok(LifecycleStage::Established),
            "Deprecated" => Ok(LifecycleStage::Deprecated),
            _ => Err(CAMError::ValidationError(format!(
                "Unknown lifecycle: {}",
                s
            ))),
        }
    }

    fn parse_relationship(&self, s: &str) -> Result<crate::cam::types::RelationshipType, CAMError> {
        use crate::cam::types::RelationshipType;

        match s {
            "Contradiction" => Ok(RelationshipType::Contradiction),
            "Reinforcement" => Ok(RelationshipType::Reinforcement),
            "Generalization" => Ok(RelationshipType::Generalization),
            "Specialization" => Ok(RelationshipType::Specialization),
            "Synthesis" => Ok(RelationshipType::Synthesis),
            "Analogy" => Ok(RelationshipType::Analogy),
            "Causation" => Ok(RelationshipType::Causation),
            "TemporalSequence" => Ok(RelationshipType::TemporalSequence),
            "CoOccurrence" => Ok(RelationshipType::CoOccurrence),
            _ => Err(CAMError::ValidationError(format!(
                "Unknown relationship: {}",
                s
            ))),
        }
    }

    fn parse_discovery_method(
        &self,
        s: &str,
    ) -> Result<crate::cam::types::DiscoveryMethod, CAMError> {
        use crate::cam::types::DiscoveryMethod;

        match s {
            "OscillationEmergence" => Ok(DiscoveryMethod::OscillationEmergence),
            "SemanticClustering" => Ok(DiscoveryMethod::SemanticClustering),
            "LLMInference" => Ok(DiscoveryMethod::LLMInference),
            "ManualCuration" => Ok(DiscoveryMethod::ManualCuration),
            "StatisticalAnalysis" => Ok(DiscoveryMethod::StatisticalAnalysis),
            _ => Err(CAMError::ValidationError(format!(
                "Unknown discovery method: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests will be added in next phase
    // For now, just ensure module compiles
}
