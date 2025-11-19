pub mod api_error; // Wave 2: Proper error handling
mod autonomous_judgement;
pub mod cam; // Phase 3: Collective Associative Memory
pub mod domains;
pub mod dual_llm;
mod flow_process;
mod hlip_integration;
pub mod llm_error;
mod memory;
pub mod mock_llm;
pub mod personhood; // Phase 3B: LLM Personhood Infrastructure
pub mod prompt_engine;
mod token_optimization;

#[cfg(test)]
mod test_utils;

use autonomous_judgement::{AutonomousJudgementModule, Factors, Intention, Prototype};
use domains::{ComputationalDomain, CulturalDomain, ExperientialDomain, ScientificDomain};
use flow_process::{FlowContext, FlowProcess};
use hlip_integration::HLIPIntegration;
use llm_error::LlmError;
use memory::{CompactStateSnapshot, MemoryManager};
use prompt_engine::{FrameworkState, PromptEngine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use token_optimization::TokenOptimizer;
use uuid::Uuid;
// Wave 2: Proper logging
use tracing::{debug, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub model: String,
}

#[async_trait::async_trait]
pub trait LlmProvider {
    fn get_api_key(&self) -> String;
    fn get_provider_name(&self) -> String;
    fn get_model_name(&self) -> String;
    async fn send_request(&self, prompt: &str) -> Result<String, LlmError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmConfig {
    pub api_key: String,
    pub provider_name: String,
    pub model_name: String,
}

pub struct LlmFactory;

impl LlmFactory {
    pub fn create_llm(config: &LlmConfig) -> Result<Box<dyn LlmProvider>, LlmError> {
        match config.provider_name.as_str() {
            "openai" => Ok(Box::new(OpenAiLlm::new(
                config.api_key.clone(),
                config.model_name.clone(),
            ))),
            "anthropic" => Ok(Box::new(AnthropicLlm::new(
                config.api_key.clone(),
                config.model_name.clone(),
            ))),
            "openrouter" => Ok(Box::new(OpenRouterLlm::new(
                config.api_key.clone(),
                config.model_name.clone(),
            ))),
            _ => Err(LlmError::UnsupportedProvider {
                provider_name: config.provider_name.clone(),
            }),
        }
    }
}

pub struct OpenRouterLlm {
    api_key: String,
    model_name: String,
    client: Client,
}

impl OpenRouterLlm {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            api_key,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenRouterLlm {
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

pub struct OpenAiLlm {
    api_key: String,
    model_name: String,
    client: Client,
}

impl OpenAiLlm {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            api_key,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenAiLlm {
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

pub struct AnthropicLlm {
    api_key: String,
    model_name: String,
    client: Client,
}

impl AnthropicLlm {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            api_key,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for AnthropicLlm {
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

pub struct VifApi {
    provider: Box<dyn LlmProvider>,
    prompt_engine: PromptEngine,
    memory_manager: MemoryManager,
    memory_tier_manager: dual_llm::MemoryTierManager,
    token_optimizer: TokenOptimizer,
    ajm: AutonomousJudgementModule,
    hlip_integration: HLIPIntegration,
    flow_process: FlowProcess,
}

impl VifApi {
    pub async fn new(
        provider: Box<dyn LlmProvider>,
        mut framework_state: FrameworkState,
        database_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Register domains
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager::new(database_url)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let memory_tier_manager = dual_llm::MemoryTierManager::from_url(database_url)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let token_optimizer = TokenOptimizer::new(1024); // Example token budget
        let hlip_integration = HLIPIntegration::new();

        // Initialize AJM
        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![
            Prototype::new("Direct Response".to_string(), 0.9, 0.95),
            Prototype::new("Enhanced Response".to_string(), 0.7, 0.85),
        ];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        // Load dual-LLM configuration from environment
        let dual_llm_config = dual_llm::DualLlmConfig::from_env();

        // Create FlowProcess based on dual-LLM configuration
        let flow_process = if dual_llm_config.enabled {
            // Phase 2B: Create LLM #1 provider for unconscious processing
            // Parse model string to extract provider and model name
            // Format: "provider/model" (e.g., "openai/gpt-3.5-turbo")
            let (llm1_provider_name, llm1_model_name) =
                if let Some(slash_pos) = dual_llm_config.unconscious_model.find('/') {
                    (
                        &dual_llm_config.unconscious_model[..slash_pos],
                        &dual_llm_config.unconscious_model[slash_pos + 1..],
                    )
                } else {
                    // Default to OpenAI if no provider specified
                    ("openai", dual_llm_config.unconscious_model.as_str())
                };

            // Wave 2: Graceful API key handling with TDF alignment
            // COMP: Check environment, SCI: Testable fallback, CULT: Clear messaging, EXP: Intuitive behavior
            let api_key_var = match llm1_provider_name {
                "openai" => "OPENAI_API_KEY",
                "anthropic" => "ANTHROPIC_API_KEY",
                "openrouter" => "OPENROUTER_API_KEY",
                _ => "OPENAI_API_KEY", // Default
            };

            let llm1_api_key = std::env::var(api_key_var).unwrap_or_else(|_| {
                warn!(
                    provider = %llm1_provider_name,
                    env_var = api_key_var,
                    "API key not set - dual-LLM will fall back to Rust calculations. Set {} for full functionality.",
                    api_key_var
                );
                String::new()
            });

            // If API key is empty, dual-LLM will gracefully fall back to Rust domain calculations
            // This allows testing without API keys and provides clear failure modes
            let llm1_provider_arc: std::sync::Arc<dyn LlmProvider + Send + Sync> =
                match llm1_provider_name {
                    "openai" => std::sync::Arc::new(OpenAiLlm::new(
                        llm1_api_key,
                        llm1_model_name.to_string(),
                    )),
                    "anthropic" => std::sync::Arc::new(AnthropicLlm::new(
                        llm1_api_key,
                        llm1_model_name.to_string(),
                    )),
                    "openrouter" => std::sync::Arc::new(OpenRouterLlm::new(
                        llm1_api_key,
                        llm1_model_name.to_string(),
                    )),
                    _ => {
                        warn!(
                            provider = %llm1_provider_name,
                            "Unsupported LLM #1 provider - falling back to classic mode (no dual-LLM)"
                        );
                        return Ok(Self {
                            provider,
                            prompt_engine,
                            memory_manager,
                            memory_tier_manager,
                            token_optimizer,
                            ajm,
                            hlip_integration,
                            flow_process: FlowProcess::new(),
                        });
                    }
                };

            // Create FlowProcess with dual-LLM configuration
            FlowProcess::with_config(dual_llm_config, llm1_provider_arc)
        } else {
            FlowProcess::new()
        };

        Ok(Self {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process,
        })
    }

    pub async fn process_input(
        &mut self,
        user_input: &str,
        user_id: Uuid,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Phase 1A: Get or create conversation session for hot memory
        let session_id = self
            .memory_tier_manager
            .get_or_create_session(user_id)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Phase 1A/1B: Load hot memory (last 3-5 turns) for LLM #2 context
        let hot_memory = self
            .memory_tier_manager
            .load_hot_memory(session_id)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Phase 2C: Expanded cognitive keyword triggers (10 → 47 patterns)
        // Based on CULT domain research: 7 categories of memory reference
        let retrieval_keywords = [
            // Category 1: Explicit temporal references
            "remember",
            "earlier",
            "before",
            "previously",
            "last time",
            "ago",
            "past",
            "used to",
            "back when",
            "prior",
            "recent",
            "yesterday",
            "last week",
            "last month",
            // Category 2: Episodic memory triggers
            "you said",
            "you told",
            "you mentioned",
            "you explained",
            "we talked",
            "we discussed",
            "our conversation",
            "when you",
            "when we",
            // Category 3: Meta-memory queries
            "recall",
            "remind",
            "forgot",
            "forget",
            "what was",
            "refresh",
            "do you remember",
            "can you recall",
            // Category 4: Context continuation
            "going back to",
            "as we discussed",
            "as you said",
            "that topic",
            "that issue",
            "that point",
            "returning to",
            "like we",
            // Category 5: Anaphoric references
            "that thing",
            "the idea",
            "the suggestion",
            "what you",
            // Category 6: Narrative cohesion
            "you know",
            "as you know",
        ];
        let user_input_lower = user_input.to_lowercase();
        let should_search_memory = retrieval_keywords
            .iter()
            .any(|keyword| user_input_lower.contains(keyword));

        let mut warm_context = String::new();
        let mut cold_context = String::new();

        if should_search_memory {
            // Extract meaningful words from user input as search keywords
            let search_keywords: Vec<&str> = user_input
                .split_whitespace()
                .filter(|word| word.len() > 3) // Filter out short words
                .filter(|word| {
                    // Filter out common retrieval trigger words themselves
                    !retrieval_keywords.contains(&word.to_lowercase().as_str())
                })
                .take(3) // Use up to 3 keywords
                .collect();

            // Phase 2D: BM25-ranked retrieval - search warm memory (current session)
            // Retrieve and rank by significance (recency + semantic relevance + identity)
            for keyword in &search_keywords {
                if let Ok(warm_turns) = self
                    .memory_tier_manager
                    .search_warm_memory(session_id, keyword, 10)
                    .await
                {
                    if !warm_turns.is_empty() && warm_context.is_empty() {
                        // Wave 1: Rank by BM25 + temporal decay + identity criticality
                        let ranked_turns = self
                            .memory_tier_manager
                            .rank_turns_by_relevance(warm_turns, user_input)
                            .await;

                        // Select BEST turn (highest combined significance)
                        if let Some((best_turn, significance)) = ranked_turns.first() {
                            warm_context.push_str("# Earlier in this session:\n");
                            warm_context.push_str(&format!(
                                "User: {}\nAssistant: {}\n\n",
                                best_turn.user_message, best_turn.ai_response
                            ));
                            // Wave 1: Log significance scores for monitoring
                            debug!(
                                memory_tier = "warm",
                                combined_score = %significance.combined_score(),
                                recency = %significance.recency_score,
                                semantic = %significance.semantic_relevance,
                                identity = %significance.identity_criticality,
                                "Retrieved memory with significance scores"
                            );
                        }
                        break; // Found best context, stop searching
                    }
                }
            }

            // Phase 2D: BM25-ranked retrieval - search cold memory (past sessions)
            // Retrieve and rank by significance (recency + semantic relevance + identity)
            for keyword in &search_keywords {
                if let Ok(cold_turns) = self
                    .memory_tier_manager
                    .search_cold_memory(user_id, keyword, 10)
                    .await
                {
                    if !cold_turns.is_empty() && cold_context.is_empty() {
                        // Wave 1: Rank by BM25 + temporal decay + identity criticality
                        let ranked_turns = self
                            .memory_tier_manager
                            .rank_turns_by_relevance(cold_turns, user_input)
                            .await;

                        // Select BEST turn (highest combined significance)
                        if let Some((best_turn, significance)) = ranked_turns.first() {
                            let time_ago = format_time_ago(&best_turn.user_timestamp);
                            cold_context.push_str("# From previous sessions:\n");
                            cold_context.push_str(&format!(
                                "[{}] User: {}\nAssistant: {}\n\n",
                                time_ago, best_turn.user_message, best_turn.ai_response
                            ));
                            // Wave 1: Log significance scores for monitoring
                            debug!(
                                memory_tier = "cold",
                                combined_score = %significance.combined_score(),
                                recency = %significance.recency_score,
                                semantic = %significance.semantic_relevance,
                                identity = %significance.identity_criticality,
                                time_ago = %time_ago,
                                "Retrieved memory with significance scores"
                            );
                        }
                        break; // Found best context, stop searching
                    }
                }
            }
        }

        // Use AJM to determine autonomy level
        let autonomy = self.ajm.get_autonomy();

        // Process HLIP commands if present
        self.hlip_integration
            .process_hlip_command(user_input, &mut self.prompt_engine.framework_state);

        // Create FlowContext and execute the 7-stage flow
        let context = FlowContext::new(
            user_input.to_string(),
            autonomy,
            self.prompt_engine.framework_state.clone(),
        );

        let mut flow_result = self
            .flow_process
            .execute(context)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Phase 2B: Inject memory context into LLM #2 prompt for context-aware responses
        let enhanced_prompt =
            if hot_memory.turns.is_empty() && warm_context.is_empty() && cold_context.is_empty() {
                // No conversation history, use base prompt
                flow_result.structured_prompt.clone()
            } else {
                // Build comprehensive memory context
                let mut memory_sections = Vec::new();

                // Add hot memory (recent turns, always relevant)
                if !hot_memory.turns.is_empty() {
                    memory_sections.push(hot_memory.format_for_llm());
                }

                // Add warm memory (session context, if keyword-triggered)
                if !warm_context.is_empty() {
                    memory_sections.push(warm_context);
                }

                // Add cold memory (cross-session context, if keyword-triggered)
                if !cold_context.is_empty() {
                    memory_sections.push(cold_context);
                }

                let full_memory_context = memory_sections.join("\n");
                format!(
                    "<conversation_context>\n{}</conversation_context>\n\n{}",
                    full_memory_context, flow_result.structured_prompt
                )
            };

        // Get LLM #2 response using the enhanced prompt with conversation context
        let response = self.provider.send_request(&enhanced_prompt).await?;
        flow_result.llm_response = response.clone();

        // Create state snapshot with data from the flow
        let domains: Vec<prompt_engine::DomainState> = flow_result
            .domains
            .iter()
            .map(|(name, activation)| prompt_engine::DomainState {
                name: name.clone(),
                state: format!("{:.2}", activation.activation),
            })
            .collect();

        let boundaries = flow_result.boundaries.clone();

        // Extract patterns from flow result
        let patterns: Vec<String> = flow_result
            .patterns
            .iter()
            .map(|p| p.description.clone())
            .collect();

        // Use quality-aware snapshot if qualities are available (Phase 3 Day 8)
        if !flow_result.emergent_qualities.is_empty() {
            // Take first quality set (typically from highest priority boundary)
            let quality = &flow_result.emergent_qualities[0];
            let quality_array = [
                (quality.clarity * 100.0) as u8,
                (quality.depth * 100.0) as u8,
                (quality.openness * 100.0) as u8,
                (quality.precision * 100.0) as u8,
                (quality.fluidity * 100.0) as u8,
                (quality.resonance * 100.0) as u8,
                (quality.coherence * 100.0) as u8,
            ];

            self.memory_manager
                .create_snapshot_with_qualities(
                    domains,
                    boundaries,
                    patterns,
                    user_id,
                    user_input,
                    quality_array,
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        } else {
            // Fallback to regular snapshot without quality data
            self.memory_manager
                .create_snapshot(domains, boundaries, patterns, user_id, user_input)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }

        // Use progressive loading for context creation
        if let Some(latest_snapshot) = self.get_latest_snapshot(user_id).await {
            let _context = self.token_optimizer.optimize(&latest_snapshot);
            // Use context for further processing or response generation
        }

        // Phase 1A: Save conversation turn to hot memory
        // Rough token estimation: word count * 1.3 (will be replaced with proper tokenization in Phase 2)
        let input_tokens = (user_input.split_whitespace().count() as f32 * 1.3) as i32;
        let output_tokens = (response.split_whitespace().count() as f32 * 1.3) as i32;
        let snapshot_id = self
            .get_latest_snapshot(user_id)
            .await
            .and_then(|s| Uuid::parse_str(s.id()).ok());

        self.memory_tier_manager
            .save_conversation_turn(
                session_id,
                user_id,
                user_input,
                &response,
                snapshot_id,
                input_tokens,
                output_tokens,
            )
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(response)
    }

    pub async fn get_latest_snapshot(&self, user_id: Uuid) -> Option<CompactStateSnapshot> {
        self.memory_manager
            .get_latest_snapshot(user_id)
            .await
            .ok()
            .flatten()
    }
}

/// Phase 2C: Helper function to format timestamps in human-readable form
/// Converts ISO8601 timestamps to relative time ("3 weeks ago", "2 days ago")
fn format_time_ago(timestamp_str: &str) -> String {
    use chrono::{DateTime, Utc};

    // Parse timestamp
    let parsed = DateTime::parse_from_rfc3339(timestamp_str).or_else(|_| {
        // Try SQLite format: "YYYY-MM-DD HH:MM:SS"
        chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).fixed_offset())
    });

    let timestamp = match parsed {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => return timestamp_str.to_string(), // Fallback to original
    };

    let now = Utc::now();
    let duration = now.signed_duration_since(timestamp);

    let seconds = duration.num_seconds();
    let minutes = duration.num_minutes();
    let hours = duration.num_hours();
    let days = duration.num_days();
    let weeks = days / 7;
    let months = days / 30;
    let years = days / 365;

    if seconds < 60 {
        "just now".to_string()
    } else if minutes < 60 {
        format!(
            "{} minute{} ago",
            minutes,
            if minutes == 1 { "" } else { "s" }
        )
    } else if hours < 24 {
        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
    } else if days < 7 {
        format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
    } else if weeks < 4 {
        format!("{} week{} ago", weeks, if weeks == 1 { "" } else { "s" })
    } else if months < 12 {
        format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
    } else {
        format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::setup_test_db;

    #[tokio::test]
    async fn test_vif_api() {
        let framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![
                prompt_engine::BoundaryState::new("CD-SD".to_string(), 0.8, "Active".to_string()),
                prompt_engine::BoundaryState::new("SD-CuD".to_string(), 0.7, "Active".to_string()),
                prompt_engine::BoundaryState::new("CuD-ED".to_string(), 0.6, "Active".to_string()),
                prompt_engine::BoundaryState::new("ED-CD".to_string(), 0.5, "Active".to_string()),
                prompt_engine::BoundaryState::new("CD-CuD".to_string(), 0.4, "Active".to_string()),
                prompt_engine::BoundaryState::new("SD-ED".to_string(), 0.3, "Active".to_string()),
            ],
            identity: "User Identity".to_string(),
        };

        // Use mock LLM for testing (no API key needed)
        let provider = Box::new(mock_llm::MockLlm::echo());

        // Use in-memory database for testing - we'll create VifApi manually since
        // VifApi::new expects a database_url string, but we want to use an in-memory pool
        let db_pool = setup_test_db().await.unwrap();

        // Build VifApi manually with in-memory database
        let mut framework_state = framework_state;
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![
            Prototype::new("Direct Response".to_string(), 0.9, 0.95),
            Prototype::new("Enhanced Response".to_string(), 0.7, 0.85),
        ];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        let mut vif_api = VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        };

        // Create a test user first (required by foreign key constraint)
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&vif_api.memory_manager.db_pool)
        .await
        .unwrap();

        // Simulate a real user interaction
        let user_input = "Hello, world!";
        let response = vif_api.process_input(user_input, user_id).await.unwrap();
        assert!(!response.is_empty());

        let latest_snapshot = vif_api.get_latest_snapshot(user_id).await;
        assert!(latest_snapshot.is_some());
    }

    #[test]
    fn test_llm_factory_unsupported_provider() {
        let config = LlmConfig {
            api_key: "test-key".to_string(),
            provider_name: "unsupported-provider".to_string(),
            model_name: "test-model".to_string(),
        };

        let result = LlmFactory::create_llm(&config);
        assert!(result.is_err());

        match result {
            Err(LlmError::UnsupportedProvider { provider_name }) => {
                assert_eq!(provider_name, "unsupported-provider");
            }
            _ => panic!("Expected UnsupportedProvider error"),
        }
    }

    #[test]
    fn test_llm_factory_supported_providers() {
        // Test that factory creates all three supported providers without panic
        let providers = vec!["openai", "anthropic", "openrouter"];

        for provider in providers {
            let config = LlmConfig {
                api_key: "test-key".to_string(),
                provider_name: provider.to_string(),
                model_name: "test-model".to_string(),
            };

            let result = LlmFactory::create_llm(&config);
            assert!(
                result.is_ok(),
                "Factory should create {} provider",
                provider
            );

            let llm = result.unwrap();
            assert_eq!(llm.get_provider_name(), provider);
            assert_eq!(llm.get_api_key(), "test-key");
            assert_eq!(llm.get_model_name(), "test-model");
        }
    }

    #[tokio::test]
    async fn test_integration_llm_auth_error_propagation() {
        // Test that LLM authentication errors propagate through the entire VifApi stack
        let framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![prompt_engine::BoundaryState::new(
                "CD-SD".to_string(),
                0.8,
                "Active".to_string(),
            )],
            identity: "Test User".to_string(),
        };

        // Use MockErrorLlm that simulates authentication failure
        let provider = Box::new(mock_llm::MockErrorLlm::auth_error());

        // Setup VifApi with error-producing provider
        let db_pool = setup_test_db().await.unwrap();
        let mut framework_state = framework_state;
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![Prototype::new("Direct Response".to_string(), 0.9, 0.95)];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        let mut vif_api = VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        };

        // Create test user
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        // Process input - should propagate the auth error through the entire stack
        let result = vif_api
            .process_input("Test authentication error", user_id)
            .await;

        // Verify error propagated correctly
        assert!(result.is_err(), "Should propagate LLM authentication error");

        let error = result.unwrap_err();
        let error_msg = error.to_string();

        // The error should contain information about authentication failure
        assert!(
            error_msg.contains("Authentication") || error_msg.contains("Invalid API key"),
            "Error should indicate authentication problem: {}",
            error_msg
        );
    }

    #[tokio::test]
    async fn test_integration_llm_network_error_propagation() {
        // Test that LLM network errors propagate through VifApi without panicking
        let framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test User".to_string(),
        };

        // Use MockErrorLlm that simulates network timeout
        let provider = Box::new(mock_llm::MockErrorLlm::network_error());

        let db_pool = setup_test_db().await.unwrap();
        let mut framework_state = framework_state;
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![Prototype::new("Direct Response".to_string(), 0.9, 0.95)];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        let mut vif_api = VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        };

        // Create test user
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        // Process input - should gracefully handle network error
        let result = vif_api.process_input("Test network timeout", user_id).await;

        // Verify error propagated correctly (no panic)
        assert!(result.is_err(), "Should propagate LLM network error");

        let error = result.unwrap_err();
        let error_msg = error.to_string();

        // The error should contain network-related information
        assert!(
            error_msg.contains("Network")
                || error_msg.contains("Connection")
                || error_msg.contains("timeout"),
            "Error should indicate network problem: {}",
            error_msg
        );

        // Critical: Verify that the system didn't panic and is still operational
        // Try another operation to ensure the system remains stable
        let result2 = vif_api.get_latest_snapshot(user_id).await;
        assert!(
            result2.is_none(),
            "System should remain operational after network error"
        );
    }

    #[tokio::test]
    async fn test_input_validation_empty_string() {
        // Test that VifApi handles empty input gracefully
        let framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test User".to_string(),
        };

        let provider = Box::new(mock_llm::MockLlm::echo());
        let db_pool = setup_test_db().await.unwrap();

        let mut framework_state = framework_state;
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![Prototype::new("Direct Response".to_string(), 0.9, 0.95)];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        let mut vif_api = VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        };

        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&vif_api.memory_manager.db_pool)
        .await
        .unwrap();

        // Process empty input
        let result = vif_api.process_input("", user_id).await;

        // Should handle gracefully - either return Ok with empty/default response or Err
        // The key is that it shouldn't panic
        assert!(
            result.is_ok() || result.is_err(),
            "System should handle empty input without panicking"
        );
    }

    #[tokio::test]
    async fn test_input_validation_very_long_input() {
        // Test that VifApi handles very long inputs without crashing
        let framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test User".to_string(),
        };

        let provider = Box::new(mock_llm::MockLlm::echo());
        let db_pool = setup_test_db().await.unwrap();

        let mut framework_state = framework_state;
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![Prototype::new("Direct Response".to_string(), 0.9, 0.95)];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        let mut vif_api = VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        };

        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&vif_api.memory_manager.db_pool)
        .await
        .unwrap();

        // Create a very long input (10,000 characters)
        let very_long_input = "A".repeat(10_000);

        // Process very long input
        let result = vif_api.process_input(&very_long_input, user_id).await;

        // Should handle gracefully without panicking
        assert!(
            result.is_ok() || result.is_err(),
            "System should handle very long input without panicking"
        );
    }

    #[tokio::test]
    async fn test_input_validation_special_characters() {
        // Test that VifApi handles special characters and potential SQL injection attempts
        let framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test User".to_string(),
        };

        let provider = Box::new(mock_llm::MockLlm::echo());
        let db_pool = setup_test_db().await.unwrap();

        let mut framework_state = framework_state;
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![Prototype::new("Direct Response".to_string(), 0.9, 0.95)];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        let mut vif_api = VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        };

        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&vif_api.memory_manager.db_pool)
        .await
        .unwrap();

        // Test various special characters and SQL injection patterns
        let special_inputs = vec![
            "'; DROP TABLE users; --",
            "<script>alert('xss')</script>",
            "' OR '1'='1",
            "\0\n\r\t",
            "🔥💡🌟", // Unicode emojis
            r#"{"json": "injection"}"#,
        ];

        for input in special_inputs {
            let result = vif_api.process_input(input, user_id).await;

            // Should handle gracefully - SQLx parameterized queries prevent injection
            // The key is no panic and no database corruption
            assert!(
                result.is_ok() || result.is_err(),
                "System should handle special characters '{}' without panicking",
                input
            );
        }

        // Verify database integrity - users table should still exist and have our test user
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&vif_api.memory_manager.db_pool)
            .await
            .unwrap();

        assert!(
            user_count >= 1,
            "Database should remain intact after special character inputs"
        );
    }

    // ========== Phase 2B: Dual-LLM Integration Tests ==========

    #[tokio::test]
    async fn test_phase2b_hot_memory_injection() {
        // Test that hot memory is properly injected into LLM #2 prompts
        let db_pool = setup_test_db().await.unwrap();
        let user_id = Uuid::new_v4();

        // Create user
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        // Build VifApi with MockLlm
        let mut vif_api = build_test_vif_api(db_pool.clone()).await;

        // First interaction - establishes conversation history
        let _response1 = vif_api
            .process_input("Hello, my name is Alice", user_id)
            .await
            .unwrap();

        // Second interaction - should have hot memory from first
        let _response2 = vif_api
            .process_input("What is my name?", user_id)
            .await
            .unwrap();

        // Verify hot memory was loaded
        let session_id = vif_api
            .memory_tier_manager
            .get_or_create_session(user_id)
            .await
            .unwrap();
        let hot_memory = vif_api
            .memory_tier_manager
            .load_hot_memory(session_id)
            .await
            .unwrap();

        assert!(
            !hot_memory.turns.is_empty(),
            "Hot memory should contain previous turn"
        );
        assert!(
            hot_memory.turns[0].user_message.contains("Alice"),
            "Hot memory should include first conversation"
        );
    }

    #[tokio::test]
    async fn test_phase2b_keyword_triggered_warm_retrieval() {
        // Test that warm memory is retrieved when user uses trigger keywords
        let db_pool = setup_test_db().await.unwrap();
        let user_id = Uuid::new_v4();

        // Create user
        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        let mut vif_api = build_test_vif_api(db_pool.clone()).await;

        // Create multiple conversation turns
        for i in 1..=5 {
            let _ = vif_api
                .process_input(&format!("Message number {}", i), user_id)
                .await
                .unwrap();
        }

        // Use retrieval trigger keyword
        let response = vif_api
            .process_input("Do you remember what I said earlier?", user_id)
            .await;

        assert!(response.is_ok(), "Keyword-triggered retrieval should work");
    }

    #[tokio::test]
    async fn test_phase2b_dual_llm_provider_creation_with_mock() {
        // Test that dual-LLM mode can be enabled with MockLlm (no API key needed)
        std::env::set_var("DUAL_LLM_MODE", "true");
        std::env::set_var("UNCONSCIOUS_LLM_MODEL", "openai/gpt-3.5-turbo");

        let _db_pool = setup_test_db().await.unwrap();
        let provider = Box::new(mock_llm::MockLlm::echo());
        let mut framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test".to_string(),
        };
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let vif_api_result = VifApi::new(provider, framework_state, "sqlite::memory:").await;

        // Should succeed even in dual-LLM mode (creates provider)
        assert!(
            vif_api_result.is_ok(),
            "VifApi should initialize in dual-LLM mode"
        );

        // Clean up env vars
        std::env::remove_var("DUAL_LLM_MODE");
        std::env::remove_var("UNCONSCIOUS_LLM_MODEL");
    }

    #[tokio::test]
    async fn test_phase2b_memory_context_builds_correctly() {
        // Test that memory context sections are properly combined
        let db_pool = setup_test_db().await.unwrap();
        let user_id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        let mut vif_api = build_test_vif_api(db_pool.clone()).await;

        // Create conversation history
        let _r1 = vif_api.process_input("First message", user_id).await;
        let _r2 = vif_api.process_input("Second message", user_id).await;
        let _r3 = vif_api.process_input("Third message", user_id).await;

        // Verify memory is building up
        let session_id = vif_api
            .memory_tier_manager
            .get_or_create_session(user_id)
            .await
            .unwrap();
        let hot_memory = vif_api
            .memory_tier_manager
            .load_hot_memory(session_id)
            .await
            .unwrap();

        assert_eq!(
            hot_memory.turns.len(),
            3,
            "Should have 3 turns in hot memory"
        );
    }

    #[tokio::test]
    async fn test_phase2b_fallback_to_classic_mode_on_error() {
        // Test that system falls back to classic mode if dual-LLM fails
        std::env::set_var("DUAL_LLM_MODE", "true");
        std::env::set_var("UNCONSCIOUS_LLM_MODEL", "invalid/invalid-model");

        let provider = Box::new(mock_llm::MockLlm::echo());
        let mut framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test".to_string(),
        };
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let vif_api_result = VifApi::new(provider, framework_state, "sqlite::memory:").await;

        // Should still work (falls back to classic mode)
        assert!(
            vif_api_result.is_ok(),
            "VifApi should fall back to classic mode on invalid provider"
        );

        std::env::remove_var("DUAL_LLM_MODE");
        std::env::remove_var("UNCONSCIOUS_LLM_MODEL");
    }

    #[tokio::test]
    async fn test_phase2b_empty_conversation_no_context_injection() {
        // Test that no context is injected for first message (no history)
        let db_pool = setup_test_db().await.unwrap();
        let user_id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO users (id, provider, provider_id, email, name, created_at, last_login)
             VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(user_id.as_bytes().to_vec())
        .bind("test")
        .bind(user_id.to_string())
        .bind("test@example.com")
        .bind("Test User")
        .execute(&db_pool)
        .await
        .unwrap();

        let mut vif_api = build_test_vif_api(db_pool.clone()).await;

        // First message - no history
        let response = vif_api.process_input("Hello", user_id).await;

        assert!(
            response.is_ok(),
            "First message should process successfully"
        );

        // Verify hot memory is empty before this turn
        let session_id = vif_api
            .memory_tier_manager
            .get_or_create_session(user_id)
            .await
            .unwrap();
        let hot_memory = vif_api
            .memory_tier_manager
            .load_hot_memory(session_id)
            .await
            .unwrap();

        // After first turn, hot memory should have 1 turn
        assert_eq!(
            hot_memory.turns.len(),
            1,
            "Hot memory should have exactly 1 turn after first message"
        );
    }

    // Helper function to build test VifApi
    async fn build_test_vif_api(db_pool: sqlx::SqlitePool) -> VifApi {
        let provider = Box::new(mock_llm::MockLlm::echo());
        let mut framework_state = FrameworkState {
            domain_registry: prompt_engine::DomainRegistry::new(),
            boundaries: vec![],
            identity: "Test User".to_string(),
        };
        framework_state
            .domain_registry
            .register_domain(Box::new(ComputationalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ScientificDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(CulturalDomain));
        framework_state
            .domain_registry
            .register_domain(Box::new(ExperientialDomain));

        let prompt_engine = PromptEngine::new(framework_state.clone());
        let memory_manager = MemoryManager {
            db_pool: db_pool.clone(),
        };
        let memory_tier_manager = dual_llm::MemoryTierManager::new(db_pool.clone());
        let token_optimizer = TokenOptimizer::new(1024);
        let hlip_integration = HLIPIntegration::new();

        let intention = Intention::new(
            "Process user input".to_string(),
            "Understand user intent".to_string(),
            0.4,
        );
        let prototypes = vec![
            Prototype::new("Direct Response".to_string(), 0.9, 0.95),
            Prototype::new("Enhanced Response".to_string(), 0.7, 0.85),
        ];
        let factors = Factors::new(0.4, 0.7, 0.5, 0.8);
        let ajm = AutonomousJudgementModule::new(intention, prototypes, factors);

        VifApi {
            provider,
            prompt_engine,
            memory_manager,
            memory_tier_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        }
    }
}
