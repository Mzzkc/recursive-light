# SOFTWARE ARCHITECTURE - INTEGRATION PLAN
Agent: Software Architecture Expert | Date: 2025-11-01 | Status: Complete

## EXEC SUMMARY

**Approach:** Add dual-LLM as opt-in feature alongside existing Rust calculators using feature flag pattern. Preserve all 87 passing tests through backward compatibility layer. Enable phased rollout with zero breaking changes.

**Risk Mitigation:**
- Feature flag (`DUAL_LLM_MODE`) defaults to `false` (existing behavior)
- Existing processors remain functional (no deletion)
- Mock-based testing for dual-LLM prevents expensive API calls
- Memory tiering is backward-compatible with existing snapshots
- Gradual migration allows validation at each phase

**Timeline:** 24-32 hours across 4 phases
- Phase 1: 8 hours (infrastructure, no behavior change)
- Phase 2: 10 hours (dual-LLM implementation + tests)
- Phase 3: 6 hours (validation + toggle default)
- Phase 4: 8 hours (cleanup, optional deprecation)

---

## TDF ANALYSIS

### COMP (Computational Domain)
**Code Organization:**
- Create `src/dual_llm/` module for new architecture
- Separate concerns: `UnconscciousLlmProcessor` (Stage 1-2), memory tiering, prompts
- Preserve existing processors in `flow_process.rs` for fallback
- Add configuration abstraction (`DualLlmConfig`) to centralize settings

**Refactoring Strategy:**
- NO modification to existing 7-stage trait (`StageProcessor`)
- Add new processors that implement same trait (drop-in replacement)
- Configuration determines which processors to use at runtime
- Clean separation between "classic mode" and "dual-LLM mode"

### SCI (Scientific Domain)
**Impact Analysis:**
- 87 tests currently passing with Rust calculators
- Dual-LLM changes Stages 1-2 only (domain emergence + boundary dissolution)
- Stages 3-7 remain identical (interface attention → evolution)
- LLM #1 output must match existing `FlowContext` structure

**Regression Risk:**
- LOW: Feature flag prevents accidental activation
- MEDIUM: LLM #1 output validation crucial (schema mismatch = runtime error)
- LOW: Memory system already supports arbitrary metadata
- MITIGATION: Comprehensive integration tests + schema validation

### CULT (Cultural Domain)
**Rust Best Practices:**
- Use cargo features for compile-time toggling (`default = []`, `dual-llm = []`)
- Leverage trait objects for runtime polymorphism (existing pattern)
- Error handling via `Result<T, FlowError>` (existing pattern)
- Async/await for LLM calls (already used in `LlmProvider`)

**Maintainability:**
- Clear module boundaries prevent code sprawl
- Configuration-driven behavior reduces conditionals
- Comprehensive documentation in module headers
- Separate test files for dual-LLM (`tests/dual_llm_integration.rs`)

### EXP (Experiential Domain)
**Developer Experience:**
- Environment variable toggle: `DUAL_LLM_MODE=true cargo test`
- No code changes required for existing workflows
- New developers see classic mode by default (simpler onboarding)
- Advanced users opt-in to dual-LLM explicitly

**Migration Path:**
- Phase 1: Add infrastructure (no visible change)
- Phase 2: Test dual-LLM in isolated tests
- Phase 3: Validate parity, flip default
- Phase 4: Deprecate Rust calculators (optional, keep for research)

---

## REFACTORING PLAN

### Files to CREATE

#### 1. `/home/emzi/Projects/recursive-light/api/src/dual_llm/mod.rs`
```rust
//! Dual-LLM Architecture Module
//!
//! Implements two-LLM system:
//! - LLM #1 (Unconscious): Domain emergence + boundary states
//! - LLM #2 (Conscious): Integration + response generation
//!
//! Configuration via environment variables:
//! - DUAL_LLM_MODE=true|false (default: false)
//! - UNCONSCIOUS_LLM_MODEL=model-name (default: claude-3-5-haiku)
//! - CONSCIOUS_LLM_MODEL=model-name (default: claude-3-5-sonnet)

pub mod config;
pub mod processors;
pub mod prompts;
pub mod memory_tiering;
```

#### 2. `/home/emzi/Projects/recursive-light/api/src/dual_llm/config.rs`
```rust
use serde::{Deserialize, Serialize};
use crate::llm_error::LlmError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualLlmConfig {
    pub enabled: bool,
    pub unconscious_model: String,
    pub conscious_model: String,
    pub memory_hot_turns: usize,    // Default: 5
    pub memory_warm_turns: usize,   // Default: 20
    pub fallback_on_error: bool,    // Default: true (use Rust calculators)
}

impl DualLlmConfig {
    pub fn from_env() -> Result<Self, LlmError> {
        // Load from environment variables
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
```

#### 3. `/home/emzi/Projects/recursive-light/api/src/dual_llm/processors.rs`
```rust
use crate::flow_process::{StageProcessor, FlowContext, FlowError};
use crate::LlmProvider;
use super::prompts::UnconscciousPromptBuilder;

/// Replaces Stages 1-2 (DomainEmergenceProcessor + BoundaryDissolutionProcessor)
/// when dual-LLM mode is enabled
pub struct UnconscciousLlmProcessor {
    llm: Box<dyn LlmProvider>,
    config: DualLlmConfig,
}

impl StageProcessor for UnconscciousLlmProcessor {
    fn name(&self) -> &str {
        "Unconsccious LLM (Stages 1-2)"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // 1. Load memory from previous snapshots
        // 2. Build prompt with user input + memory context
        // 3. Call LLM #1 (async)
        // 4. Parse response into domain activations + boundary states
        // 5. Populate context.domains and context.boundaries
        // 6. Handle errors (fallback to Rust calculators if enabled)
    }
}
```

#### 4. `/home/emzi/Projects/recursive-light/api/src/dual_llm/prompts.rs`
```rust
use crate::flow_process::FlowContext;
use crate::memory::CompactStateSnapshot;

pub struct UnconscciousPromptBuilder;

impl UnconscciousPromptBuilder {
    pub fn build(
        user_input: &str,
        memory_hot: Vec<CompactStateSnapshot>,
        memory_warm: Vec<CompactStateSnapshot>,
    ) -> String {
        // Construct structured prompt for LLM #1
        // Include:
        // - User input
        // - Memory tier summaries (hot: full detail, warm: compressed)
        // - Task: Output domain activations + boundary states as JSON
    }
}
```

#### 5. `/home/emzi/Projects/recursive-light/api/src/dual_llm/memory_tiering.rs`
```rust
use crate::memory::{MemoryManager, CompactStateSnapshot};
use sqlx::types::Uuid;

pub struct MemoryTierLoader {
    memory_manager: MemoryManager,
}

impl MemoryTierLoader {
    pub async fn load_tiered_memory(
        &self,
        user_id: Uuid,
        hot_turns: usize,
        warm_turns: usize,
    ) -> Result<(Vec<CompactStateSnapshot>, Vec<CompactStateSnapshot>), sqlx::Error> {
        // Load last N snapshots from database
        // Split into hot (recent) and warm (older) tiers
    }
}
```

#### 6. `/home/emzi/Projects/recursive-light/api/tests/dual_llm_integration.rs`
```rust
//! Integration tests for dual-LLM architecture
//! Tests both classic mode and dual-LLM mode

#[tokio::test]
async fn test_dual_llm_stage_1_2_replacement() {
    // Verify UnconscciousLlmProcessor produces same context as classic processors
}

#[tokio::test]
async fn test_dual_llm_memory_tiering() {
    // Verify memory loading and tiering logic
}

#[tokio::test]
async fn test_dual_llm_fallback_on_error() {
    // Verify fallback to Rust calculators when LLM #1 fails
}
```

### Files to MODIFY

#### 1. `/home/emzi/Projects/recursive-light/api/src/flow_process.rs`
**Change:** Add conditional processor selection in `FlowProcess::new()`

```rust
impl FlowProcess {
    pub fn new() -> Self {
        Self::with_config(DualLlmConfig::from_env().unwrap_or_default())
    }

    pub fn with_config(config: DualLlmConfig) -> Self {
        let stages: Vec<Box<dyn StageProcessor>> = if config.is_enabled() {
            // Dual-LLM mode: Replace Stages 1-2 with single UnconscciousLlmProcessor
            vec![
                Box::new(UnconscciousLlmProcessor::new(config.clone())),
                Box::new(InterfaceAttentionProcessor),
                Box::new(QualityEmergenceProcessor),
                Box::new(IntegrationProcessor),
                Box::new(ContinuityProcessor),
                Box::new(EvolutionProcessor),
            ]
        } else {
            // Classic mode: Keep existing 7-stage pipeline
            vec![
                Box::new(DomainEmergenceProcessor),
                Box::new(BoundaryDissolutionProcessor),
                Box::new(InterfaceAttentionProcessor),
                Box::new(QualityEmergenceProcessor),
                Box::new(IntegrationProcessor),
                Box::new(ContinuityProcessor),
                Box::new(EvolutionProcessor),
            ]
        };
        Self { stages }
    }
}
```

**Impact:** ZERO for existing tests (default config uses classic mode)

#### 2. `/home/emzi/Projects/recursive-light/api/src/memory.rs`
**Change:** Add method to load N recent snapshots (currently only loads latest)

```rust
impl MemoryManager {
    /// Load N most recent snapshots for memory tiering
    pub async fn get_recent_snapshots(
        &self,
        user_id: Uuid,
        limit: usize,
    ) -> Result<Vec<CompactStateSnapshot>, sqlx::Error> {
        // Query: ORDER BY timestamp DESC LIMIT ?
        // Returns vector of snapshots (newest first)
    }
}
```

**Impact:** New method, no changes to existing methods

#### 3. `/home/emzi/Projects/recursive-light/api/src/lib.rs`
**Change:** Add `pub mod dual_llm;` export

```rust
pub mod domains;
mod flow_process;
pub mod llm_error;
mod memory;
pub mod mock_llm;
pub mod prompt_engine;
pub mod dual_llm;  // NEW
```

**Impact:** Exposes new module, no breaking changes

#### 4. `/home/emzi/Projects/recursive-light/api/.env.example`
**Change:** Add dual-LLM configuration variables

```bash
# =============================================================================
# DUAL-LLM ARCHITECTURE (Experimental)
# =============================================================================
# Enable dual-LLM mode (default: false)
DUAL_LLM_MODE=false

# LLM #1 (Unconscious): Domain emergence + boundary states
UNCONSCIOUS_LLM_MODEL=anthropic/claude-3-5-haiku

# LLM #2 (Conscious): Integration + response (uses DEFAULT_MODEL)
CONSCIOUS_LLM_MODEL=anthropic/claude-3-5-sonnet

# Memory tiering configuration
MEMORY_HOT_TURNS=5   # Full detail for last 5 turns
MEMORY_WARM_TURNS=20 # Compressed detail for turns 6-25

# Fallback to Rust calculators if LLM #1 fails (default: true)
DUAL_LLM_FALLBACK=true
```

**Impact:** Documentation only, no code changes

#### 5. `/home/emzi/Projects/recursive-light/api/Cargo.toml`
**Change:** Add optional feature flag (optional, can use env var instead)

```toml
[features]
default = []
dual-llm = []  # Enable at compile time: cargo build --features dual-llm
```

**Impact:** Optional compile-time optimization (can skip this in Phase 1)

### Files to DELETE
**NONE** (all existing code preserved for backward compatibility)

---

## CODE ORGANIZATION

### Module Structure
```
api/src/
├── dual_llm/
│   ├── mod.rs              # Module root, public exports
│   ├── config.rs           # DualLlmConfig, environment loading
│   ├── processors.rs       # UnconscciousLlmProcessor
│   ├── prompts.rs          # Prompt templates for LLM #1
│   └── memory_tiering.rs   # Memory tier loading logic
├── flow_process.rs         # MODIFY: Conditional processor selection
├── memory.rs               # MODIFY: Add get_recent_snapshots()
├── lib.rs                  # MODIFY: Export dual_llm module
└── [existing files...]     # NO CHANGES

tests/
├── dual_llm_integration.rs # NEW: Dual-LLM tests
└── [existing tests...]     # NO CHANGES (87 tests preserved)
```

### Separation of Concerns

**Configuration Layer** (`dual_llm/config.rs`)
- Load environment variables
- Validate configuration
- Provide defaults
- No business logic

**Processor Layer** (`dual_llm/processors.rs`)
- Implement `StageProcessor` trait
- Orchestrate LLM calls + memory loading
- Handle errors and fallback
- Parse LLM responses into `FlowContext`

**Prompt Layer** (`dual_llm/prompts.rs`)
- Construct prompts for LLM #1
- Separate templates from logic
- Testable in isolation

**Memory Layer** (`dual_llm/memory_tiering.rs`)
- Load snapshots from database
- Split into hot/warm tiers
- Compress warm tier (future optimization)
- No awareness of prompts or LLMs

**Integration Point** (`flow_process.rs`)
- Single conditional: `if config.is_enabled()`
- Select processor set
- Zero changes to trait or context types

---

## CONFIGURATION STRATEGY

### Feature Flags
**Primary Toggle:** Environment variable `DUAL_LLM_MODE=true|false`
- Default: `false` (classic mode)
- Per-test override: `#[test] fn test_with_dual_llm() { std::env::set_var(...) }`
- Runtime toggle (no recompilation)

**Optional Compile-Time Flag:** Cargo feature `dual-llm`
- Allows skipping dual-LLM code in production builds if unused
- Phase 4 optimization (not required initially)

### Configuration Files
**Environment Variables** (`.env` file)
```bash
# Core toggle
DUAL_LLM_MODE=false

# Model selection
UNCONSCIOUS_LLM_MODEL=anthropic/claude-3-5-haiku
CONSCIOUS_LLM_MODEL=anthropic/claude-3-5-sonnet

# Memory configuration
MEMORY_HOT_TURNS=5
MEMORY_WARM_TURNS=20

# Error handling
DUAL_LLM_FALLBACK=true
```

**Loading Strategy:**
```rust
impl DualLlmConfig {
    pub fn from_env() -> Result<Self, LlmError> {
        dotenv::dotenv().ok();
        Ok(Self {
            enabled: std::env::var("DUAL_LLM_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            unconscious_model: std::env::var("UNCONSCIOUS_LLM_MODEL")
                .unwrap_or_else(|_| "anthropic/claude-3-5-haiku".to_string()),
            // ... etc
        })
    }
}
```

### Defaults
- `DUAL_LLM_MODE`: `false` (preserves existing behavior)
- `UNCONSCIOUS_LLM_MODEL`: `"anthropic/claude-3-5-haiku"` (fast, cheap)
- `CONSCIOUS_LLM_MODEL`: `"anthropic/claude-3-5-sonnet"` (smart, expensive)
- `MEMORY_HOT_TURNS`: `5` (last 5 interactions in full detail)
- `MEMORY_WARM_TURNS`: `20` (interactions 6-25 in compressed form)
- `DUAL_LLM_FALLBACK`: `true` (use Rust calculators if LLM #1 fails)

---

## BACKWARD COMPATIBILITY

### How to Maintain Existing Behavior

**1. Default Configuration Preserves Classic Mode**
```rust
impl Default for DualLlmConfig {
    fn default() -> Self {
        Self {
            enabled: false,  // KEY: Classic mode by default
            // ... other fields
        }
    }
}
```

**2. Processor Selection is Conditional**
```rust
impl FlowProcess {
    pub fn new() -> Self {
        let config = DualLlmConfig::from_env().unwrap_or_default();
        // If config.enabled == false, uses existing 7-stage pipeline
        // If config.enabled == true, uses 6-stage dual-LLM pipeline
        Self::with_config(config)
    }
}
```

**3. Existing Processors Remain Unchanged**
- `DomainEmergenceProcessor` code stays identical
- `BoundaryDissolutionProcessor` code stays identical
- Stages 3-7 completely unaffected
- 87 existing tests pass without modification

**4. Memory System Backward Compatible**
```rust
// NEW method (doesn't affect existing get_latest_snapshot)
pub async fn get_recent_snapshots(&self, user_id: Uuid, limit: usize)
    -> Result<Vec<CompactStateSnapshot>, sqlx::Error>

// EXISTING method (unchanged)
pub async fn get_latest_snapshot(&self, user_id: Uuid)
    -> Result<Option<CompactStateSnapshot>, sqlx::Error>
```

**5. Test Compatibility**
```rust
// Existing tests use FlowProcess::new() → default config → classic mode
#[test]
fn test_domain_emergence_processor() {
    let processor = DomainEmergenceProcessor;  // Still exists
    // Test passes unchanged
}

// New tests explicitly enable dual-LLM mode
#[tokio::test]
async fn test_dual_llm_mode() {
    std::env::set_var("DUAL_LLM_MODE", "true");
    let config = DualLlmConfig::from_env().unwrap();
    let flow = FlowProcess::with_config(config);
    // Test dual-LLM specific behavior
}
```

### Fallback Mechanisms

**LLM #1 Failure Handling:**
```rust
impl UnconscciousLlmProcessor {
    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        match self.call_unconscious_llm(context).await {
            Ok(llm_output) => {
                // Parse LLM response into context
                context.domains = llm_output.domains;
                context.boundaries = llm_output.boundaries;
                Ok(())
            }
            Err(e) if self.config.fallback_on_error => {
                // FALLBACK: Use Rust calculators
                eprintln!("LLM #1 failed, falling back to Rust calculators: {}", e);
                DomainEmergenceProcessor.process(context)?;
                BoundaryDissolutionProcessor.process(context)?;
                Ok(())
            }
            Err(e) => Err(FlowError::StageProcessingFailed {
                stage: "Unconsccious LLM".to_string(),
                reason: e.to_string(),
            })
        }
    }
}
```

**Response Validation:**
```rust
// Ensure LLM #1 output matches expected schema
#[derive(Deserialize)]
struct UnconscciousLlmOutput {
    domains: HashMap<String, f64>,  // domain_name → activation
    boundaries: Vec<BoundaryOutput>, // boundary states
}

impl UnconscciousLlmOutput {
    fn validate(&self) -> Result<(), FlowError> {
        // Check all activations are [0.0, 1.0]
        // Check boundary names are valid (domain-domain format)
        // Return error if schema violated
    }
}
```

---

## TESTING STRATEGY

### How to Test Without Breaking 87 Tests

**1. Preserve Existing Tests (0 modifications)**
- All 87 tests use `FlowProcess::new()` → default config → classic mode
- Tests in `src/flow_process.rs`, `src/memory.rs`, etc. run unchanged
- CI/CD continues passing without changes

**2. Add Isolated Dual-LLM Tests**
```rust
// tests/dual_llm_integration.rs
#[tokio::test]
async fn test_dual_llm_with_mock() {
    // Use MockLlm for LLM #1 (no API costs)
    let mock = MockLlm::new(vec![
        r#"{"domains": {"CD": 0.8, "SD": 0.7}, "boundaries": [...]}"#.to_string()
    ]);

    let config = DualLlmConfig {
        enabled: true,
        // ... configure to use mock
    };

    let flow = FlowProcess::with_config(config);
    let context = FlowContext::new(...);
    let result = flow.execute(context).await;

    assert!(result.is_ok());
    // Verify domains and boundaries populated by LLM #1
}
```

**3. Mock Strategy for LLM #1**
```rust
// MockUnconscciousLlm returns deterministic JSON responses
pub struct MockUnconscciousLlm {
    responses: Vec<String>,  // Pre-defined JSON outputs
}

impl LlmProvider for MockUnconscciousLlm {
    async fn send_request(&self, _prompt: &str) -> Result<String, LlmError> {
        Ok(self.responses[0].clone())  // Return canned response
    }
}

// Tests create mock with expected output
let mock = MockUnconscciousLlm {
    responses: vec![
        r#"{
            "domains": {"CD": 0.9, "SD": 0.8, "CuD": 0.6, "ED": 0.5},
            "boundaries": [
                {"name": "CD-SD", "permeability": 0.85, "status": "Transcendent"},
                {"name": "SD-CuD", "permeability": 0.7, "status": "Transitional"}
            ]
        }"#.to_string()
    ]
};
```

**4. Test Categories**

**Unit Tests** (in `src/dual_llm/processors.rs`)
- Test prompt building logic
- Test LLM response parsing
- Test validation logic
- Test fallback mechanism

**Integration Tests** (in `tests/dual_llm_integration.rs`)
- Test full dual-LLM flow with mock
- Test memory tiering
- Test parity with classic mode (same input → similar output)
- Test error handling (LLM #1 fails, invalid response, etc.)

**Regression Tests** (new file `tests/dual_llm_regression.rs`)
```rust
#[tokio::test]
async fn test_classic_mode_unchanged() {
    // Run same test in classic mode and verify it still passes
    let config = DualLlmConfig::default();  // enabled = false
    assert!(!config.is_enabled());

    let flow = FlowProcess::with_config(config);
    // ... verify behavior matches pre-dual-LLM behavior
}
```

**5. CI/CD Considerations**
```yaml
# .github/workflows/test.yml
- name: Run classic tests
  run: cargo test --lib
  env:
    DUAL_LLM_MODE: false  # Explicit classic mode

- name: Run dual-LLM tests (mocked)
  run: cargo test --test dual_llm_integration
  env:
    DUAL_LLM_MODE: true
    USE_MOCK_LLM: true  # No API costs in CI
```

**6. Do We Need 87 New Tests?**
**NO.** Here's the strategy:

- **Existing 87 tests:** Run in classic mode (unchanged)
- **New tests (~20-30):** Test dual-LLM specific functionality
  - Prompt construction (5 tests)
  - Response parsing (5 tests)
  - Memory tiering (5 tests)
  - Error handling/fallback (5 tests)
  - Integration parity (5 tests)
  - End-to-end dual-LLM flow (5 tests)

**Total test count:** ~110-120 tests (87 existing + 25 new)

---

## MIGRATION PATH

### Phase 1: Add Dual-LLM Infrastructure (Disabled) [8 hours]

**Goal:** Create all new code with feature flag OFF. Zero behavior change.

**Tasks:**
1. Create `src/dual_llm/` module structure (1 hour)
   - `mod.rs`, `config.rs`, `processors.rs`, `prompts.rs`, `memory_tiering.rs`
   - All files compile but are unused

2. Implement `DualLlmConfig` with environment loading (1 hour)
   - `from_env()` method
   - Default to `enabled: false`

3. Add `get_recent_snapshots()` to `MemoryManager` (2 hours)
   - SQL query: `ORDER BY timestamp DESC LIMIT ?`
   - Tests for new method

4. Modify `FlowProcess::new()` to support configuration (2 hours)
   - Add `with_config()` method
   - Conditional processor selection
   - Tests verify classic mode still works

5. Add environment variables to `.env.example` (0.5 hours)
   - Document all dual-LLM settings

6. CI/CD validation (1.5 hours)
   - Run full test suite
   - Verify 87 tests still pass
   - Verify no performance regression

**Validation:**
- `cargo test --lib` → 87 tests pass
- `DUAL_LLM_MODE=false cargo test` → 87 tests pass
- `DUAL_LLM_MODE=true cargo test` → Should fail gracefully (not implemented yet)

**Blocking:** NONE (can proceed to Phase 2)

---

### Phase 2: Implement Dual-LLM (Testing Only) [10 hours]

**Goal:** Fully implement dual-LLM but only enable in new tests. Classic mode remains default.

**Tasks:**
1. Implement `UnconscciousLlmProcessor` (4 hours)
   - LLM #1 call logic
   - Response parsing
   - Error handling + fallback
   - Unit tests with mocks

2. Implement `UnconscciousPromptBuilder` (2 hours)
   - Prompt template for LLM #1
   - Memory tier formatting
   - Tests for prompt construction

3. Implement `MemoryTierLoader` (2 hours)
   - Load hot + warm memory tiers
   - Integration with `MemoryManager`
   - Tests for tiering logic

4. Create integration tests (`tests/dual_llm_integration.rs`) (2 hours)
   - Test full dual-LLM flow with mock
   - Test memory tiering
   - Test fallback mechanism
   - Test parity with classic mode

**Validation:**
- `cargo test --lib` → 87 tests pass (classic mode)
- `cargo test --test dual_llm_integration` → New tests pass (dual-LLM mode with mocks)
- `DUAL_LLM_MODE=true cargo test` → Mix of classic + dual-LLM tests pass

**Blocking:** Need System Architect's LLM #1 prompt specification

**Dependencies:**
| Needs_From | What | Why | Blocking |
|------------|------|-----|----------|
| System Architect | LLM #1 prompt template | Define structure for unconscious processing | Yes |
| System Architect | LLM #1 output schema (JSON) | Define parsing logic | Yes |
| Memory Expert | Memory tier compression algorithm | Optimize warm tier storage | No (can use full snapshots initially) |

---

### Phase 3: Enable Dual-LLM by Default [6 hours]

**Goal:** Flip default to dual-LLM mode after validation. Keep classic mode available.

**Tasks:**
1. Run extensive validation (2 hours)
   - 100 test conversations with dual-LLM vs classic mode
   - Compare outputs (domain activations, boundary states)
   - Measure performance (latency, API costs)
   - Identify regressions

2. Update default configuration (0.5 hours)
   ```rust
   impl Default for DualLlmConfig {
       fn default() -> Self {
           Self {
               enabled: true,  // CHANGED: Now default
               // ...
           }
       }
   }
   ```

3. Update documentation (1 hour)
   - Update README: "Now uses dual-LLM by default"
   - Add migration guide for users who want classic mode
   - Document configuration options

4. CI/CD updates (1 hour)
   - Default tests now run dual-LLM mode
   - Add explicit classic-mode test suite
   - Update deployment config

5. Smoke testing (1.5 hours)
   - Run all 87 existing tests in dual-LLM mode
   - Fix any edge cases discovered
   - Verify performance acceptable

**Validation:**
- `cargo test` → All tests pass in dual-LLM mode
- `DUAL_LLM_MODE=false cargo test` → Classic mode still works
- Performance benchmarks within acceptable range (< 2x latency increase)

**Rollback Plan:**
- Revert default to `enabled: false`
- Keep dual-LLM code in place for future retry
- Document issues discovered

---

### Phase 4: Cleanup and Deprecation (Optional) [8 hours]

**Goal:** Optionally remove Rust calculators if dual-LLM proves superior.

**Tasks:**
1. Deprecate Rust calculators (2 hours)
   - Add `#[deprecated]` attributes to `DomainEmergenceProcessor`, `BoundaryDissolutionProcessor`
   - Update docs: "Use dual-LLM mode instead"
   - Keep code for research/fallback

2. Add compile-time feature flag (1 hour)
   ```toml
   [features]
   default = ["dual-llm"]
   dual-llm = []
   classic-mode = []
   ```

3. Performance optimization (3 hours)
   - Implement warm tier compression
   - Cache LLM #1 responses for identical inputs
   - Batch memory loading queries

4. Advanced testing (2 hours)
   - Stress tests (1000 turns)
   - Concurrency tests (multiple users)
   - Error injection tests

**Validation:**
- Cargo build with `--features dual-llm` → Smaller binary (classic code excluded)
- Performance improvement measurable (> 10% latency reduction)

**Note:** This phase is OPTIONAL. Keeping both implementations allows:
- Research comparison between approaches
- Fallback if LLM providers have outages
- A/B testing in production

---

## INTEGRATION POINTS

### 1. Processor Selection (flow_process.rs)
**Location:** `impl FlowProcess::new()` (line ~700)
**Change:**
```rust
pub fn new() -> Self {
    let config = DualLlmConfig::from_env().unwrap_or_default();
    Self::with_config(config)
}

pub fn with_config(config: DualLlmConfig) -> Self {
    let stages: Vec<Box<dyn StageProcessor>> = if config.is_enabled() {
        vec![
            Box::new(UnconscciousLlmProcessor::new(config.clone())),
            Box::new(InterfaceAttentionProcessor),
            Box::new(QualityEmergenceProcessor),
            Box::new(IntegrationProcessor),
            Box::new(ContinuityProcessor),
            Box::new(EvolutionProcessor),
        ]
    } else {
        vec![
            Box::new(DomainEmergenceProcessor),
            Box::new(BoundaryDissolutionProcessor),
            Box::new(InterfaceAttentionProcessor),
            Box::new(QualityEmergenceProcessor),
            Box::new(IntegrationProcessor),
            Box::new(ContinuityProcessor),
            Box::new(EvolutionProcessor),
        ]
    };
    Self { stages }
}
```

### 2. Memory Loading (memory.rs)
**Location:** `impl MemoryManager` (line ~105)
**Change:** Add new method
```rust
pub async fn get_recent_snapshots(
    &self,
    user_id: Uuid,
    limit: usize,
) -> Result<Vec<CompactStateSnapshot>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, user_id, timestamp, domain_states, boundary_states,
                pattern_ids, identity_anchors, metadata
         FROM state_snapshots
         WHERE user_id = ?
         ORDER BY timestamp DESC
         LIMIT ?"
    )
    .bind(user_id.as_bytes().to_vec())
    .bind(limit as i64)
    .fetch_all(&self.db_pool)
    .await?;

    // Parse each row into CompactStateSnapshot (reuse existing logic)
    let snapshots = rows.into_iter()
        .map(|row| self.parse_snapshot_row(row))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(snapshots)
}
```

### 3. LLM #1 Call (dual_llm/processors.rs)
**Location:** New file
**Implementation:**
```rust
impl UnconscciousLlmProcessor {
    async fn call_unconscious_llm(
        &self,
        context: &FlowContext,
    ) -> Result<UnconscciousLlmOutput, FlowError> {
        // 1. Load memory tiers
        let user_id = context.user_id;
        let (hot, warm) = self.memory_loader
            .load_tiered_memory(user_id, self.config.memory_hot_turns, self.config.memory_warm_turns)
            .await
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "Memory Loading".to_string(),
                reason: e.to_string(),
            })?;

        // 2. Build prompt
        let prompt = UnconscciousPromptBuilder::build(&context.user_input, hot, warm);

        // 3. Call LLM #1
        let response_text = self.llm.send_request(&prompt).await
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "LLM #1 Call".to_string(),
                reason: e.to_string(),
            })?;

        // 4. Parse response
        let output: UnconscciousLlmOutput = serde_json::from_str(&response_text)
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "LLM #1 Response Parsing".to_string(),
                reason: format!("Invalid JSON: {}", e),
            })?;

        // 5. Validate schema
        output.validate()?;

        Ok(output)
    }
}
```

### 4. State Transition (dual_llm/processors.rs)
**Location:** `impl StageProcessor for UnconscciousLlmProcessor`
**Implementation:**
```rust
fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
    let llm_output = self.call_unconscious_llm(context).await?;

    // Populate context.domains
    context.domains = llm_output.domains.into_iter()
        .map(|(name, activation)| (name, DomainActivation { activation }))
        .collect();

    // Populate context.boundaries
    context.boundaries = llm_output.boundaries.into_iter()
        .map(|b| BoundaryState::new(b.name, b.permeability, b.status))
        .collect();

    Ok(())
}
```

### 5. Configuration Loading (dual_llm/config.rs)
**Location:** New file
**Implementation:**
```rust
impl DualLlmConfig {
    pub fn from_env() -> Result<Self, LlmError> {
        dotenv::dotenv().ok();

        let enabled = std::env::var("DUAL_LLM_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let unconscious_model = std::env::var("UNCONSCIOUS_LLM_MODEL")
            .unwrap_or_else(|_| "anthropic/claude-3-5-haiku".to_string());

        let conscious_model = std::env::var("CONSCIOUS_LLM_MODEL")
            .unwrap_or_else(|_| "anthropic/claude-3-5-sonnet".to_string());

        let memory_hot_turns = std::env::var("MEMORY_HOT_TURNS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let memory_warm_turns = std::env::var("MEMORY_WARM_TURNS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);

        let fallback_on_error = std::env::var("DUAL_LLM_FALLBACK")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Ok(Self {
            enabled,
            unconscious_model,
            conscious_model,
            memory_hot_turns,
            memory_warm_turns,
            fallback_on_error,
        })
    }
}
```

---

## DEPENDENCIES ON OTHERS

| Needs_From | What | Why | Blocking |
|------------|------|-----|----------|
| System Architect | LLM #1 prompt template + output schema | Define unconscious processing contract | Yes (Phase 2) |
| Memory Expert | Memory tier compression algorithm | Optimize token usage for warm tier | No (can use full snapshots initially) |
| Prompt Engineer | Structured output format for LLM #1 | Ensure reliable JSON parsing | Yes (Phase 2) |
| Testing Team | Parity validation criteria | Define acceptable divergence between classic/dual-LLM | No (can use heuristics initially) |
| DevOps | CI/CD updates for dual-LLM testing | Ensure mocked tests run without API keys | No (can run locally first) |

**Critical Path:** System Architect's prompt specification blocks Phase 2 implementation.

---

## SUCCESS CRITERIA

### Phase 1 Success Metrics
- [ ] All 87 existing tests pass unchanged
- [ ] `DUAL_LLM_MODE=false` explicitly works (classic mode)
- [ ] `DUAL_LLM_MODE=true` fails gracefully (not yet implemented)
- [ ] New module compiles without warnings
- [ ] Zero performance regression in classic mode
- [ ] CI/CD pipeline green

### Phase 2 Success Metrics
- [ ] `UnconscciousLlmProcessor` passes unit tests with mocks
- [ ] Memory tiering loads correct snapshots (hot + warm)
- [ ] LLM #1 response parsing handles valid + invalid JSON
- [ ] Fallback to Rust calculators works on LLM #1 failure
- [ ] Integration tests pass (20+ new tests)
- [ ] Dual-LLM mode produces valid `FlowContext` for Stages 3-7

### Phase 3 Success Metrics
- [ ] Dual-LLM mode produces similar output to classic mode (> 90% parity)
- [ ] Performance acceptable (< 2x latency increase)
- [ ] API costs reasonable (< $0.01 per conversation turn)
- [ ] All 87 existing tests pass in dual-LLM mode
- [ ] Documentation updated
- [ ] Classic mode still works (rollback available)

### Phase 4 Success Metrics (Optional)
- [ ] Warm tier compression reduces token usage (> 50% reduction)
- [ ] Performance optimization measurable (> 10% improvement)
- [ ] Rust calculators deprecated but functional
- [ ] Stress tests pass (1000 turns, 100 concurrent users)

### Overall Validation
1. **Correctness:** Dual-LLM output semantically equivalent to classic mode
2. **Performance:** < 2x latency increase (target: < 500ms added latency)
3. **Reliability:** Fallback mechanism handles all LLM #1 failure modes
4. **Maintainability:** Code coverage > 80%, documentation complete
5. **Cost:** API costs < $0.01 per turn (haiku is ~$0.001 per request)

---

## RISK ANALYSIS

### Technical Risks

**1. LLM #1 Output Variability (HIGH)**
- **Risk:** LLM might return invalid JSON or unexpected structure
- **Mitigation:** Strict schema validation + fallback to Rust calculators
- **Detection:** Unit tests with edge cases (malformed JSON, missing fields)

**2. Performance Degradation (MEDIUM)**
- **Risk:** Adding LLM #1 call increases latency significantly
- **Mitigation:** Use fast model (haiku), parallel execution (future), caching
- **Detection:** Benchmark tests in Phase 3

**3. Memory Loading Cost (LOW)**
- **Risk:** Loading 25 snapshots per turn might slow down database
- **Mitigation:** Add database index on (user_id, timestamp), limit to 25
- **Detection:** Load testing with high user count

**4. Test Brittleness (MEDIUM)**
- **Risk:** LLM responses non-deterministic, tests flaky
- **Mitigation:** Use mocks for all automated tests, real LLMs only for manual validation
- **Detection:** CI/CD flakiness monitoring

### Process Risks

**1. Breaking Existing Tests (HIGH)**
- **Risk:** Refactoring accidentally breaks 87 passing tests
- **Mitigation:** Feature flag defaults to OFF, phased rollout, CI/CD validation
- **Detection:** Run full test suite after every commit

**2. Scope Creep (MEDIUM)**
- **Risk:** Phase 4 optimization delays other work
- **Mitigation:** Phase 4 is optional, can defer to future sprint
- **Detection:** Time-boxing each phase

**3. Dependency Blocking (MEDIUM)**
- **Risk:** Waiting for System Architect's prompt spec delays implementation
- **Mitigation:** Start Phase 1 (infrastructure) immediately, Phase 2 can wait
- **Detection:** Track dependencies in daily standups

---

## APPENDIX: CODE EXAMPLES

### Example: UnconscciousLlmProcessor Implementation

```rust
use crate::flow_process::{StageProcessor, FlowContext, FlowError, DomainActivation};
use crate::prompt_engine::BoundaryState;
use crate::LlmProvider;
use super::config::DualLlmConfig;
use super::prompts::UnconscciousPromptBuilder;
use super::memory_tiering::MemoryTierLoader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct UnconscciousLlmProcessor {
    llm: Box<dyn LlmProvider>,
    config: DualLlmConfig,
    memory_loader: MemoryTierLoader,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnconscciousLlmOutput {
    domains: HashMap<String, f64>,
    boundaries: Vec<BoundaryOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundaryOutput {
    name: String,
    permeability: f64,
    status: String,
}

impl UnconscciousLlmOutput {
    fn validate(&self) -> Result<(), FlowError> {
        // Validate domain activations
        for (name, activation) in &self.domains {
            if *activation < 0.0 || *activation > 1.0 {
                return Err(FlowError::StageProcessingFailed {
                    stage: "LLM #1 Validation".to_string(),
                    reason: format!("Domain {} activation {} out of range [0.0, 1.0]", name, activation),
                });
            }
        }

        // Validate boundary states
        for boundary in &self.boundaries {
            if boundary.permeability < 0.0 || boundary.permeability > 1.0 {
                return Err(FlowError::StageProcessingFailed {
                    stage: "LLM #1 Validation".to_string(),
                    reason: format!("Boundary {} permeability {} out of range", boundary.name, boundary.permeability),
                });
            }

            let valid_statuses = ["Maintained", "Transitional", "Transcendent"];
            if !valid_statuses.contains(&boundary.status.as_str()) {
                return Err(FlowError::StageProcessingFailed {
                    stage: "LLM #1 Validation".to_string(),
                    reason: format!("Boundary {} has invalid status '{}'", boundary.name, boundary.status),
                });
            }
        }

        Ok(())
    }
}

impl StageProcessor for UnconscciousLlmProcessor {
    fn name(&self) -> &str {
        "Unconsccious LLM (Stages 1-2)"
    }

    fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        match self.call_unconscious_llm(context).await {
            Ok(llm_output) => {
                // Populate context with LLM #1 output
                context.domains = llm_output.domains.into_iter()
                    .map(|(name, activation)| (name, DomainActivation { activation }))
                    .collect();

                context.boundaries = llm_output.boundaries.into_iter()
                    .map(|b| BoundaryState::new(b.name, b.permeability, b.status))
                    .collect();

                Ok(())
            }
            Err(e) if self.config.fallback_on_error => {
                // Fallback to Rust calculators
                eprintln!("LLM #1 failed ({}), falling back to Rust calculators", e);

                use crate::flow_process::{DomainEmergenceProcessor, BoundaryDissolutionProcessor};
                DomainEmergenceProcessor.process(context)?;
                BoundaryDissolutionProcessor.process(context)?;

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl UnconscciousLlmProcessor {
    pub fn new(config: DualLlmConfig, llm: Box<dyn LlmProvider>, memory_loader: MemoryTierLoader) -> Self {
        Self { llm, config, memory_loader }
    }

    async fn call_unconscious_llm(&self, context: &FlowContext) -> Result<UnconscciousLlmOutput, FlowError> {
        // 1. Load memory tiers
        let user_id = context.user_id;
        let (hot, warm) = self.memory_loader
            .load_tiered_memory(user_id, self.config.memory_hot_turns, self.config.memory_warm_turns)
            .await
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "Memory Loading".to_string(),
                reason: e.to_string(),
            })?;

        // 2. Build prompt
        let prompt = UnconscciousPromptBuilder::build(&context.user_input, hot, warm);

        // 3. Call LLM #1
        let response_text = self.llm.send_request(&prompt).await
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "LLM #1 Call".to_string(),
                reason: e.to_string(),
            })?;

        // 4. Parse response
        let output: UnconscciousLlmOutput = serde_json::from_str(&response_text)
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "LLM #1 Response Parsing".to_string(),
                reason: format!("Invalid JSON: {}. Response: {}", e, response_text),
            })?;

        // 5. Validate schema
        output.validate()?;

        Ok(output)
    }
}
```

### Example: Test with Mock

```rust
#[tokio::test]
async fn test_unconscious_llm_processor_with_mock() {
    use crate::dual_llm::processors::UnconscciousLlmProcessor;
    use crate::dual_llm::config::DualLlmConfig;
    use crate::mock_llm::MockLlm;

    // Mock LLM #1 response
    let mock_response = r#"{
        "domains": {
            "CD": 0.9,
            "SD": 0.8,
            "CuD": 0.6,
            "ED": 0.5
        },
        "boundaries": [
            {"name": "CD-SD", "permeability": 0.85, "status": "Transcendent"},
            {"name": "SD-CuD", "permeability": 0.7, "status": "Transitional"},
            {"name": "CuD-ED", "permeability": 0.6, "status": "Maintained"}
        ]
    }"#;

    let mock_llm = Box::new(MockLlm::new(vec![mock_response.to_string()]));
    let config = DualLlmConfig {
        enabled: true,
        fallback_on_error: false,
        // ... other config
    };

    let memory_loader = MemoryTierLoader::new_mock(vec![]); // Empty memory for test
    let processor = UnconscciousLlmProcessor::new(config, mock_llm, memory_loader);

    let mut context = FlowContext::new(
        "Test user input".to_string(),
        0.7,
        create_test_framework_state()
    );

    let result = processor.process(&mut context);

    assert!(result.is_ok());
    assert_eq!(context.domains.len(), 4);
    assert_eq!(context.domains.get("CD").unwrap().activation, 0.9);
    assert_eq!(context.boundaries.len(), 3);
    assert_eq!(context.boundaries[0].name, "CD-SD");
    assert_eq!(context.boundaries[0].permeability, 0.85);
}
```

---

**END OF REPORT**

**Next Steps:**
1. Review this plan with team (System Architect, Memory Expert, Prompt Engineer)
2. Get approval on phased approach
3. Request LLM #1 prompt specification from System Architect
4. Begin Phase 1 implementation (8 hours)

**Questions for Team:**
1. Do we want compile-time feature flags (Cargo.toml) or runtime-only (env vars)?
2. What is acceptable latency increase for dual-LLM mode (current target: < 2x)?
3. Should we implement Phase 4 (deprecation) or keep both modes permanently?
4. What parity threshold between classic/dual-LLM is acceptable (current target: 90%)?
