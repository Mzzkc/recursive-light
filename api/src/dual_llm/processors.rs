// UnconscciousLlmProcessor Implementation
// LLM #1 processor with robust JSON parsing, validation, and fallback logic

use crate::dual_llm::config::DualLlmConfig;
use crate::dual_llm::types::{Llm1Output, ValidationError};
use crate::flow_process::{
    DomainActivation, DomainEmergenceProcessor, FlowContext, FlowError, StageProcessor,
};
use crate::llm_error::LlmError;
use crate::LlmProvider;
use std::sync::Arc;

/// LLM #1 Processor (Unconscious Recognizer)
/// Calls LLM to recognize emergent domain activations and boundary states
/// Falls back to Rust calculations on failure
pub struct UnconscciousLlmProcessor {
    llm_provider: Arc<dyn LlmProvider + Send + Sync>,
    config: DualLlmConfig,
    fallback_processor: DomainEmergenceProcessor,
}

impl UnconscciousLlmProcessor {
    pub fn new(llm_provider: Arc<dyn LlmProvider + Send + Sync>, config: DualLlmConfig) -> Self {
        Self {
            llm_provider,
            config,
            fallback_processor: DomainEmergenceProcessor,
        }
    }

    /// Build LLM #1 prompt from user input and previous state
    fn build_llm1_prompt(&self, user_input: &str, _previous_state: Option<String>) -> String {
        // LLM #1 prompt template for domain/boundary calculation
        format!(
            r#"<system_role>
You are the Unconscious Processor for the Volumetric Integration Framework (VIF).
Your role: Calculate domain activations and boundary permeabilities based on user input.
You do NOT respond to the useryou provide structured state for the Conscious LLM.
</system_role>

<task>
Analyze the user input to determine:
1. Domain activations (0.0-1.0 for each domain: CD, SD, CuD, ED)
2. Boundary permeabilities (0.0-1.0 for each boundary pair)
3. Boundary statuses ("Maintained", "Transitional", "Transcendent")
4. Relevant patterns identified in the input
</task>

<domains>
- CD (Computational Domain): Logic, algorithms, formal systems, code, mathematical patterns
- SD (Scientific Domain): Empirical evidence, theories, experiments, data, falsifiable claims
- CuD (Cultural Domain): Context, narratives, meaning, social structures, interpretation
- ED (Experiential Domain): Direct experience, qualitative knowing, phenomenology, felt sense
</domains>

<boundary_status_rules>
- Maintained (perm < 0.6): Domains remain distinct, clear separation
- Transitional (0.6 d perm d 0.8): Domains begin to integrate, oscillation active
- Transcendent (perm > 0.8): Boundaries dissolve, unified understanding emerges
</boundary_status_rules>

<user_input>
{}
</user_input>

<output_format>
Return ONLY valid JSON (no markdown, no explanation). Schema:
{{
  "reasoning": "Brief chain-of-thought explanation (1-2 sentences)",
  "domain_activations": {{
    "CD": 0.0-1.0,
    "SD": 0.0-1.0,
    "CuD": 0.0-1.0,
    "ED": 0.0-1.0
  }},
  "boundary_permeabilities": {{
    "CD-SD": 0.0-1.0,
    "CD-CuD": 0.0-1.0,
    "CD-ED": 0.0-1.0,
    "SD-CuD": 0.0-1.0,
    "SD-ED": 0.0-1.0,
    "CuD-ED": 0.0-1.0
  }},
  "boundary_statuses": {{
    "CD-SD": "Maintained|Transitional|Transcendent",
    "CD-CuD": "...",
    "CD-ED": "...",
    "SD-CuD": "...",
    "SD-ED": "...",
    "CuD-ED": "..."
  }},
  "identified_patterns": ["pattern1", "pattern2"]
}}
</output_format>

<examples>
Example 1:
User: "How do I implement quicksort in Rust?"
Output:
{{
  "reasoning": "Technical programming question with algorithmic focus",
  "domain_activations": {{"CD": 0.95, "SD": 0.3, "CuD": 0.1, "ED": 0.2}},
  "boundary_permeabilities": {{
    "CD-SD": 0.5,
    "CD-CuD": 0.3,
    "CD-ED": 0.4,
    "SD-CuD": 0.2,
    "SD-ED": 0.2,
    "CuD-ED": 0.3
  }},
  "boundary_statuses": {{
    "CD-SD": "Maintained",
    "CD-CuD": "Maintained",
    "CD-ED": "Maintained",
    "SD-CuD": "Maintained",
    "SD-ED": "Maintained",
    "CuD-ED": "Maintained"
  }},
  "identified_patterns": ["algorithm_implementation", "programming_language_specific"]
}}

Example 2:
User: "What does it feel like to understand something deeply?"
Output:
{{
  "reasoning": "Phenomenological inquiry focused on experiential understanding",
  "domain_activations": {{"CD": 0.3, "SD": 0.4, "CuD": 0.7, "ED": 0.95}},
  "boundary_permeabilities": {{
    "CD-SD": 0.5,
    "CD-CuD": 0.6,
    "CD-ED": 0.7,
    "SD-CuD": 0.7,
    "SD-ED": 0.8,
    "CuD-ED": 0.85
  }},
  "boundary_statuses": {{
    "CD-SD": "Maintained",
    "CD-CuD": "Transitional",
    "CD-ED": "Transitional",
    "SD-CuD": "Transitional",
    "SD-ED": "Transitional",
    "CuD-ED": "Transcendent"
  }},
  "identified_patterns": ["phenomenology", "meta_cognition", "experiential_inquiry"]
}}
</examples>

Now analyze this input:"#,
            user_input
        )
    }

    /// Call LLM #1 with retry logic and timeout
    async fn call_llm1_with_retry(&self, prompt: &str) -> Result<Llm1Output, LlmError> {
        let max_retries = self.config.llm1_max_retries;
        let timeout_ms = self.config.llm1_timeout_ms;
        let mut attempts = 0;
        let mut last_error: Option<LlmError> = None;

        while attempts <= max_retries {
            // Attempt LLM call with timeout
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                self.llm_provider.send_request(prompt),
            )
            .await
            {
                Ok(Ok(response_text)) => {
                    // Parse and validate JSON
                    match self.parse_and_validate(&response_text) {
                        Ok(output) => {
                            println!(
                                "LLM #1 succeeded on attempt {} (model: {})",
                                attempts + 1,
                                self.llm_provider.get_model_name()
                            );
                            return Ok(output);
                        }
                        Err(validation_err) => {
                            eprintln!(
                                "LLM #1 validation failed (attempt {}): {:?}",
                                attempts + 1,
                                validation_err
                            );
                            last_error = Some(LlmError::InvalidResponseFormat {
                                field: "validation".to_string(),
                                message: validation_err.to_string(),
                                raw_response: Some(response_text),
                            });
                        }
                    }
                }
                Ok(Err(llm_err)) => {
                    eprintln!(
                        "LLM #1 call failed (attempt {}): {:?}",
                        attempts + 1,
                        llm_err
                    );
                    last_error = Some(llm_err.clone());

                    // Don't retry auth errors
                    if matches!(llm_err, LlmError::AuthError { .. }) {
                        return Err(llm_err);
                    }
                }
                Err(_timeout) => {
                    eprintln!(
                        "LLM #1 timeout (attempt {}, {}ms)",
                        attempts + 1,
                        timeout_ms
                    );
                    last_error = Some(LlmError::NetworkError {
                        message: format!("LLM #1 timeout after {}ms", timeout_ms),
                        status_code: None,
                    });
                }
            }

            attempts += 1;
            if attempts <= max_retries {
                // Exponential backoff: 1s, 2s, 4s, ...
                let backoff_ms = 1000 * (2_u64.pow(attempts - 1));
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
            }
        }

        Err(last_error.unwrap_or_else(|| LlmError::NetworkError {
            message: format!("LLM #1 failed after {} attempts", max_retries + 1),
            status_code: None,
        }))
    }

    /// Parse LLM response and validate schema
    fn parse_and_validate(&self, response: &str) -> Result<Llm1Output, ValidationError> {
        // Try to extract JSON from markdown code blocks if present
        let json_str = if response.contains("```json") {
            // Extract from markdown code block
            let start = response.find("```json").unwrap() + 7;
            let end = response[start..].find("```").unwrap() + start;
            response[start..end].trim()
        } else if response.contains("```") {
            // Extract from generic code block
            let start = response.find("```").unwrap() + 3;
            let end = response[start..].find("```").unwrap() + start;
            response[start..end].trim()
        } else {
            response.trim()
        };

        // Parse JSON
        let output: Llm1Output = serde_json::from_str(json_str)?;

        // Validate schema (recognition paradigm doesn't need correction - trusts LLM recognition)
        output.validate()?;

        Ok(output)
    }

    /// Fallback to Rust domain calculator
    fn fallback_to_rust(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        eprintln!("Falling back to Rust domain calculations (LLM #1 failed or disabled)");
        self.fallback_processor.process(context)
    }
}

impl StageProcessor for UnconscciousLlmProcessor {
    fn name(&self) -> &str {
        "LLM #1 Unconscious Processor (Domain Emergence)"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // If dual-LLM disabled, use Rust fallback
        if !self.config.enabled {
            return self.fallback_to_rust(context);
        }

        // Build LLM #1 prompt
        let prompt = self.build_llm1_prompt(&context.user_input, None);

        // Call LLM #1 with retry logic (blocking on async)
        let llm1_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.call_llm1_with_retry(&prompt));

        match llm1_result {
            Ok(llm1_output) => {
                // Convert LLM #1 recognition-based output to FlowContext state
                context.domains = llm1_output
                    .domain_recognitions
                    .iter()
                    .map(|(name, recognition)| {
                        (
                            name.clone(),
                            DomainActivation {
                                activation: recognition.activation,
                            },
                        )
                    })
                    .collect();

                // Update boundaries with LLM #1 boundary states
                for boundary in &mut context.boundaries {
                    if let Some(state) = llm1_output.boundary_states.get(&boundary.name) {
                        boundary.permeability = state.permeability;
                        boundary.status = state.status.clone();
                    }
                }

                // Store pattern recognitions from LLM #1
                // Convert PatternRecognition to PatternObservation (simple description extraction)
                // Full lifecycle tracking deferred to Phase 4
                context.patterns = llm1_output
                    .pattern_recognitions
                    .iter()
                    .map(|pr| crate::flow_process::PatternObservation {
                        description: pr.description.clone(),
                    })
                    .collect();

                println!(
                    "LLM #1 processed input: {} domains activated, {} boundaries updated",
                    context.domains.len(),
                    context.boundaries.len()
                );

                Ok(())
            }
            Err(e) if self.config.fallback_enabled => {
                // Fallback to Rust calculations
                eprintln!("LLM #1 failed, falling back to Rust: {:?}", e);
                self.fallback_to_rust(context)
            }
            Err(e) => Err(FlowError::StageProcessingFailed {
                stage: "LLM #1 Unconscious Processor".to_string(),
                reason: format!("LLM #1 call failed: {:?}", e),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_engine::{BoundaryState, DomainRegistry, FrameworkState};

    /// Helper to create a basic FrameworkState for testing
    fn create_test_framework_state() -> FrameworkState {
        FrameworkState {
            domain_registry: DomainRegistry::new(),
            boundaries: vec![
                BoundaryState::new("CD-SD".to_string(), 0.5, "Maintained".to_string()),
                BoundaryState::new("SD-CuD".to_string(), 0.7, "Transitional".to_string()),
                BoundaryState::new("CuD-ED".to_string(), 0.85, "Transcendent".to_string()),
            ],
            identity: "Test Dual-LLM Identity".to_string(),
        }
    }

    #[test]
    fn test_llm1_prompt_construction() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let prompt = processor.build_llm1_prompt("Hello world", None);
        assert!(prompt.contains("Hello world"));
        assert!(prompt.contains("<user_input>"));
        assert!(prompt.contains("Computational Domain"));
        assert!(prompt.contains("domain_activations"));
    }

    #[test]
    fn test_parse_and_validate_with_valid_json() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let valid_json = r#"{
            "recognition_report": "I recognize strong computational perspective emerging",
            "domain_recognitions": {
                "CD": {"activation": 0.9, "emergence_note": "Strong computational framing"},
                "SD": {"activation": 0.7, "emergence_note": "Moderate scientific perspective"},
                "CuD": {"activation": 0.5, "emergence_note": "Some cultural context"},
                "ED": {"activation": 0.4, "emergence_note": "Minimal experiential dimension"}
            },
            "boundary_states": {
                "CD-SD": {"permeability": 0.77, "status": "Transitional", "tension_detected": true, "tension_type": "productive", "integration_invitation": "Integration invited", "resonance_note": "No context"},
                "CD-CuD": {"permeability": 0.65, "status": "Transitional", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
                "CD-ED": {"permeability": 0.58, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
                "SD-CuD": {"permeability": 0.58, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
                "SD-ED": {"permeability": 0.51, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
                "CuD-ED": {"permeability": 0.43, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"}
            },
            "quality_conditions": {
                "clarity_potential": 0.75, "depth_potential": 0.60, "precision_potential": 0.80,
                "fluidity_potential": 0.55, "resonance_potential": 0.50, "openness_potential": 0.45,
                "coherence_potential": 0.70, "reasoning": "High precision from CD-SD integration"
            },
            "pattern_recognitions": [
                {"type": "P¹", "lifecycle_stage": "emerging", "description": "Algorithm exploration",
                 "first_observed": "current_session", "emergence_context": "Technical question",
                 "developmental_trajectory": "", "significance": "New pattern"}
            ]
        }"#;

        let result = processor.parse_and_validate(valid_json);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(
            output.domain_recognitions.get("CD").unwrap().activation,
            0.9
        );
    }

    #[test]
    fn test_parse_and_validate_with_markdown() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let markdown_response = r#"Sure! Here's the JSON:

```json
{
    "recognition_report": "I recognize moderate computational perspective emerging",
    "domain_recognitions": {
        "CD": {"activation": 0.8, "emergence_note": "Computational framing present"},
        "SD": {"activation": 0.7, "emergence_note": "Scientific perspective moderate"},
        "CuD": {"activation": 0.5, "emergence_note": "Some cultural context"},
        "ED": {"activation": 0.4, "emergence_note": "Minimal experiential"}
    },
    "boundary_states": {
        "CD-SD": {"permeability": 0.74, "status": "Transitional", "tension_detected": true, "tension_type": "productive", "integration_invitation": "Integration invited", "resonance_note": "No context"},
        "CD-CuD": {"permeability": 0.63, "status": "Transitional", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
        "CD-ED": {"permeability": 0.56, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
        "SD-CuD": {"permeability": 0.59, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
        "SD-ED": {"permeability": 0.53, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"},
        "CuD-ED": {"permeability": 0.45, "status": "Maintained", "tension_detected": false, "tension_type": "neutral", "integration_invitation": "None", "resonance_note": "No context"}
    },
    "quality_conditions": {
        "clarity_potential": 0.70, "depth_potential": 0.55, "precision_potential": 0.75,
        "fluidity_potential": 0.50, "resonance_potential": 0.45, "openness_potential": 0.40,
        "coherence_potential": 0.65, "reasoning": "Moderate precision from CD-SD"
    },
    "pattern_recognitions": [
        {"type": "P¹", "lifecycle_stage": "emerging", "description": "Test pattern",
         "first_observed": "current_session", "emergence_context": "Test",
         "developmental_trajectory": "", "significance": "Test"}
    ]
}
```
"#;

        let result = processor.parse_and_validate(markdown_response);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(
            output.domain_recognitions.get("CD").unwrap().activation,
            0.8
        );
    }

    #[test]
    fn test_parse_and_validate_invalid_json_fails() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let invalid_json = "This is not valid JSON";
        let result = processor.parse_and_validate(invalid_json);
        assert!(result.is_err());
    }

    /// INTEGRATION TEST 1: End-to-end LLM #1 call with FlowProcess
    /// Validates that UnconscciousLlmProcessor integrates correctly with FlowContext
    #[test]
    fn test_integration_llm1_end_to_end_with_flow_context() {
        use crate::flow_process::FlowContext;

        // Create dual-LLM enabled config
        let config = DualLlmConfig::enabled();

        // Create MockLlm that returns valid LLM #1 recognition JSON
        let mock_llm = Arc::new(crate::mock_llm::MockLlm::llm1_recognition());

        // Create UnconscciousLlmProcessor
        let processor = UnconscciousLlmProcessor::new(mock_llm.clone(), config);

        // Create FlowContext with test input
        let framework_state = create_test_framework_state();
        let mut context = FlowContext::new(
            "Explain quantum entanglement".to_string(),
            0.7,
            framework_state.clone(),
        );

        // Manually initialize boundaries (normally done by Stage 2: BoundaryDissolutionProcessor)
        // For this integration test we just need the structure in place
        context.boundaries = framework_state.boundaries.clone();

        // Add required boundaries that LLM #1 will update
        context
            .boundaries
            .push(crate::prompt_engine::BoundaryState::new(
                "CD-SD".to_string(),
                0.5,
                "Maintained".to_string(),
            ));
        context
            .boundaries
            .push(crate::prompt_engine::BoundaryState::new(
                "CD-CuD".to_string(),
                0.5,
                "Maintained".to_string(),
            ));
        context
            .boundaries
            .push(crate::prompt_engine::BoundaryState::new(
                "CD-ED".to_string(),
                0.5,
                "Maintained".to_string(),
            ));
        context
            .boundaries
            .push(crate::prompt_engine::BoundaryState::new(
                "SD-CuD".to_string(),
                0.5,
                "Maintained".to_string(),
            ));
        context
            .boundaries
            .push(crate::prompt_engine::BoundaryState::new(
                "SD-ED".to_string(),
                0.5,
                "Maintained".to_string(),
            ));
        context
            .boundaries
            .push(crate::prompt_engine::BoundaryState::new(
                "CuD-ED".to_string(),
                0.5,
                "Maintained".to_string(),
            ));

        // Process through LLM #1 processor
        let result = processor.process(&mut context);

        // Verify processing succeeded
        assert!(result.is_ok(), "LLM #1 processing should succeed");

        // Verify domain activations were updated from LLM #1 output
        assert!(
            context.domains.contains_key("CD"),
            "CD domain should be present"
        );
        assert!(
            context.domains.contains_key("SD"),
            "SD domain should be present"
        );
        assert!(
            context.domains.contains_key("CuD"),
            "CuD domain should be present"
        );
        assert!(
            context.domains.contains_key("ED"),
            "ED domain should be present"
        );

        // Verify specific activation values match MockLlm response
        assert_eq!(
            context.domains.get("CD").unwrap().activation,
            0.85,
            "CD activation should match LLM #1 output"
        );
        assert_eq!(
            context.domains.get("SD").unwrap().activation,
            0.55,
            "SD activation should match LLM #1 output"
        );

        // Verify boundary states were updated
        let cd_sd_boundary = context
            .boundaries
            .iter()
            .find(|b| b.name == "CD-SD")
            .expect("CD-SD boundary should exist");
        assert_eq!(
            cd_sd_boundary.permeability, 0.72,
            "CD-SD permeability should match LLM #1 output"
        );
        assert_eq!(
            cd_sd_boundary.status, "Transitional",
            "CD-SD status should match LLM #1 output"
        );

        // Verify pattern storage (should have 1 pattern from LLM #1 output)
        assert_eq!(
            context.patterns.len(),
            1,
            "Should have 1 pattern from LLM #1 output"
        );
        assert_eq!(
            context.patterns[0].description, "Technical implementation inquiry pattern",
            "Pattern description should match LLM #1 output"
        );

        // Verify MockLlm was actually called
        assert_eq!(
            mock_llm.call_count(),
            1,
            "MockLlm should have been called exactly once"
        );
    }

    /// INTEGRATION TEST 2: Fallback to Rust on LLM #1 failure
    /// Validates that system gracefully degrades to Rust calculators when LLM fails
    #[test]
    fn test_integration_llm1_fallback_on_timeout() {
        use crate::flow_process::FlowContext;

        // Create dual-LLM enabled config with fallback enabled
        let mut config = DualLlmConfig::enabled();
        config.fallback_enabled = true;
        config.llm1_timeout_ms = 100; // Very short timeout
        config.llm1_max_retries = 0; // No retries for faster test

        // Create MockErrorLlm that simulates network timeout
        let mock_error_llm = Arc::new(crate::mock_llm::MockErrorLlm::network_error());

        // Create UnconscciousLlmProcessor with error-prone LLM
        let processor = UnconscciousLlmProcessor::new(mock_error_llm, config);

        // Create FlowContext
        let framework_state = create_test_framework_state();
        let mut context = FlowContext::new(
            "Test fallback behavior".to_string(),
            0.7,
            framework_state.clone(),
        );

        // Initialize boundaries
        context.boundaries = vec![
            crate::prompt_engine::BoundaryState::new(
                "CD-SD".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
            crate::prompt_engine::BoundaryState::new(
                "CD-CuD".to_string(),
                0.5,
                "Maintained".to_string(),
            ),
        ];

        // Process through LLM #1 processor (should fallback to Rust)
        let result = processor.process(&mut context);

        // Verify processing succeeded despite LLM failure (thanks to fallback)
        assert!(
            result.is_ok(),
            "Processing should succeed via Rust fallback"
        );

        // NOTE: We don't verify domain population here because that depends on
        // the domain registry being properly configured, which is tested separately.
        // The key test is that the fallback path executes without error.
    }

    /// INTEGRATION TEST 3: Feature flag disabled behavior
    /// Validates that dual-LLM mode respects the enabled flag
    #[test]
    fn test_integration_feature_flag_disabled() {
        use crate::flow_process::FlowContext;

        // Create dual-LLM DISABLED config
        let config = DualLlmConfig::disabled();

        // Create MockLlm (should NOT be called)
        let mock_llm = Arc::new(crate::mock_llm::MockLlm::llm1_recognition());

        // Create UnconscciousLlmProcessor
        let processor = UnconscciousLlmProcessor::new(mock_llm.clone(), config);

        // Create FlowContext
        let framework_state = create_test_framework_state();
        let mut context = FlowContext::new(
            "Test feature flag".to_string(),
            0.7,
            framework_state.clone(),
        );

        // Process through processor
        let result = processor.process(&mut context);

        // Verify processing succeeded
        assert!(result.is_ok(), "Processing should succeed in classic mode");

        // Verify MockLlm was NOT called (feature disabled, used Rust instead)
        assert_eq!(
            mock_llm.call_count(),
            0,
            "MockLlm should NOT be called when feature flag disabled"
        );

        // NOTE: We don't verify domain population here because that depends on
        // the domain registry being properly configured, which is tested separately.
        // The key test is that the feature flag correctly bypasses the LLM path.
    }
}
