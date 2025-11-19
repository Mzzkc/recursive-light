// Insight Extraction Processor - Post-flow CAM integration
// Phase 3 CAM: Extracts collective insights from FlowContext after BDE execution

use super::config::DualLlmConfig;
use super::insight_extraction::{
    build_significance_prompt, create_insight_from_evaluation, SignificanceEvaluation,
};
use crate::cam::manager::CAMManager;
use crate::cam::types::{DiscoveryMethod, Hyperedge, RelationshipType};
use crate::flow_process::FlowContext;
use crate::llm_error::LlmError;
use crate::LlmProvider;
use sqlx::types::Uuid;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Processor that extracts significant insights from FlowContext and stores in CAM
///
/// Runs after the 7-stage BDE flow completes, evaluating pattern observations
/// for collective significance using LLM #1.
pub struct InsightExtractionProcessor {
    llm_provider: Arc<dyn LlmProvider + Send + Sync>,
    cam_manager: Option<Arc<CAMManager>>,
    config: DualLlmConfig,
    source_instance_id: Uuid,
}

impl InsightExtractionProcessor {
    /// Create new insight extractor
    pub fn new(
        llm_provider: Arc<dyn LlmProvider + Send + Sync>,
        cam_manager: Option<Arc<CAMManager>>,
        config: DualLlmConfig,
        source_instance_id: Uuid,
    ) -> Self {
        Self {
            llm_provider,
            cam_manager,
            config,
            source_instance_id,
        }
    }

    /// Extract insights from completed FlowContext
    ///
    /// This is the main entry point: after the 7-stage BDE flow completes,
    /// call this to evaluate patterns for collective significance.
    pub async fn extract_insights(
        &self,
        context: &FlowContext,
        source_user_id: Option<Uuid>,
    ) -> Result<Vec<Uuid>, InsightExtractionError> {
        // Skip if CAM not configured
        let cam = match &self.cam_manager {
            Some(cam) => cam,
            None => {
                debug!("CAM not configured, skipping insight extraction");
                return Ok(vec![]);
            }
        };

        // Skip if dual-LLM not enabled
        if !self.config.enabled {
            debug!("Dual-LLM not enabled, skipping insight extraction");
            return Ok(vec![]);
        }

        // Skip if no patterns observed
        if context.patterns.is_empty() {
            debug!("No patterns observed, skipping insight extraction");
            return Ok(vec![]);
        }

        info!(
            pattern_count = context.patterns.len(),
            "Evaluating patterns for collective significance"
        );

        let mut extracted_insight_ids = Vec::new();

        // Evaluate each pattern observation
        for pattern in &context.patterns {
            match self.evaluate_pattern_significance(pattern, context).await {
                Ok(Some(evaluation)) => {
                    if evaluation.is_significant {
                        // Create Insight from evaluation
                        if let Some(insight) = create_insight_from_evaluation(
                            evaluation.clone(),
                            context,
                            self.source_instance_id,
                            source_user_id,
                        ) {
                            info!(
                                insight_id = %insight.id,
                                content = %insight.content,
                                "Pattern recognized as collectively significant"
                            );

                            // Store insight in CAM
                            match cam.insert_insight(insight.clone()).await {
                                Ok(_) => {
                                    extracted_insight_ids.push(insight.id);

                                    // Discover and create associations
                                    if let Err(e) = self
                                        .create_associations(
                                            &insight,
                                            &evaluation.suggested_associations,
                                            cam,
                                        )
                                        .await
                                    {
                                        warn!(
                                            insight_id = %insight.id,
                                            error = ?e,
                                            "Failed to create associations for insight"
                                        );
                                    }
                                }
                                Err(e) => {
                                    warn!(
                                        insight_id = %insight.id,
                                        error = ?e,
                                        "Failed to store insight in CAM"
                                    );
                                }
                            }
                        }
                    } else {
                        debug!(
                            pattern = %pattern.description,
                            rationale = %evaluation.rationale,
                            "Pattern not collectively significant"
                        );
                    }
                }
                Ok(None) => {
                    debug!(
                        pattern = %pattern.description,
                        "Pattern evaluation returned no result"
                    );
                }
                Err(e) => {
                    warn!(
                        pattern = %pattern.description,
                        error = ?e,
                        "Failed to evaluate pattern significance"
                    );
                }
            }
        }

        info!(
            extracted_count = extracted_insight_ids.len(),
            total_patterns = context.patterns.len(),
            "Insight extraction complete"
        );

        Ok(extracted_insight_ids)
    }

    /// Evaluate a single pattern observation for collective significance
    async fn evaluate_pattern_significance(
        &self,
        pattern: &crate::flow_process::PatternObservation,
        context: &FlowContext,
    ) -> Result<Option<SignificanceEvaluation>, InsightExtractionError> {
        // Build significance evaluation prompt
        let prompt = build_significance_prompt(pattern, context);

        // Call LLM #1
        let response = self
            .call_llm1(&prompt)
            .await
            .map_err(InsightExtractionError::LlmCallFailed)?;

        // Parse JSON response
        let evaluation: SignificanceEvaluation = serde_json::from_str(&response)
            .map_err(|e| InsightExtractionError::JsonParseFailed(response.clone(), e))?;

        Ok(Some(evaluation))
    }

    /// Call LLM #1 for significance evaluation
    async fn call_llm1(&self, prompt: &str) -> Result<String, LlmError> {
        // Note: LlmProvider doesn't support temperature parameter yet
        // Using default temperature from provider configuration
        self.llm_provider.send_request(prompt).await
    }

    /// Create hyperedge associations for a new insight
    async fn create_associations(
        &self,
        insight: &crate::cam::types::Insight,
        suggested_types: &Option<Vec<String>>,
        cam: &CAMManager,
    ) -> Result<(), InsightExtractionError> {
        let default_types = vec![
            "boundary_resonance".to_string(),
            "semantic_similarity".to_string(),
        ];
        let association_types = suggested_types.as_ref().unwrap_or(&default_types);

        for assoc_type in association_types {
            match assoc_type.as_str() {
                "semantic_similarity" => {
                    // Find semantically similar insights
                    let similar = cam
                        .semantic_search(&insight.content, 5, 0.7)
                        .await
                        .map_err(|e| {
                            InsightExtractionError::AssociationFailed(format!("{:?}", e))
                        })?;

                    for (related_insight, score) in similar {
                        // Don't link to self
                        if related_insight.id == insight.id {
                            continue;
                        }

                        // Create hyperedge using the correct structure
                        let mut hyperedge = Hyperedge::new(
                            vec![insight.id, related_insight.id],
                            RelationshipType::Reinforcement, // Semantically similar insights reinforce each other
                            score as f64,
                            self.source_instance_id,
                            DiscoveryMethod::SemanticClustering,
                        )
                        .map_err(|e| {
                            InsightExtractionError::AssociationFailed(format!("{:?}", e))
                        })?;

                        // Add metadata about how this was discovered
                        hyperedge.metadata = serde_json::json!({
                            "similarity_score": score,
                            "discovery_context": "post_insertion_semantic_search",
                            "search_query": insight.content,
                        });

                        cam.insert_hyperedge(&hyperedge).await.map_err(|e| {
                            InsightExtractionError::AssociationFailed(format!("{:?}", e))
                        })?;

                        debug!(
                            insight_1 = %insight.id,
                            insight_2 = %related_insight.id,
                            strength = score,
                            "Created semantic similarity hyperedge"
                        );
                    }
                }
                "boundary_resonance" => {
                    // Future: Find insights from similar boundary states
                    // Would require querying CAM for insights with similar oscillation_context
                    debug!("Boundary resonance association not yet implemented");
                }
                "domain_cluster" => {
                    // Future: Link insights within same primary domain
                    debug!("Domain cluster association not yet implemented");
                }
                _ => {
                    debug!(assoc_type = assoc_type, "Unknown association type");
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InsightExtractionError {
    #[error("LLM call failed: {0}")]
    LlmCallFailed(#[from] LlmError),

    #[error("Failed to parse JSON response: {1}. Response was: {0}")]
    JsonParseFailed(String, #[source] serde_json::Error),

    #[error("Failed to create association: {0}")]
    AssociationFailed(String),

    #[error("CAM operation failed: {0}")]
    CamError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests require mock LLM and CAM infrastructure
    // Integration tests will cover full flow

    #[test]
    fn test_insight_processor_creation() {
        use crate::mock_llm::MockLlm;

        let mock_llm = Arc::new(MockLlm::new(vec![]));
        let config = DualLlmConfig {
            enabled: true,
            fallback_enabled: true,
            unconscious_model: "openai/gpt-3.5-turbo".to_string(),
            llm1_timeout_ms: 30000,
            llm1_max_retries: 3,
        };

        let processor = InsightExtractionProcessor::new(
            mock_llm,
            None, // No CAM
            config,
            Uuid::new_v4(),
        );

        // Should create successfully
        assert!(processor.cam_manager.is_none());
    }
}
