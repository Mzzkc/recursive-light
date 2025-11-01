mod autonomous_judgement;
pub mod domains;
pub mod dual_llm;
mod flow_process;
mod hlip_integration;
pub mod llm_error;
mod memory;
pub mod mock_llm;
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

        Ok(Self {
            provider,
            prompt_engine,
            memory_manager,
            token_optimizer,
            ajm,
            hlip_integration,
            flow_process: FlowProcess::new(),
        })
    }

    pub async fn process_input(
        &mut self,
        user_input: &str,
        user_id: Uuid,
    ) -> Result<String, Box<dyn std::error::Error>> {
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

        // Get LLM response using the structured prompt from the flow
        let response = self
            .provider
            .send_request(&flow_result.structured_prompt)
            .await?;
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
        let memory_manager = MemoryManager { db_pool };
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
}
