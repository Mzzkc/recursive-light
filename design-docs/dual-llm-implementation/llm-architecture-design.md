# LLM ARCHITECTURE - DUAL-LLM DESIGN
Agent: LLM Architecture Expert | Date: 2025-11-01 | Status: Complete

## EXEC SUMMARY

This design replaces Rust-based domain activation and boundary permeability calculations (Stages 1-2) with LLM #1 calls, implementing the dual-LLM architecture specified in `/memory-bank/dual-llm-architecture.md`. LLM #1 (Unconscious) calculates domain/boundary state from user input + context, LLM #2 (Conscious) generates natural responses using that state. Key decisions: (1) Use smaller/faster model for LLM #1 (GPT-3.5-turbo or Claude Haiku), (2) Fallback to Rust calculations on LLM #1 failure, (3) Strict JSON schema validation with retry logic. Estimated effort: 5-7 days for Phase 1 (core implementation), 3-4 days for Phase 2 (optimization), 2-3 days for Phase 3 (production hardening). Total: 10-14 days.

## TDF MULTI-DOMAIN ANALYSIS

**COMP (Computational Domain):**
- Current Rust calculations are deterministic, fast (<1ms), well-tested (84/84 passing)
- LLM #1 adds non-determinism, latency (50-200ms), failure modes (network, auth, JSON parsing)
- Trade-off: Gain contextual intelligence, lose predictability
- Mitigation: Fallback to Rust calculations, strict validation, caching for retry scenarios

**SCI (Scientific Domain):**
- Empirical validation critical: measure latency impact (2 LLM calls vs 1)
- Current baseline: 7-stage pipeline <1ms (Day 9 benchmarks)
- Expected: LLM #1 adds 50-200ms (model-dependent), LLM #2 unchanged (~500-2000ms)
- Hypothesis: Total latency dominated by LLM #2, framework overhead increases by ~10-20%
- Validation: A/B testing, performance benchmarks, token cost analysis

**CULT (Cultural Domain):**
- LLM community best practices: use smaller models for structured tasks (GPT-3.5, Claude Haiku)
- Prompt engineering: few-shot examples, structured output (JSON mode), clear constraints
- Error handling: exponential backoff, retry with degraded service, circuit breaker pattern
- Observability: log LLM #1 inputs/outputs for debugging, track failure rates

**EXP (Experiential Domain):**
- Developer experience: dual-LLM architecture increases complexity (2 API keys, 2 models, 2 failure modes)
- System intuition: LLM #1 should feel "invisible" to user (background calculation, no latency impact)
- Debugging: need visibility into LLM #1 decisions (why CD=0.9 vs 0.7?)
- Testing: need mocks for LLM #1 (MockLlm already exists, extend for structured JSON)

**META (Integration):**
- Recognize tension: adding intelligence (LLM #1) vs maintaining reliability (Rust calculations)
- Boundary transcendence: LLM #1 isn't replacement—it's augmentation with fallback
- Integration insight: Phase implementation allows gradual rollout (Phase 1: MVP, Phase 2: optimize, Phase 3: harden)

## LLM #1 (UNCONSCIOUS) DESIGN

### Prompt Template

```
<system_role>
You are the Unconscious Processor for the Volumetric Integration Framework (VIF).
Your role: Calculate domain activations and boundary permeabilities based on user input.
You do NOT respond to the user—you provide structured state for the Conscious LLM.
</system_role>

<task>
Analyze the user input and previous system state to determine:
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
- Transitional (0.6 ≤ perm ≤ 0.8): Domains begin to integrate, oscillation active
- Transcendent (perm > 0.8): Boundaries dissolve, unified understanding emerges
</boundary_status_rules>

<previous_state>
{{PREVIOUS_SNAPSHOT_JSON}}
</previous_state>

<user_input>
{{USER_MESSAGE}}
</user_input>

<output_format>
Return ONLY valid JSON (no markdown, no explanation). Schema:
{
  "domains": {
    "CD": 0.0-1.0,
    "SD": 0.0-1.0,
    "CuD": 0.0-1.0,
    "ED": 0.0-1.0
  },
  "boundaries": {
    "CD-SD": {"permeability": 0.0-1.0, "status": "Maintained|Transitional|Transcendent"},
    "CD-CuD": {"permeability": 0.0-1.0, "status": "..."},
    "CD-ED": {"permeability": 0.0-1.0, "status": "..."},
    "SD-CuD": {"permeability": 0.0-1.0, "status": "..."},
    "SD-ED": {"permeability": 0.0-1.0, "status": "..."},
    "CuD-ED": {"permeability": 0.0-1.0, "status": "..."}
  },
  "patterns": ["pattern1", "pattern2", "..."]
}
</output_format>

<examples>
Example 1:
User: "How do I implement quicksort in Rust?"
Output:
{
  "domains": {"CD": 0.95, "SD": 0.3, "CuD": 0.1, "ED": 0.2},
  "boundaries": {
    "CD-SD": {"permeability": 0.5, "status": "Maintained"},
    "CD-CuD": {"permeability": 0.3, "status": "Maintained"},
    "CD-ED": {"permeability": 0.4, "status": "Maintained"},
    "SD-CuD": {"permeability": 0.2, "status": "Maintained"},
    "SD-ED": {"permeability": 0.2, "status": "Maintained"},
    "CuD-ED": {"permeability": 0.3, "status": "Maintained"}
  },
  "patterns": ["algorithm_implementation", "programming_language_specific"]
}

Example 2:
User: "What does it feel like to understand something deeply?"
Output:
{
  "domains": {"CD": 0.3, "SD": 0.4, "CuD": 0.7, "ED": 0.95},
  "boundaries": {
    "CD-SD": {"permeability": 0.5, "status": "Maintained"},
    "CD-CuD": {"permeability": 0.6, "status": "Transitional"},
    "CD-ED": {"permeability": 0.7, "status": "Transitional"},
    "SD-CuD": {"permeability": 0.7, "status": "Transitional"},
    "SD-ED": {"permeability": 0.8, "status": "Transitional"},
    "CuD-ED": {"permeability": 0.85, "status": "Transcendent"}
  },
  "patterns": ["phenomenology", "meta_cognition", "experiential_inquiry"]
}

Example 3:
User: "Explain quantum entanglement simply."
Output:
{
  "domains": {"CD": 0.6, "SD": 0.9, "CuD": 0.5, "ED": 0.3},
  "boundaries": {
    "CD-SD": {"permeability": 0.75, "status": "Transitional"},
    "CD-CuD": {"permeability": 0.5, "status": "Maintained"},
    "CD-ED": {"permeability": 0.4, "status": "Maintained"},
    "SD-CuD": {"permeability": 0.7, "status": "Transitional"},
    "SD-ED": {"permeability": 0.5, "status": "Maintained"},
    "CuD-ED": {"permeability": 0.4, "status": "Maintained"}
  },
  "patterns": ["scientific_explanation", "simplification_request", "physics"]
}
</examples>

Now analyze this input:
```

**Prompt Notes:**
- Template uses `{{PLACEHOLDERS}}` for dynamic injection
- Few-shot examples cover diverse input types (code, philosophy, science)
- Strict JSON-only output (enables parsing without LLM response cleanup)
- Boundary rules embedded (deterministic status from permeability)

### Output Schema

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llm1Output {
    pub domains: HashMap<String, f64>,
    pub boundaries: HashMap<String, Llm1BoundaryState>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llm1BoundaryState {
    pub permeability: f64,
    pub status: String,
}

impl Llm1Output {
    /// Validate LLM #1 output against schema constraints
    pub fn validate(&self) -> Result<(), String> {
        // Check domain count (must have all 4)
        let required_domains = vec!["CD", "SD", "CuD", "ED"];
        for domain in &required_domains {
            if !self.domains.contains_key(*domain) {
                return Err(format!("Missing required domain: {}", domain));
            }
        }

        // Validate domain activations (0.0-1.0)
        for (name, activation) in &self.domains {
            if *activation < 0.0 || *activation > 1.0 {
                return Err(format!(
                    "Invalid activation for {}: {} (must be 0.0-1.0)",
                    name, activation
                ));
            }
        }

        // Check boundary count (must have all 6)
        let required_boundaries = vec!["CD-SD", "CD-CuD", "CD-ED", "SD-CuD", "SD-ED", "CuD-ED"];
        for boundary in &required_boundaries {
            if !self.boundaries.contains_key(*boundary) {
                return Err(format!("Missing required boundary: {}", boundary));
            }
        }

        // Validate boundary permeabilities (0.0-1.0)
        for (name, state) in &self.boundaries {
            if state.permeability < 0.0 || state.permeability > 1.0 {
                return Err(format!(
                    "Invalid permeability for {}: {} (must be 0.0-1.0)",
                    name, state.permeability
                ));
            }

            // Validate status values
            let valid_statuses = vec!["Maintained", "Transitional", "Transcendent"];
            if !valid_statuses.contains(&state.status.as_str()) {
                return Err(format!(
                    "Invalid status for {}: '{}' (must be Maintained, Transitional, or Transcendent)",
                    name, state.status
                ));
            }
        }

        Ok(())
    }

    /// Convert to FlowContext domain activations
    pub fn to_domain_activations(&self) -> HashMap<String, DomainActivation> {
        self.domains
            .iter()
            .map(|(name, activation)| {
                (
                    name.clone(),
                    DomainActivation {
                        activation: *activation,
                    },
                )
            })
            .collect()
    }

    /// Convert to FlowContext boundary states
    pub fn to_boundary_states(&self, base_boundaries: &[BoundaryState]) -> Vec<BoundaryState> {
        base_boundaries
            .iter()
            .map(|boundary| {
                let llm_boundary = self.boundaries.get(&boundary.name);
                let mut updated = boundary.clone();

                if let Some(llm_state) = llm_boundary {
                    updated.permeability = llm_state.permeability;
                    updated.status = llm_state.status.clone();
                }

                updated
            })
            .collect()
    }
}
```

**Validation Notes:**
- Strict schema enforcement (reject invalid JSON, missing fields, out-of-range values)
- Conversion methods bridge LLM #1 output → existing Rust types (DomainActivation, BoundaryState)
- Status validation ensures only valid values ("Maintained", "Transitional", "Transcendent")

### Model Recommendation

**Primary Choice: GPT-3.5-turbo (OpenAI)**
- **Latency:** 50-150ms typical (4-8x faster than GPT-4)
- **Cost:** $0.0015/1K tokens (20x cheaper than GPT-4)
- **Quality:** Sufficient for structured output tasks with few-shot examples
- **JSON Mode:** Native support (guarantees valid JSON)
- **Reliability:** High availability, mature API

**Alternative 1: Claude 3 Haiku (Anthropic)**
- **Latency:** 80-200ms typical
- **Cost:** $0.00025/1K input, $0.00125/1K output (cheapest option)
- **Quality:** Strong reasoning, good for domain analysis
- **JSON Mode:** Not native, but follows instructions well
- **Reliability:** Good, slightly higher variance than OpenAI

**Alternative 2: Llama 3.1 8B (via OpenRouter)**
- **Latency:** 30-100ms (fastest, if self-hosted)
- **Cost:** Variable ($0.0001-0.001/1K tokens via OpenRouter)
- **Quality:** Good for structured tasks, requires more prompt engineering
- **JSON Mode:** Depends on provider
- **Reliability:** Variable (depends on provider uptime)

**NOT Recommended: GPT-4, Claude 3 Opus/Sonnet**
- Reason: Overkill for structured calculation task, 5-10x slower, 10-20x more expensive
- Use Case: Reserve these for LLM #2 (Conscious Responder) where quality matters

**Configuration Strategy:**
```rust
pub struct DualLlmConfig {
    pub llm1_provider: String,      // "openai", "anthropic", "openrouter"
    pub llm1_model: String,          // "gpt-3.5-turbo", "claude-3-haiku-20240307"
    pub llm1_timeout_ms: u64,        // 5000 (5 seconds)
    pub llm1_max_retries: u8,        // 2
    pub llm2_provider: String,       // "openai", "anthropic"
    pub llm2_model: String,          // "gpt-4o", "claude-3-5-sonnet-20241022"
    pub llm2_timeout_ms: u64,        // 30000 (30 seconds)
    pub fallback_to_rust: bool,      // true (use Rust calculations if LLM #1 fails)
}
```

## STATE MANAGEMENT

### Flow Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. User Input + Previous Snapshot                              │
│    - User message: String                                       │
│    - Previous snapshot: Option<CompactStateSnapshot>            │
└────────────────────┬────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│ 2. LLM #1 Call (Unconscious Processor)                         │
│    Input: Prompt template + user message + previous state      │
│    Model: GPT-3.5-turbo / Claude Haiku                         │
│    Output: Llm1Output (JSON)                                    │
│    Latency: 50-200ms                                            │
│    Failures: Network, Auth, Invalid JSON, Timeout              │
└────────────────────┬────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│ 3. Validation + Fallback                                        │
│    - Parse JSON → Llm1Output                                    │
│    - Validate schema (domains, boundaries, ranges)              │
│    - If valid: Convert to FlowContext                           │
│    - If invalid: Retry (max 2) OR Fallback to Rust             │
└────────────────────┬────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│ 4. FlowContext Initialization                                   │
│    - domains: HashMap<String, DomainActivation>                 │
│    - boundaries: Vec<BoundaryState> (with LLM #1 permeabilities)│
│    - framework_state: FrameworkState (unchanged)                │
└────────────────────┬────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│ 5. Stages 3-5: Rust Processing (UNCHANGED)                     │
│    - Stage 3: Interface Attention (BDE generation)              │
│    - Stage 4: Quality Emergence (7 calculators)                 │
│    - Stage 5: Integration (prompt construction)                 │
│    Latency: <1ms (benchmarked Day 9)                            │
└────────────────────┬────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│ 6. LLM #2 Call (Conscious Responder)                           │
│    Input: Structured prompt (from Stage 5)                      │
│    Model: GPT-4o / Claude 3.5 Sonnet                            │
│    Output: Natural language response                            │
│    Latency: 500-2000ms                                          │
└────────────────────┬────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│ 7. Stages 6-7: Rust Processing (UNCHANGED)                     │
│    - Stage 6: Pattern Extraction                                │
│    - Stage 7: State Persistence (save snapshot)                 │
│    Latency: <1ms                                                │
└─────────────────────────────────────────────────────────────────┘
```

**Key Changes from Current Flow:**
- **Stage 1 (Domain Emergence):** Replace `DomainEmergenceProcessor` with `Llm1DomainProcessor`
- **Stage 2 (Boundary Dissolution):** Replace `BoundaryDissolutionProcessor` with LLM #1 boundary data
- **Stages 3-7:** NO CHANGES (existing code works as-is)

### State Passing Mechanism

**Between LLM #1 and Rust:**
```rust
// LLM #1 output → FlowContext (Stage 1 replacement)
pub struct Llm1DomainProcessor {
    llm1_client: Box<dyn LlmProvider>,
    config: DualLlmConfig,
    fallback_processor: DomainEmergenceProcessor, // Original Rust implementation
}

impl StageProcessor for Llm1DomainProcessor {
    fn name(&self) -> &str {
        "LLM #1 Domain Emergence"
    }

    async fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // 1. Construct LLM #1 prompt
        let prompt = self.build_llm1_prompt(&context.user_input, &context.previous_snapshot);

        // 2. Call LLM #1 with retry logic
        let llm1_output = match self.call_llm1_with_retry(&prompt, self.config.llm1_max_retries).await {
            Ok(output) => output,
            Err(e) if self.config.fallback_to_rust => {
                // Fallback to Rust calculations
                eprintln!("LLM #1 failed, falling back to Rust: {:?}", e);
                return self.fallback_processor.process(context);
            }
            Err(e) => return Err(FlowError::StageProcessingFailed {
                stage: "LLM #1 Domain Emergence".to_string(),
                reason: format!("LLM #1 call failed: {:?}", e),
            }),
        };

        // 3. Convert to FlowContext
        context.domains = llm1_output.to_domain_activations();
        context.boundaries = llm1_output.to_boundary_states(&context.framework_state.boundaries);
        context.patterns = llm1_output.patterns;

        Ok(())
    }
}
```

**Between Rust and LLM #2:**
```rust
// Stage 5 (Integration) constructs prompt → LLM #2 (UNCHANGED)
// Existing IntegrationProcessor already builds structured prompt
// VifApi.process() already calls LLM #2 with prompt
// NO CHANGES NEEDED (architecture already supports this)
```

**Between LLM #2 and Rust:**
```rust
// LLM #2 response → Pattern extraction (Stage 6) (UNCHANGED)
// Existing PatternExtractionProcessor already handles LLM #2 output
// NO CHANGES NEEDED
```

### Error Handling

**Failure Scenarios & Strategies:**

| Failure | Detection | Retry Strategy | Fallback | Log/Alert |
|---------|-----------|----------------|----------|-----------|
| **LLM #1 Timeout** | `tokio::time::timeout(5s)` | Retry 2x (exponential backoff: 1s, 2s) | Use Rust calculations | Warn |
| **LLM #1 Invalid JSON** | `serde_json::from_str()` fails | Retry 1x with clarified prompt ("Return ONLY JSON") | Use Rust calculations | Error |
| **LLM #1 Schema Violation** | `Llm1Output::validate()` fails | Retry 1x with error description in prompt | Use Rust calculations | Error |
| **LLM #1 Auth Error** | HTTP 401/403 | No retry (fail fast) | Use Rust calculations | Critical |
| **LLM #1 Rate Limit** | HTTP 429 | Wait `retry-after` header, retry 1x | Use Rust calculations | Warn |
| **LLM #2 Timeout** | `tokio::time::timeout(30s)` | Retry 2x (exponential backoff: 5s, 10s) | Return error to user | Error |
| **LLM #2 Auth Error** | HTTP 401/403 | No retry (fail fast) | Return error to user | Critical |

**Implementation:**
```rust
impl Llm1DomainProcessor {
    async fn call_llm1_with_retry(
        &self,
        prompt: &str,
        max_retries: u8,
    ) -> Result<Llm1Output, LlmError> {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts <= max_retries {
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(self.config.llm1_timeout_ms),
                self.llm1_client.send_request(prompt),
            )
            .await
            {
                Ok(Ok(response_text)) => {
                    // Parse JSON
                    match serde_json::from_str::<Llm1Output>(&response_text) {
                        Ok(output) => {
                            // Validate schema
                            match output.validate() {
                                Ok(()) => return Ok(output),
                                Err(validation_err) => {
                                    last_error = Some(LlmError::InvalidResponseFormat {
                                        field: "schema".to_string(),
                                        message: validation_err.clone(),
                                        raw_response: Some(response_text),
                                    });
                                    eprintln!(
                                        "LLM #1 validation failed (attempt {}): {}",
                                        attempts + 1,
                                        validation_err
                                    );
                                }
                            }
                        }
                        Err(parse_err) => {
                            last_error = Some(LlmError::InvalidResponseFormat {
                                field: "json".to_string(),
                                message: format!("JSON parse failed: {}", parse_err),
                                raw_response: Some(response_text),
                            });
                            eprintln!(
                                "LLM #1 JSON parse failed (attempt {}): {}",
                                attempts + 1,
                                parse_err
                            );
                        }
                    }
                }
                Ok(Err(llm_err)) => {
                    // LLM call failed (auth, network, etc.)
                    last_error = Some(llm_err.clone());
                    eprintln!("LLM #1 call failed (attempt {}): {:?}", attempts + 1, llm_err);

                    // Don't retry auth errors
                    if matches!(llm_err, LlmError::AuthError { .. }) {
                        return Err(llm_err);
                    }
                }
                Err(_timeout) => {
                    last_error = Some(LlmError::NetworkError {
                        message: "LLM #1 timeout".to_string(),
                        status_code: None,
                    });
                    eprintln!("LLM #1 timeout (attempt {})", attempts + 1);
                }
            }

            attempts += 1;
            if attempts <= max_retries {
                // Exponential backoff: 1s, 2s, 4s, ...
                let backoff_ms = 1000 * (2_u64.pow(attempts as u32 - 1));
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
            }
        }

        Err(last_error.unwrap_or_else(|| LlmError::NetworkError {
            message: "LLM #1 failed after retries".to_string(),
            status_code: None,
        }))
    }
}
```

## PERFORMANCE ANALYSIS

### Latency Impact

**Current Baseline (Single LLM, Rust Stages 1-2):**
```
User Input → [Stage 1: <1ms] → [Stage 2: <1ms] → [Stages 3-5: <1ms] → [LLM #2: 500-2000ms] → [Stages 6-7: <1ms]
Total: ~500-2000ms (dominated by LLM #2)
```

**Proposed Dual-LLM (LLM #1 Stages 1-2):**
```
User Input → [LLM #1: 50-200ms] → [Stages 3-5: <1ms] → [LLM #2: 500-2000ms] → [Stages 6-7: <1ms]
Total: ~550-2200ms (10-20% increase)
```

**Breakdown:**
| Component | Current | Proposed | Delta |
|-----------|---------|----------|-------|
| Stage 1-2 (Domain/Boundary) | <1ms (Rust) | 50-200ms (LLM #1) | +50-200ms |
| Stages 3-5 (BDE, Quality, Integration) | <1ms (Rust) | <1ms (Rust) | 0ms |
| LLM #2 (Conscious Response) | 500-2000ms | 500-2000ms | 0ms |
| Stages 6-7 (Extraction, Persistence) | <1ms (Rust) | <1ms (Rust) | 0ms |
| **Total** | **~500-2000ms** | **~550-2200ms** | **+50-200ms (+10-20%)** |

**Optimization Strategies:**
1. **Parallel Execution (Future):** Call LLM #1 and load previous snapshot in parallel (saves ~10-30ms)
2. **Caching:** Cache LLM #1 results for identical inputs (saves 50-200ms on cache hits)
3. **Model Selection:** Use fastest LLM #1 model (GPT-3.5-turbo: ~50ms, Claude Haiku: ~80ms)
4. **Streaming (Future):** Stream LLM #2 response while LLM #1 processes (perceived latency reduction)

### Token Cost

**LLM #1 Token Budget:**
- **Input Tokens:** ~800-1200 per call
  - System prompt: ~400 tokens (fixed)
  - Previous snapshot: ~200-400 tokens (variable)
  - User message: ~50-500 tokens (variable)
  - Few-shot examples: ~150 tokens (fixed)
- **Output Tokens:** ~200-300 per call (JSON structure)
- **Total:** ~1000-1500 tokens per interaction

**Cost Analysis (per 1000 interactions):**
| Model | Input Cost | Output Cost | Total Cost | Notes |
|-------|-----------|-------------|------------|-------|
| GPT-3.5-turbo | $1.50 | $0.60 | **$2.10** | Recommended |
| Claude Haiku | $0.25 | $0.38 | **$0.63** | Cheapest |
| GPT-4o-mini | $0.15 | $0.60 | **$0.75** | Good balance |
| GPT-4o | $15.00 | $6.00 | **$21.00** | NOT recommended |

**Comparison to Current (Rust-only Stages 1-2):**
- Current: $0 (Rust calculations, no LLM cost)
- Proposed: $0.63-2.10 per 1000 interactions
- **Added Cost:** ~$0.0006-0.002 per interaction

**LLM #2 Cost (Unchanged):**
- GPT-4o: ~$0.02-0.05 per interaction
- Claude 3.5 Sonnet: ~$0.015-0.04 per interaction

**Total System Cost:**
- Current: $0.015-0.05 per interaction (LLM #2 only)
- Proposed: $0.016-0.052 per interaction (LLM #1 + LLM #2)
- **Increase:** 3-7% (negligible)

### Model Selection

**LLM #1 (Unconscious Processor):**
- **Primary:** GPT-3.5-turbo ($0.0015/1K tokens, 50-150ms, JSON mode)
- **Rationale:** Best balance of speed, cost, and reliability for structured output

**LLM #2 (Conscious Responder):**
- **Primary:** Claude 3.5 Sonnet ($0.003/1K input, 500-2000ms, best reasoning)
- **Alternative:** GPT-4o ($0.005/1K input, similar latency, strong reasoning)
- **Rationale:** High-quality responses require frontier models, cost justified by value

**Fallback Configuration:**
```rust
pub struct DualLlmConfigPresets;

impl DualLlmConfigPresets {
    /// Balanced: Good performance, reasonable cost
    pub fn balanced() -> DualLlmConfig {
        DualLlmConfig {
            llm1_provider: "openai".to_string(),
            llm1_model: "gpt-3.5-turbo".to_string(),
            llm1_timeout_ms: 5000,
            llm1_max_retries: 2,
            llm2_provider: "anthropic".to_string(),
            llm2_model: "claude-3-5-sonnet-20241022".to_string(),
            llm2_timeout_ms: 30000,
            fallback_to_rust: true,
        }
    }

    /// Fast: Minimize latency, higher cost acceptable
    pub fn fast() -> DualLlmConfig {
        DualLlmConfig {
            llm1_provider: "openai".to_string(),
            llm1_model: "gpt-3.5-turbo".to_string(), // Fastest major provider
            llm1_timeout_ms: 3000,
            llm1_max_retries: 1, // Fewer retries for speed
            llm2_provider: "openai".to_string(),
            llm2_model: "gpt-4o".to_string(),
            llm2_timeout_ms: 20000,
            fallback_to_rust: true,
        }
    }

    /// Cheap: Minimize cost, latency less critical
    pub fn cheap() -> DualLlmConfig {
        DualLlmConfig {
            llm1_provider: "anthropic".to_string(),
            llm1_model: "claude-3-haiku-20240307".to_string(), // Cheapest
            llm1_timeout_ms: 8000,
            llm1_max_retries: 2,
            llm2_provider: "anthropic".to_string(),
            llm2_model: "claude-3-5-sonnet-20241022".to_string(),
            llm2_timeout_ms: 30000,
            fallback_to_rust: true,
        }
    }

    /// Reliable: Maximize reliability, fallback always enabled
    pub fn reliable() -> DualLlmConfig {
        DualLlmConfig {
            llm1_provider: "openai".to_string(),
            llm1_model: "gpt-3.5-turbo".to_string(),
            llm1_timeout_ms: 5000,
            llm1_max_retries: 3, // More retries
            llm2_provider: "anthropic".to_string(),
            llm2_model: "claude-3-5-sonnet-20241022".to_string(),
            llm2_timeout_ms: 45000, // Longer timeout
            fallback_to_rust: true, // Always fallback
        }
    }
}
```

## INTEGRATION PLAN

### Code Changes Required

**File: `/api/src/flow_process.rs`**

**Change 1: Add LLM #1 Output Types (after line 64)**
```rust
// LLM #1 Output Schema (after PhenomenologicalQuality, line 64)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llm1Output {
    pub domains: HashMap<String, f64>,
    pub boundaries: HashMap<String, Llm1BoundaryState>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llm1BoundaryState {
    pub permeability: f64,
    pub status: String,
}

impl Llm1Output {
    pub fn validate(&self) -> Result<(), String> {
        // (validation logic from above)
    }

    pub fn to_domain_activations(&self) -> HashMap<String, DomainActivation> {
        // (conversion logic from above)
    }

    pub fn to_boundary_states(&self, base_boundaries: &[BoundaryState]) -> Vec<BoundaryState> {
        // (conversion logic from above)
    }
}
```

**Change 2: Add Dual-LLM Config (after line 64)**
```rust
#[derive(Debug, Clone)]
pub struct DualLlmConfig {
    pub llm1_provider: Box<dyn LlmProvider>,
    pub llm1_timeout_ms: u64,
    pub llm1_max_retries: u8,
    pub fallback_to_rust: bool,
}
```

**Change 3: Replace DomainEmergenceProcessor (line 818-844)**
```rust
/// Stage 1: Domain Emergence (LLM #1 Version)
pub struct Llm1DomainProcessor {
    llm1_client: Box<dyn LlmProvider>,
    config: DualLlmConfig,
    fallback_processor: DomainEmergenceProcessor,
}

impl Llm1DomainProcessor {
    pub fn new(llm1_client: Box<dyn LlmProvider>, config: DualLlmConfig) -> Self {
        Self {
            llm1_client,
            config,
            fallback_processor: DomainEmergenceProcessor,
        }
    }

    fn build_llm1_prompt(&self, user_input: &str, previous_snapshot: &Option<CompactStateSnapshot>) -> String {
        // (prompt template from above)
    }

    async fn call_llm1_with_retry(&self, prompt: &str, max_retries: u8) -> Result<Llm1Output, LlmError> {
        // (retry logic from above)
    }
}

#[async_trait::async_trait]
impl StageProcessor for Llm1DomainProcessor {
    fn name(&self) -> &str {
        "LLM #1 Domain Emergence"
    }

    async fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // (implementation from above)
    }
}
```

**Change 4: Remove BoundaryDissolutionProcessor (line 848-894)**
- DELETE entire `BoundaryDissolutionProcessor` struct
- Boundary calculations now come from LLM #1 output (integrated into Stage 1)

**Change 5: Update FlowProcess to Use LLM #1 (in `FlowProcess::new()` and `execute_stages()`)**
```rust
impl FlowProcess {
    pub fn new_with_llm1(llm1_client: Box<dyn LlmProvider>, config: DualLlmConfig) -> Self {
        let stage1 = Box::new(Llm1DomainProcessor::new(llm1_client, config));
        // Note: Stage 2 (BoundaryDissolutionProcessor) removed, logic in Stage 1 now
        let stage3 = Box::new(InterfaceAttentionProcessor);
        let stage4 = Box::new(QualityEmergenceProcessor);
        let stage5 = Box::new(IntegrationProcessor);

        Self {
            stages: vec![stage1, stage3, stage4, stage5],
        }
    }

    // Keep existing new() for backward compatibility (uses Rust calculations)
    pub fn new() -> Self {
        let stage1 = Box::new(DomainEmergenceProcessor);
        let stage2 = Box::new(BoundaryDissolutionProcessor);
        let stage3 = Box::new(InterfaceAttentionProcessor);
        let stage4 = Box::new(QualityEmergenceProcessor);
        let stage5 = Box::new(IntegrationProcessor);

        Self {
            stages: vec![stage1, stage2, stage3, stage4, stage5],
        }
    }
}
```

**File: `/api/src/lib.rs`**

**Change 6: Add LLM #1 Configuration to VifApi (line 250-260)**
```rust
pub struct VifApi {
    llm1_provider: Option<Box<dyn LlmProvider>>, // LLM #1 for domain calculations
    llm2_provider: Box<dyn LlmProvider>,          // LLM #2 for responses (existing)
    prompt_engine: PromptEngine,
    memory_manager: MemoryManager,
    token_optimizer: TokenOptimizer,
    ajm: AutonomousJudgementModule,
    hlip_integration: HLIPIntegration,
    flow_process: FlowProcess,
    dual_llm_config: Option<DualLlmConfig>,
}

impl VifApi {
    pub async fn new_with_dual_llm(
        llm1_provider: Box<dyn LlmProvider>,
        llm2_provider: Box<dyn LlmProvider>,
        dual_llm_config: DualLlmConfig,
        mut framework_state: FrameworkState,
        database_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // (existing domain registration code)

        let flow_process = FlowProcess::new_with_llm1(llm1_provider.clone(), dual_llm_config.clone());

        Ok(Self {
            llm1_provider: Some(llm1_provider),
            llm2_provider,
            // ... rest of initialization
            flow_process,
            dual_llm_config: Some(dual_llm_config),
        })
    }

    // Keep existing new() for backward compatibility
    pub async fn new(
        provider: Box<dyn LlmProvider>,
        framework_state: FrameworkState,
        database_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Existing implementation (Rust-only Stages 1-2)
    }
}
```

### Backward Compatibility

**Strategy: Dual Constructor Pattern**
- `VifApi::new()`: Existing behavior (Rust Stages 1-2) - NO CHANGES
- `VifApi::new_with_dual_llm()`: New behavior (LLM #1 Stages 1-2)
- All existing tests continue to pass (use `VifApi::new()`)

**Migration Path:**
1. **Phase 1:** Implement dual-LLM, keep both constructors
2. **Phase 2:** Add feature flag `dual-llm` (Cargo.toml)
3. **Phase 3:** Migrate tests to dual-LLM incrementally
4. **Phase 4:** Deprecate `VifApi::new()` (warn in docs)
5. **Phase 5:** Remove Rust-only path (major version bump)

**Feature Flag (Cargo.toml):**
```toml
[features]
default = ["rust-stages"]
rust-stages = []
dual-llm = []
```

**Example Usage:**
```rust
// Rust-only (existing)
let api = VifApi::new(llm_provider, framework_state, "sqlite::memory:").await?;

// Dual-LLM (new)
let llm1 = LlmFactory::create_llm(&LlmConfig {
    api_key: env::var("OPENAI_API_KEY")?,
    provider_name: "openai".to_string(),
    model_name: "gpt-3.5-turbo".to_string(),
})?;
let llm2 = LlmFactory::create_llm(&LlmConfig {
    api_key: env::var("ANTHROPIC_API_KEY")?,
    provider_name: "anthropic".to_string(),
    model_name: "claude-3-5-sonnet-20241022".to_string(),
})?;
let config = DualLlmConfigPresets::balanced();
let api = VifApi::new_with_dual_llm(llm1, llm2, config, framework_state, "sqlite::memory:").await?;
```

### Testing Strategy

**Unit Tests (LLM #1 Isolation):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_llm1_prompt_construction() {
        let processor = Llm1DomainProcessor::new(
            Box::new(MockLlm::echo()),
            DualLlmConfigPresets::balanced(),
        );
        let prompt = processor.build_llm1_prompt("Hello", &None);
        assert!(prompt.contains("<user_input>Hello</user_input>"));
        assert!(prompt.contains("<domains>"));
    }

    #[tokio::test]
    async fn test_llm1_output_validation() {
        let valid_output = Llm1Output {
            domains: hashmap! {
                "CD".to_string() => 0.8,
                "SD".to_string() => 0.6,
                "CuD".to_string() => 0.4,
                "ED".to_string() => 0.5,
            },
            boundaries: hashmap! {
                "CD-SD".to_string() => Llm1BoundaryState {
                    permeability: 0.7,
                    status: "Transitional".to_string(),
                },
                // ... rest of boundaries
            },
            patterns: vec!["test_pattern".to_string()],
        };
        assert!(valid_output.validate().is_ok());

        // Test invalid activation (out of range)
        let invalid_output = Llm1Output {
            domains: hashmap! {
                "CD".to_string() => 1.5, // Invalid: > 1.0
                // ...
            },
            // ...
        };
        assert!(invalid_output.validate().is_err());
    }

    #[tokio::test]
    async fn test_llm1_json_parsing() {
        let json = r#"{
            "domains": {"CD": 0.9, "SD": 0.8, "CuD": 0.5, "ED": 0.4},
            "boundaries": {
                "CD-SD": {"permeability": 0.85, "status": "Transcendent"}
            },
            "patterns": ["pattern1"]
        }"#;
        let output: Llm1Output = serde_json::from_str(json).unwrap();
        assert_eq!(output.domains.get("CD"), Some(&0.9));
    }

    #[tokio::test]
    async fn test_llm1_retry_logic() {
        // Mock LLM that fails twice, succeeds third time
        let mock = MockLlm::new(vec![
            "invalid json".to_string(),
            "invalid json".to_string(),
            r#"{"domains": {...}, "boundaries": {...}, "patterns": []}"#.to_string(),
        ]);
        let processor = Llm1DomainProcessor::new(
            Box::new(mock),
            DualLlmConfig {
                llm1_timeout_ms: 5000,
                llm1_max_retries: 3,
                fallback_to_rust: false,
            },
        );
        // Should succeed on third attempt
        let result = processor.call_llm1_with_retry("test prompt", 3).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_llm1_fallback_to_rust() {
        // Mock LLM that always fails
        let mock = MockErrorLlm::network_error();
        let processor = Llm1DomainProcessor::new(
            Box::new(mock),
            DualLlmConfig {
                fallback_to_rust: true,
                // ...
            },
        );
        let mut context = FlowContext::new(/* ... */);
        // Should fallback to Rust calculations without error
        let result = processor.process(&mut context).await;
        assert!(result.is_ok());
        assert!(!context.domains.is_empty()); // Rust fallback populated domains
    }
}
```

**Integration Tests (Dual-LLM Flow):**
```rust
#[tokio::test]
async fn test_dual_llm_full_flow() {
    // Mock LLM #1 returns valid JSON
    let llm1 = MockLlm::new(vec![
        r#"{
            "domains": {"CD": 0.9, "SD": 0.7, "CuD": 0.5, "ED": 0.4},
            "boundaries": {
                "CD-SD": {"permeability": 0.8, "status": "Transitional"},
                "CD-CuD": {"permeability": 0.7, "status": "Transitional"},
                "CD-ED": {"permeability": 0.6, "status": "Transitional"},
                "SD-CuD": {"permeability": 0.6, "status": "Transitional"},
                "SD-ED": {"permeability": 0.5, "status": "Maintained"},
                "CuD-ED": {"permeability": 0.5, "status": "Maintained"}
            },
            "patterns": ["programming", "rust"]
        }"#.to_string(),
    ]);

    // Mock LLM #2 returns natural response
    let llm2 = MockLlm::new(vec!["Quicksort is...".to_string()]);

    let api = VifApi::new_with_dual_llm(
        Box::new(llm1),
        Box::new(llm2),
        DualLlmConfigPresets::balanced(),
        framework_state,
        "sqlite::memory:",
    ).await.unwrap();

    let response = api.process("How do I implement quicksort?", 0.8).await.unwrap();
    assert!(response.contains("Quicksort"));
    assert_eq!(llm1.call_count(), 1); // LLM #1 called once
    assert_eq!(llm2.call_count(), 1); // LLM #2 called once
}

#[tokio::test]
async fn test_dual_llm_vs_rust_stages_equivalence() {
    // Test that LLM #1 and Rust stages produce similar results for same input
    let input = "Explain quantum entanglement";

    // Run with Rust stages
    let api_rust = VifApi::new(llm_provider.clone(), framework_state.clone(), "sqlite::memory:").await.unwrap();
    let response_rust = api_rust.process(input, 0.8).await.unwrap();

    // Run with LLM #1
    let api_llm = VifApi::new_with_dual_llm(
        llm1_provider,
        llm_provider,
        config,
        framework_state,
        "sqlite::memory:",
    ).await.unwrap();
    let response_llm = api_llm.process(input, 0.8).await.unwrap();

    // Both should produce valid responses (exact match not expected, LLM #1 adds intelligence)
    assert!(!response_rust.is_empty());
    assert!(!response_llm.is_empty());
}
```

**Mocking Strategy:**
```rust
// Extend MockLlm to support structured JSON responses
impl MockLlm {
    pub fn with_json_responses(jsons: Vec<&str>) -> Self {
        Self::new(jsons.iter().map(|s| s.to_string()).collect())
    }

    pub fn llm1_valid() -> Self {
        // Returns valid Llm1Output JSON
        Self::with_json_responses(vec![
            r#"{
                "domains": {"CD": 0.8, "SD": 0.6, "CuD": 0.5, "ED": 0.4},
                "boundaries": {
                    "CD-SD": {"permeability": 0.7, "status": "Transitional"},
                    "CD-CuD": {"permeability": 0.6, "status": "Transitional"},
                    "CD-ED": {"permeability": 0.5, "status": "Maintained"},
                    "SD-CuD": {"permeability": 0.5, "status": "Maintained"},
                    "SD-ED": {"permeability": 0.4, "status": "Maintained"},
                    "CuD-ED": {"permeability": 0.4, "status": "Maintained"}
                },
                "patterns": []
            }"#,
        ])
    }

    pub fn llm1_invalid_json() -> Self {
        Self::new(vec!["This is not valid JSON".to_string()])
    }

    pub fn llm1_invalid_schema() -> Self {
        // Valid JSON but missing required fields
        Self::with_json_responses(vec![
            r#"{"domains": {"CD": 0.8}, "boundaries": {}}"#,
        ])
    }
}
```

## PHASED IMPLEMENTATION

### Phase 1: Core Dual-LLM Implementation [5-7 days]

**Goal:** Replace Rust Stages 1-2 with LLM #1, maintain backward compatibility

**Day 1: Setup & Scaffolding [1 day]**
- Add `Llm1Output`, `Llm1BoundaryState` types to `flow_process.rs`
- Add `DualLlmConfig` struct
- Write unit tests for JSON parsing and validation
- **Deliverable:** Types compile, validation tests pass

**Day 2: LLM #1 Prompt Engineering [1 day]**
- Implement `build_llm1_prompt()` with template
- Write few-shot examples (3 diverse cases)
- Test prompt construction (unit tests)
- **Deliverable:** Prompt template complete, tests pass

**Day 3: LLM #1 Processor Implementation [1.5 days]**
- Implement `Llm1DomainProcessor` struct
- Implement `call_llm1_with_retry()` with timeout/retry logic
- Implement `process()` with validation + fallback
- Write unit tests (retry logic, fallback, error handling)
- **Deliverable:** LLM #1 processor complete, unit tests pass

**Day 4-5: Integration with FlowProcess [1.5 days]**
- Add `FlowProcess::new_with_llm1()` constructor
- Update `VifApi` with `new_with_dual_llm()` constructor
- Remove Stage 2 (BoundaryDissolutionProcessor) from dual-LLM path
- Write integration tests (dual-LLM flow end-to-end)
- **Deliverable:** Dual-LLM flow works, integration tests pass

**Day 6: Backward Compatibility Testing [1 day]**
- Verify existing tests still pass (Rust-only path)
- Add feature flag (`dual-llm`)
- Document migration path
- **Deliverable:** All 84 existing tests pass, dual-LLM tests added

**Day 7: Code Review & Bug Fixes [1 day]**
- Address Clippy warnings
- Fix edge cases discovered in testing
- Update documentation
- **Deliverable:** Phase 1 complete, ready for Phase 2

**Success Criteria:**
- [ ] `Llm1Output` types compile and validate correctly
- [ ] LLM #1 prompt template generates valid output (manual testing)
- [ ] Retry logic handles failures (timeout, invalid JSON, schema errors)
- [ ] Fallback to Rust calculations works
- [ ] Dual-LLM flow produces valid responses
- [ ] All existing tests pass (backward compatibility)
- [ ] New tests: +15 tests (unit + integration), target 99/99 passing

### Phase 2: Optimization & Model Selection [3-4 days]

**Goal:** Optimize latency, cost, and reliability

**Day 8: Model Evaluation [1 day]**
- Test GPT-3.5-turbo, Claude Haiku, GPT-4o-mini
- Measure latency (100 calls each)
- Measure accuracy (JSON validity rate, schema compliance)
- Measure cost (token usage)
- **Deliverable:** Model comparison report, recommendation

**Day 9: Performance Benchmarking [1 day]**
- Implement `test_performance_dual_llm_vs_rust_stages`
- Measure latency increase (P50, P95, P99)
- Measure token cost per interaction
- Compare to Day 9 baseline (<1ms Rust)
- **Deliverable:** Performance report, latency/cost metrics

**Day 10: Caching Implementation [1 day]**
- Add LRU cache for LLM #1 results (key: user input hash)
- Implement cache eviction policy (max 1000 entries, 1 hour TTL)
- Test cache hit rate (expect 10-20% for repeated queries)
- **Deliverable:** Caching works, cache hit tests pass

**Day 11: Config Presets & Tuning [1 day]**
- Implement `DualLlmConfigPresets` (balanced, fast, cheap, reliable)
- Test each preset (latency, cost, reliability)
- Document trade-offs
- **Deliverable:** Presets work, documented

**Success Criteria:**
- [ ] Model recommendation documented (likely GPT-3.5-turbo)
- [ ] Performance benchmarks show <20% latency increase
- [ ] Caching reduces latency by 10-20% on cache hits
- [ ] Config presets cover common use cases

### Phase 3: Production Hardening [2-3 days]

**Goal:** Make dual-LLM production-ready (error handling, observability, docs)

**Day 12: Error Handling & Logging [1 day]**
- Add structured logging (log LLM #1 inputs/outputs)
- Implement circuit breaker pattern (disable LLM #1 after 5 consecutive failures)
- Add metrics (LLM #1 call count, failure rate, fallback rate)
- Test failure scenarios (auth error, rate limit, timeout)
- **Deliverable:** Robust error handling, observability

**Day 13: Documentation [1 day]**
- Update `ARCHITECTURE.md` with dual-LLM section
- Write migration guide (Rust-only → Dual-LLM)
- Document config options (presets, custom)
- Add troubleshooting guide (LLM #1 failures, debugging)
- **Deliverable:** Complete documentation

**Day 14: Final Testing & Validation [1 day]**
- Run full test suite (expect 110+ tests, all passing)
- Perform manual testing (diverse inputs, edge cases)
- Load testing (100 concurrent requests)
- Validate fallback scenarios (LLM #1 disabled)
- **Deliverable:** Production-ready dual-LLM implementation

**Success Criteria:**
- [ ] Circuit breaker prevents cascading failures
- [ ] Logging provides visibility into LLM #1 decisions
- [ ] Documentation complete (architecture, migration, troubleshooting)
- [ ] All tests pass (110+ tests, 75%+ coverage maintained)
- [ ] Load testing shows stable performance

## DEPENDENCIES ON OTHERS

| Needs_From | What | Why | Blocking |
|------------|------|-----|----------|
| **Product Owner** | LLM #1 API keys (OpenAI, Anthropic) | Testing with real models | Phase 1 Day 2 onwards |
| **DevOps** | Environment variable setup (API keys) | Production deployment | Phase 3 Day 14 |
| **QA** | Manual testing of LLM #1 outputs | Validate domain calculations make sense | Phase 2 Day 8 |
| **Documentation Team** | Review migration guide | Ensure clarity for users | Phase 3 Day 13 |
| **None** | N/A (self-contained implementation) | N/A | N/A |

## SUCCESS CRITERIA

**Phase 1 (Core Implementation):**
- [ ] Dual-LLM flow produces valid responses (integration test passes)
- [ ] LLM #1 output validated (JSON schema, range checks)
- [ ] Fallback to Rust works (test with mock LLM #1 failure)
- [ ] Backward compatibility maintained (all 84 existing tests pass)
- [ ] Code quality (Clippy clean, no warnings)

**Phase 2 (Optimization):**
- [ ] Latency increase <20% vs Rust-only (P95 metric)
- [ ] Token cost <$0.002 per interaction (LLM #1 only)
- [ ] Caching reduces latency by 10-20% (cache hit scenarios)
- [ ] Model recommendation documented (GPT-3.5-turbo or Claude Haiku)

**Phase 3 (Production Hardening):**
- [ ] Circuit breaker prevents cascading failures (test with 10 consecutive failures)
- [ ] Logging provides visibility (can debug LLM #1 decisions from logs)
- [ ] Documentation complete (architecture, migration, troubleshooting)
- [ ] Load testing stable (100 concurrent requests, no crashes)
- [ ] Deployment guide ready (environment variables, config)

**Overall System Success:**
- [ ] Dual-LLM architecture matches specification (`dual-llm-architecture.md`)
- [ ] LLM #1 calculations more intelligent than Rust (qualitative assessment by QA)
- [ ] System remains reliable (fallback ensures no user-facing failures)
- [ ] Cost increase justified by intelligence gain (<10% cost increase for contextual domain activation)

---

**Next Steps:**
1. Review this design with team (COMP: implementation feasibility, SCI: performance assumptions, CULT: LLM best practices, EXP: developer ergonomics)
2. Get approval on model selection (GPT-3.5-turbo for LLM #1?)
3. Obtain API keys for testing
4. Begin Phase 1 Day 1 (setup & scaffolding)

**Open Questions for Review:**
1. Should LLM #1 also calculate oscillatory parameters (frequency, amplitude, phase) or keep Rust defaults?
2. Should we support multiple LLM #1 models simultaneously (e.g., OpenAI fallback to Anthropic)?
3. Should caching be opt-in or opt-out?
4. Should we expose LLM #1 state to users (for debugging/transparency)?
