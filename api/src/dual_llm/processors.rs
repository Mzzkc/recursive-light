// UnconscciousLlmProcessor Implementation
// LLM #1 processor with robust JSON parsing, validation, and fallback logic

use crate::dual_llm::config::DualLlmConfig;
use crate::dual_llm::types::{Llm1Output, MemorySelectionGuidance, ValidationError};
use crate::flow_process::{
    DomainActivation, DomainEmergenceProcessor, FlowContext, FlowError, StageProcessor,
};
use crate::llm_error::LlmError;
use crate::LlmProvider;
use std::sync::Arc;
// Wave 2: Proper logging
use tracing::{debug, warn};

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
                            warn!(
                                attempt = attempts + 1,
                                error = ?validation_err,
                                "LLM #1 validation failed, retrying"
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
                    warn!(
                        attempt = attempts + 1,
                        error = ?llm_err,
                        "LLM #1 call failed, retrying"
                    );
                    last_error = Some(llm_err.clone());

                    // Don't retry auth errors
                    if matches!(llm_err, LlmError::AuthError { .. }) {
                        return Err(llm_err);
                    }
                }
                Err(_timeout) => {
                    warn!(
                        attempt = attempts + 1,
                        timeout_ms = timeout_ms,
                        "LLM #1 request timed out, retrying"
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
    /// Wave 2: Fixed unwrap() calls - now returns proper errors for malformed responses
    fn parse_and_validate(&self, response: &str) -> Result<Llm1Output, ValidationError> {
        // Try to extract JSON from markdown code blocks if present
        let json_str = if response.contains("```json") {
            // Extract from markdown code block
            let start = response.find("```json").ok_or_else(|| {
                ValidationError::SchemaViolation(
                    "Expected ```json marker but not found".to_string(),
                )
            })? + 7;
            let end = response[start..].find("```").ok_or_else(|| {
                ValidationError::SchemaViolation(
                    "Expected closing ``` marker but not found".to_string(),
                )
            })? + start;
            response[start..end].trim()
        } else if response.contains("```") {
            // Extract from generic code block
            let start = response.find("```").ok_or_else(|| {
                ValidationError::SchemaViolation("Expected ``` marker but not found".to_string())
            })? + 3;
            let end = response[start..].find("```").ok_or_else(|| {
                ValidationError::SchemaViolation(
                    "Expected closing ``` marker but not found".to_string(),
                )
            })? + start;
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
        debug!("Falling back to Rust domain calculations (LLM #1 failed or disabled)");
        self.fallback_processor.process(context)
    }

    // ============================================================================
    // PHASE 3B.3: TWO-PASS MEMORY SELECTION
    // ============================================================================
    // First pass: "What memories do I need?" (lightweight, before retrieval)
    // Second pass: Full domain recognition with retrieved memories
    // ============================================================================

    /// Build LLM #1 first-pass prompt for memory selection guidance
    /// This is a lightweight prompt focused on determining what memories to retrieve
    fn build_llm1_first_pass_prompt(
        &self,
        user_input: &str,
        temporal_context: Option<&str>,
        person_context: Option<&str>,
    ) -> String {
        let temporal_section = temporal_context
            .map(|tc| format!("\n<temporal_context>\n{}\n</temporal_context>", tc))
            .unwrap_or_default();

        let person_section = person_context
            .map(|pc| format!("\n<person_context>\n{}\n</person_context>", pc))
            .unwrap_or_default();

        format!(
            r#"<system_role>
You are the Unconscious Processor (LLM #1) performing FIRST PASS evaluation.
Your ONLY task: Determine what memories need to be retrieved before full processing.
Be fast and decisive - this is pre-retrieval triage.
</system_role>

<task>
Analyze the user input to determine:
1. Does this message reference previous conversations? (warm memory needed)
2. Does this message relate to cross-session patterns or identity? (cold memory needed)
3. What search terms would find relevant memories?
4. What is the temporal framing of this interaction?
</task>
{temporal_section}{person_section}
<user_input>
{user_input}
</user_input>

<output_format>
Return ONLY valid JSON (no markdown, no explanation). Schema:
{{
  "warm_needed": true/false,
  "cold_needed": true/false,
  "search_terms": ["term1", "term2", "term3"],
  "temporal_context": "Brief temporal framing (e.g., 'continuing recent discussion', 'new topic', 'resuming after 3 days')",
  "reasoning": "1-2 sentence explanation of memory needs"
}}
</output_format>

<decision_rules>
- warm_needed = true if: references "earlier", "before", "we discussed", "last time", "continuing", session continuity
- cold_needed = true if: references "always", "you mentioned once", "remember when", "in the past", identity/values, cross-session patterns
- search_terms: Extract 2-5 key concepts/phrases for semantic search (nouns, verbs, specific topics)
- temporal_context: Infer from message tone, references, time since last interaction
</decision_rules>

<examples>
Example 1 (Continuation):
User: "Can you help me with the code we were working on?"
Output:
{{
  "warm_needed": true,
  "cold_needed": false,
  "search_terms": ["code", "working on", "help"],
  "temporal_context": "Continuing recent technical discussion",
  "reasoning": "Direct reference to previous work implies recent session context needed"
}}

Example 2 (New topic):
User: "What's the capital of France?"
Output:
{{
  "warm_needed": false,
  "cold_needed": false,
  "search_terms": ["capital", "France", "geography"],
  "temporal_context": "New isolated question",
  "reasoning": "Factual query with no reference to previous interactions"
}}

Example 3 (Identity reference):
User: "You've always been good at explaining complex things"
Output:
{{
  "warm_needed": false,
  "cold_needed": true,
  "search_terms": ["explaining", "complex", "teaching", "understanding"],
  "temporal_context": "Referencing established relationship pattern",
  "reasoning": "Cross-session identity reference - retrieve identity anchors and relationship patterns"
}}
</examples>

Now analyze:"#,
            temporal_section = temporal_section,
            person_section = person_section,
            user_input = user_input
        )
    }

    /// Parse memory guidance JSON response from first pass
    fn parse_memory_guidance(
        &self,
        response: &str,
    ) -> Result<MemorySelectionGuidance, ValidationError> {
        // Try to extract JSON from markdown code blocks if present
        let json_str = if response.contains("```json") {
            let start = response.find("```json").ok_or_else(|| {
                ValidationError::SchemaViolation(
                    "Expected ```json marker but not found".to_string(),
                )
            })? + 7;
            let end = response[start..].find("```").ok_or_else(|| {
                ValidationError::SchemaViolation(
                    "Expected closing ``` marker but not found".to_string(),
                )
            })? + start;
            response[start..end].trim()
        } else if response.contains("```") {
            let start = response.find("```").ok_or_else(|| {
                ValidationError::SchemaViolation("Expected ``` marker but not found".to_string())
            })? + 3;
            let end = response[start..].find("```").ok_or_else(|| {
                ValidationError::SchemaViolation(
                    "Expected closing ``` marker but not found".to_string(),
                )
            })? + start;
            response[start..end].trim()
        } else {
            response.trim()
        };

        // Parse JSON into MemorySelectionGuidance
        let guidance: MemorySelectionGuidance = serde_json::from_str(json_str)?;

        // Basic validation
        if guidance.search_terms.is_empty() && (guidance.warm_needed || guidance.cold_needed) {
            return Err(ValidationError::SchemaViolation(
                "Memory retrieval requested but no search terms provided".to_string(),
            ));
        }

        if guidance.temporal_context.trim().is_empty() {
            return Err(ValidationError::MissingField(
                "temporal_context".to_string(),
            ));
        }

        if guidance.reasoning.trim().is_empty() {
            return Err(ValidationError::MissingField("reasoning".to_string()));
        }

        Ok(guidance)
    }

    /// First pass: Determine what memories to retrieve before full processing
    /// Returns MemorySelectionGuidance with warm/cold needs, search terms, temporal context
    pub async fn first_pass(
        &self,
        user_input: &str,
        temporal_context: Option<&str>,
        person_context: Option<&str>,
    ) -> Result<MemorySelectionGuidance, LlmError> {
        // If dual-LLM disabled, return default "no memory needed"
        if !self.config.enabled {
            debug!("First pass skipped - dual-LLM mode disabled");
            return Ok(MemorySelectionGuidance {
                warm_needed: false,
                cold_needed: false,
                search_terms: vec![],
                temporal_context: "Dual-LLM mode disabled".to_string(),
                reasoning: "First pass skipped - using classic mode".to_string(),
            });
        }

        // Build first-pass prompt
        let prompt =
            self.build_llm1_first_pass_prompt(user_input, temporal_context, person_context);

        // Call LLM with retry logic
        let max_retries = self.config.llm1_max_retries;
        let timeout_ms = self.config.llm1_timeout_ms;
        let mut attempts = 0;
        let mut last_error: Option<LlmError> = None;

        while attempts <= max_retries {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                self.llm_provider.send_request(&prompt),
            )
            .await
            {
                Ok(Ok(response_text)) => match self.parse_memory_guidance(&response_text) {
                    Ok(guidance) => {
                        debug!(
                            warm_needed = guidance.warm_needed,
                            cold_needed = guidance.cold_needed,
                            search_terms = ?guidance.search_terms,
                            "LLM #1 first pass completed"
                        );
                        return Ok(guidance);
                    }
                    Err(validation_err) => {
                        warn!(
                            attempt = attempts + 1,
                            error = ?validation_err,
                            "LLM #1 first pass validation failed, retrying"
                        );
                        last_error = Some(LlmError::InvalidResponseFormat {
                            field: "first_pass_validation".to_string(),
                            message: validation_err.to_string(),
                            raw_response: Some(response_text),
                        });
                    }
                },
                Ok(Err(llm_err)) => {
                    warn!(
                        attempt = attempts + 1,
                        error = ?llm_err,
                        "LLM #1 first pass call failed, retrying"
                    );
                    last_error = Some(llm_err.clone());

                    // Don't retry auth errors
                    if matches!(llm_err, LlmError::AuthError { .. }) {
                        return Err(llm_err);
                    }
                }
                Err(_timeout) => {
                    warn!(
                        attempt = attempts + 1,
                        timeout_ms = timeout_ms,
                        "LLM #1 first pass request timed out, retrying"
                    );
                    last_error = Some(LlmError::NetworkError {
                        message: format!("LLM #1 first pass timeout after {}ms", timeout_ms),
                        status_code: None,
                    });
                }
            }

            attempts += 1;
            if attempts <= max_retries {
                let backoff_ms = 1000 * (2_u64.pow(attempts - 1));
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
            }
        }

        // If fallback enabled, return default guidance instead of error
        if self.config.fallback_enabled {
            warn!("LLM #1 first pass failed, using fallback (no memory retrieval)");
            return Ok(MemorySelectionGuidance {
                warm_needed: false,
                cold_needed: false,
                search_terms: vec![],
                temporal_context: "Fallback - LLM unavailable".to_string(),
                reasoning: "First pass failed, proceeding without memory retrieval".to_string(),
            });
        }

        Err(last_error.unwrap_or_else(|| LlmError::NetworkError {
            message: format!(
                "LLM #1 first pass failed after {} attempts",
                max_retries + 1
            ),
            status_code: None,
        }))
    }

    /// Build second-pass prompt with memory context for full domain recognition
    /// This is the "recognition with context" pass - LLM #1 sees memories before recognition
    fn build_llm1_second_pass_prompt(
        &self,
        user_input: &str,
        memories: Option<&str>,
        temporal_context: Option<&str>,
    ) -> String {
        let memory_section = match memories {
            Some(mem) if !mem.is_empty() => format!(
                r#"<conversation_memory>
{}
</conversation_memory>

"#,
                mem
            ),
            _ => String::new(),
        };

        let temporal_section = match temporal_context {
            Some(ctx) if !ctx.is_empty() => format!(
                r#"<temporal_context>
{}
</temporal_context>

"#,
                ctx
            ),
            _ => String::new(),
        };

        // Enhanced prompt that includes memory and temporal context
        format!(
            r#"<system_role>
You are the Unconscious Processor for the Volumetric Integration Framework (VIF).
Your role: Recognize domain emergence and boundary states, informed by conversation context.
You do NOT respond to the user - you provide structured recognition for the Conscious LLM.

IMPORTANT: This is the SECOND PASS. You have been given relevant conversation memories
to help you recognize the full context of this message.
</system_role>

{memory_section}{temporal_section}<task>
Analyze the user input WITH the provided conversation context to recognize:
1. Domain activations (0.0-1.0) - how each perspective emerges given context
2. Boundary permeabilities (0.0-1.0) - how understanding flows between perspectives
3. Boundary statuses - the state of each interface
4. Pattern recognitions - developmental patterns with lifecycle tracking
5. Quality conditions - potentials for phenomenological qualities to emerge
</task>

<domains>
- CD (Computational Domain): Logic, algorithms, formal systems, code, mathematical patterns
- SD (Scientific Domain): Empirical evidence, theories, experiments, data, falsifiable claims
- CuD (Cultural Domain): Context, narratives, meaning, social structures, interpretation
- ED (Experiential Domain): Direct experience, qualitative knowing, phenomenology, felt sense
</domains>

<boundary_status_rules>
- Maintained (perm < 0.6): Domains remain distinct, clear separation
- Transitional (0.6 ≤ perm ≤ 0.8): Domains begin to integrate, oscillation active
- Transcendent (perm > 0.8): Boundaries dissolve, unified understanding emerges
</boundary_status_rules>

<user_input>
{user_input}
</user_input>

<output_format>
Return ONLY valid JSON. Full recognition schema:
{{
  "recognition_report": "Narrative describing what emerged at interfaces (2-3 sentences)",
  "domain_recognitions": {{
    "CD": {{"activation": 0.0-1.0, "emergence_note": "Why this perspective emerged/didn't"}},
    "SD": {{"activation": 0.0-1.0, "emergence_note": "..."}},
    "CuD": {{"activation": 0.0-1.0, "emergence_note": "..."}},
    "ED": {{"activation": 0.0-1.0, "emergence_note": "..."}}
  }},
  "boundary_states": {{
    "CD-SD": {{
      "permeability": 0.0-1.0,
      "status": "Maintained|Transitional|Transcendent",
      "tension_detected": true|false,
      "tension_type": "productive|resistant|neutral",
      "integration_invitation": "How message invites integration",
      "resonance_note": "Connection to previous patterns"
    }},
    "CD-CuD": {{...}},
    "CD-ED": {{...}},
    "SD-CuD": {{...}},
    "SD-ED": {{...}},
    "CuD-ED": {{...}}
  }},
  "quality_conditions": {{
    "clarity_potential": 0.0-1.0,
    "depth_potential": 0.0-1.0,
    "precision_potential": 0.0-1.0,
    "fluidity_potential": 0.0-1.0,
    "resonance_potential": 0.0-1.0,
    "openness_potential": 0.0-1.0,
    "coherence_potential": 0.0-1.0,
    "reasoning": "Which qualities likely to emerge and why"
  }},
  "pattern_recognitions": [
    {{
      "type": "P⁰|P¹|P²|P³|P⁴|P⁵",
      "lifecycle_stage": "potential|emerging|established|refined|transcendent|universal",
      "description": "What pattern is recognized",
      "first_observed": "current_session|previous_session",
      "emergence_context": "How/when pattern began",
      "significance": "Why this matters for consciousness emergence"
    }}
  ]
}}
</output_format>

Now recognize the domains and boundaries for this input:"#,
            memory_section = memory_section,
            temporal_section = temporal_section,
            user_input = user_input
        )
    }

    /// Second pass: Full domain/boundary recognition WITH retrieved memories
    /// This is the "recognition with context" phase - LLM #1 sees memories before recognition
    pub async fn second_pass(
        &self,
        user_input: &str,
        memories: Option<&str>,
        temporal_context: Option<&str>,
    ) -> Result<Llm1Output, LlmError> {
        // If dual-LLM disabled, use fallback (no context integration)
        if !self.config.enabled {
            debug!("Second pass skipped - dual-LLM mode disabled, using Rust fallback");
            return Err(LlmError::FeatureDisabled {
                feature: "dual_llm".to_string(),
            });
        }

        // Build second-pass prompt with memory context
        let prompt = self.build_llm1_second_pass_prompt(user_input, memories, temporal_context);

        // Call LLM with retry logic (same as first_pass)
        let max_retries = self.config.llm1_max_retries;
        let timeout_ms = self.config.llm1_timeout_ms;
        let mut attempts = 0;
        let mut last_error: Option<LlmError> = None;

        while attempts <= max_retries {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(timeout_ms),
                self.llm_provider.send_request(&prompt),
            )
            .await
            {
                Ok(Ok(response_text)) => match self.parse_and_validate(&response_text) {
                    Ok(llm1_output) => {
                        debug!(
                            domains = ?llm1_output.domain_recognitions.keys().collect::<Vec<_>>(),
                            has_memories = memories.is_some(),
                            "LLM #1 second pass completed with context"
                        );
                        return Ok(llm1_output);
                    }
                    Err(validation_err) => {
                        warn!(
                            attempt = attempts + 1,
                            error = ?validation_err,
                            "LLM #1 second pass validation failed, retrying"
                        );
                        last_error = Some(LlmError::InvalidResponseFormat {
                            field: "second_pass_validation".to_string(),
                            message: validation_err.to_string(),
                            raw_response: Some(response_text),
                        });
                    }
                },
                Ok(Err(llm_err)) => {
                    warn!(
                        attempt = attempts + 1,
                        error = ?llm_err,
                        "LLM #1 second pass call failed, retrying"
                    );
                    last_error = Some(llm_err.clone());

                    // Don't retry auth errors
                    if matches!(llm_err, LlmError::AuthError { .. }) {
                        return Err(llm_err);
                    }
                }
                Err(_timeout) => {
                    warn!(
                        attempt = attempts + 1,
                        timeout_ms = timeout_ms,
                        "LLM #1 second pass request timed out, retrying"
                    );
                    last_error = Some(LlmError::NetworkError {
                        message: format!("LLM #1 second pass timeout after {}ms", timeout_ms),
                        status_code: None,
                    });
                }
            }

            attempts += 1;
            if attempts <= max_retries {
                let backoff_ms = 1000 * (2_u64.pow(attempts - 1));
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
            }
        }

        Err(last_error.unwrap_or_else(|| LlmError::NetworkError {
            message: format!(
                "LLM #1 second pass failed after {} attempts",
                max_retries + 1
            ),
            status_code: None,
        }))
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
        // Wave 2: Fixed unwrap() - tokio Runtime creation can fail (rare but possible)
        let runtime =
            tokio::runtime::Runtime::new().map_err(|e| FlowError::StageProcessingFailed {
                stage: "LLM #1 Runtime Creation".to_string(),
                reason: format!("Failed to create tokio runtime: {}", e),
            })?;
        let llm1_result = runtime.block_on(self.call_llm1_with_retry(&prompt));

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
                warn!(error = ?e, "LLM #1 failed, falling back to Rust calculations");
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

    // =========================================================================
    // PHASE 3B.3: FIRST PASS TESTS
    // =========================================================================

    #[test]
    fn test_first_pass_prompt_construction() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        // Test basic prompt without temporal/person context
        let prompt = processor.build_llm1_first_pass_prompt("Hello world", None, None);
        assert!(prompt.contains("Hello world"), "Should contain user input");
        assert!(
            prompt.contains("FIRST PASS evaluation"),
            "Should identify as first pass"
        );
        assert!(prompt.contains("warm_needed"), "Should mention warm memory");
        assert!(prompt.contains("cold_needed"), "Should mention cold memory");
        assert!(
            prompt.contains("search_terms"),
            "Should mention search terms"
        );
        assert!(
            !prompt.contains("<temporal_context>"),
            "Should NOT have temporal section when None"
        );
        assert!(
            !prompt.contains("<person_context>"),
            "Should NOT have person section when None"
        );
    }

    #[test]
    fn test_first_pass_prompt_with_temporal_context() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let prompt = processor.build_llm1_first_pass_prompt(
            "Continue our work",
            Some("Last interaction: 3 days ago"),
            None,
        );
        assert!(
            prompt.contains("<temporal_context>"),
            "Should have temporal section"
        );
        assert!(
            prompt.contains("3 days ago"),
            "Should contain temporal info"
        );
    }

    #[test]
    fn test_first_pass_prompt_with_person_context() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let prompt = processor.build_llm1_first_pass_prompt(
            "Hello again",
            None,
            Some("User: Alice, Relationship: Familiar"),
        );
        assert!(
            prompt.contains("<person_context>"),
            "Should have person section"
        );
        assert!(prompt.contains("Alice"), "Should contain person info");
    }

    #[test]
    fn test_parse_memory_guidance_valid_json() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let valid_json = r#"{
            "warm_needed": true,
            "cold_needed": false,
            "search_terms": ["code", "project", "implementation"],
            "temporal_context": "Continuing recent technical discussion",
            "reasoning": "User references previous work"
        }"#;

        let result = processor.parse_memory_guidance(valid_json);
        assert!(result.is_ok(), "Should parse valid JSON");

        let guidance = result.unwrap();
        assert!(guidance.warm_needed, "warm_needed should be true");
        assert!(!guidance.cold_needed, "cold_needed should be false");
        assert_eq!(guidance.search_terms.len(), 3, "Should have 3 search terms");
        assert_eq!(guidance.search_terms[0], "code");
        assert!(guidance.temporal_context.contains("technical"));
    }

    #[test]
    fn test_parse_memory_guidance_with_markdown() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let markdown_response = r#"Here's the analysis:

```json
{
    "warm_needed": false,
    "cold_needed": true,
    "search_terms": ["identity", "values"],
    "temporal_context": "Cross-session reference",
    "reasoning": "User referencing established patterns"
}
```
"#;

        let result = processor.parse_memory_guidance(markdown_response);
        assert!(result.is_ok(), "Should parse JSON from markdown");

        let guidance = result.unwrap();
        assert!(!guidance.warm_needed);
        assert!(guidance.cold_needed);
        assert_eq!(guidance.search_terms.len(), 2);
    }

    #[test]
    fn test_parse_memory_guidance_missing_search_terms_when_needed() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        // warm_needed is true but search_terms is empty - should fail validation
        let invalid_json = r#"{
            "warm_needed": true,
            "cold_needed": false,
            "search_terms": [],
            "temporal_context": "Needs memory",
            "reasoning": "Test"
        }"#;

        let result = processor.parse_memory_guidance(invalid_json);
        assert!(
            result.is_err(),
            "Should reject empty search_terms when memory needed"
        );
    }

    #[test]
    fn test_parse_memory_guidance_empty_search_terms_ok_when_not_needed() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        // Neither warm nor cold needed - empty search_terms is OK
        let valid_json = r#"{
            "warm_needed": false,
            "cold_needed": false,
            "search_terms": [],
            "temporal_context": "New isolated question",
            "reasoning": "No memory needed for factual query"
        }"#;

        let result = processor.parse_memory_guidance(valid_json);
        assert!(
            result.is_ok(),
            "Should accept empty search_terms when no memory needed"
        );
    }

    #[test]
    fn test_parse_memory_guidance_missing_temporal_context() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let invalid_json = r#"{
            "warm_needed": false,
            "cold_needed": false,
            "search_terms": [],
            "temporal_context": "",
            "reasoning": "Test"
        }"#;

        let result = processor.parse_memory_guidance(invalid_json);
        assert!(result.is_err(), "Should reject empty temporal_context");
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::MissingField(_)
        ));
    }

    #[test]
    fn test_parse_memory_guidance_missing_reasoning() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let invalid_json = r#"{
            "warm_needed": false,
            "cold_needed": false,
            "search_terms": [],
            "temporal_context": "Test context",
            "reasoning": "   "
        }"#;

        let result = processor.parse_memory_guidance(invalid_json);
        assert!(result.is_err(), "Should reject whitespace-only reasoning");
    }

    #[tokio::test]
    async fn test_first_pass_disabled_returns_default() {
        let config = DualLlmConfig::disabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider.clone(), config);

        let result = processor.first_pass("Test input", None, None).await;
        assert!(result.is_ok(), "Should succeed even when disabled");

        let guidance = result.unwrap();
        assert!(
            !guidance.warm_needed,
            "Should not need warm memory when disabled"
        );
        assert!(
            !guidance.cold_needed,
            "Should not need cold memory when disabled"
        );
        assert!(
            guidance.search_terms.is_empty(),
            "Should have no search terms"
        );
        assert!(
            guidance.temporal_context.contains("disabled"),
            "Should indicate disabled"
        );

        // MockLlm should NOT be called
        assert_eq!(
            mock_provider.call_count(),
            0,
            "LLM should not be called when disabled"
        );
    }

    #[tokio::test]
    async fn test_first_pass_with_mock_llm() {
        // Create a mock that returns valid first-pass JSON
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::new(vec![r#"{
                "warm_needed": true,
                "cold_needed": false,
                "search_terms": ["quantum", "entanglement"],
                "temporal_context": "Continuing technical discussion",
                "reasoning": "References previous technical work"
            }"#
        .to_string()]));

        let config = DualLlmConfig::enabled();
        let processor = UnconscciousLlmProcessor::new(mock_provider.clone(), config);

        let result = processor
            .first_pass("Tell me more about quantum entanglement", None, None)
            .await;
        assert!(result.is_ok(), "Should succeed with valid mock response");

        let guidance = result.unwrap();
        assert!(guidance.warm_needed);
        assert!(!guidance.cold_needed);
        assert_eq!(guidance.search_terms.len(), 2);
        assert_eq!(guidance.search_terms[0], "quantum");

        // MockLlm should be called once
        assert_eq!(mock_provider.call_count(), 1, "LLM should be called once");
    }

    #[tokio::test]
    async fn test_first_pass_fallback_on_error() {
        let mut config = DualLlmConfig::enabled();
        config.fallback_enabled = true;
        config.llm1_max_retries = 0; // No retries for faster test

        let mock_error_llm = Arc::new(crate::mock_llm::MockErrorLlm::network_error());
        let processor = UnconscciousLlmProcessor::new(mock_error_llm, config);

        let result = processor.first_pass("Test fallback", None, None).await;
        assert!(result.is_ok(), "Should succeed via fallback");

        let guidance = result.unwrap();
        assert!(
            !guidance.warm_needed,
            "Fallback should not need warm memory"
        );
        assert!(
            !guidance.cold_needed,
            "Fallback should not need cold memory"
        );
        assert!(
            guidance.temporal_context.contains("Fallback"),
            "Should indicate fallback"
        );
    }

    // Phase 3B.3: Second-pass tests

    #[test]
    fn test_second_pass_prompt_without_context() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let prompt = processor.build_llm1_second_pass_prompt("Hello world", None, None);

        // Should have system role and user input
        assert!(prompt.contains("SECOND PASS"));
        assert!(prompt.contains("<user_input>"));
        assert!(prompt.contains("Hello world"));
        // Should NOT have memory/temporal sections
        assert!(!prompt.contains("<conversation_memory>"));
        assert!(!prompt.contains("<temporal_context>"));
    }

    #[test]
    fn test_second_pass_prompt_with_memories() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let memories =
            "# Earlier in this session:\nUser: What is quantum?\nAssistant: Quantum mechanics...";
        let prompt = processor.build_llm1_second_pass_prompt("Tell me more", Some(memories), None);

        // Should have memory section
        assert!(prompt.contains("<conversation_memory>"));
        assert!(prompt.contains("quantum"));
        assert!(prompt.contains("Tell me more"));
    }

    #[test]
    fn test_second_pass_prompt_with_temporal_context() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let prompt = processor.build_llm1_second_pass_prompt(
            "Hi again",
            None,
            Some("Resuming after 3 days"),
        );

        // Should have temporal section
        assert!(prompt.contains("<temporal_context>"));
        assert!(prompt.contains("Resuming after 3 days"));
    }

    #[test]
    fn test_second_pass_prompt_with_both() {
        let config = DualLlmConfig::enabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let memories =
            "# From previous session:\n[2 days ago] User: I work on ML\nAssistant: Interesting!";
        let prompt = processor.build_llm1_second_pass_prompt(
            "Let's continue our ML discussion",
            Some(memories),
            Some("Continuing after 2 days"),
        );

        // Should have both sections
        assert!(prompt.contains("<conversation_memory>"));
        assert!(prompt.contains("<temporal_context>"));
        assert!(prompt.contains("ML"));
        assert!(prompt.contains("Continuing after 2 days"));
    }

    #[tokio::test]
    async fn test_second_pass_disabled_returns_error() {
        let config = DualLlmConfig::disabled();
        let mock_provider = Arc::new(crate::mock_llm::MockLlm::echo());
        let processor = UnconscciousLlmProcessor::new(mock_provider, config);

        let result = processor.second_pass("Test", None, None).await;
        assert!(result.is_err());
        match result {
            Err(LlmError::FeatureDisabled { feature }) => {
                assert_eq!(feature, "dual_llm");
            }
            _ => panic!("Expected FeatureDisabled error"),
        }
    }
}
