# DUAL-LLM IMPLEMENTATION ROADMAP
**Integration Lead** | Date: 2025-11-01 | Status: Complete

## EXECUTIVE SUMMARY

**Mission:** Implement the missing dual-LLM architecture (LLM #1 Unconscious + LLM #2 Conscious) and memory tiering system (hot/warm/cold) as specified in the original design documents.

**Approach:** Phased implementation with backward compatibility, feature flags, and comprehensive testing. All 87 existing tests preserved while adding dual-LLM capability.

**Timeline:** 15-19 working days total
- **Dual-LLM Core:** 10-14 days (3 phases)
- **Memory Tiering:** 5 days (3 phases)
- **Can run in parallel after infrastructure phase**

**Key Decisions:**
1. **Feature flag pattern** - `DUAL_LLM_MODE` env var (defaults to `false` initially)
2. **Backward compatibility** - All existing Rust processors preserved for fallback
3. **Model selection** - GPT-3.5-turbo for LLM #1 (fast/cheap), Claude 3.5 Sonnet for LLM #2
4. **Memory architecture** - Conversation turns as atomic units, separate from state snapshots
5. **Risk mitigation** - Comprehensive mocking strategy prevents expensive API calls in tests

---

## INTEGRATION SYNTHESIS

### Cross-Cutting Insights

**1. Architecture Convergence**
All 4 specialists independently arrived at the same core architecture:
- LLM #1 replaces Stages 1-2 (DomainEmergenceProcessor + BoundaryDissolutionProcessor)
- Stages 3-7 remain unchanged (interface attention → evolution)
- Feature flag enables conditional processor selection
- Fallback to Rust calculations on LLM #1 failure

**2. Memory System as Foundation**
Memory tiering is prerequisite for effective dual-LLM:
- Hot memory (3-5 turns) provides immediate context for LLM #1 calculations
- Warm memory (20-50 turns) enables session continuity
- Cold memory (all history) enables identity persistence across sessions
- **Dependency:** Implement at least hot memory before LLM #1 can be effective

**3. Cost-Performance Trade-offs**
Specialists converged on optimal model selection:
- **LLM #1:** GPT-3.5-turbo or Claude Haiku (~$0.002/call, 50-150ms)
- **LLM #2:** Claude 3.5 Sonnet (~$0.02/call, 500-2000ms)
- **Total cost increase:** 3-7% (LLM #1 adds negligible cost vs LLM #2)
- **Latency increase:** 10-20% (+50-200ms for LLM #1 call)

**4. Testing Strategy Alignment**
All specialists emphasized mock-based testing:
- Mocks prevent API costs in CI/CD
- Deterministic outputs prevent test flakiness
- Real LLM calls reserved for manual validation only
- Target: 25 new tests for dual-LLM, preserving all 87 existing tests

---

## UNIFIED TIMELINE

### Critical Path Analysis

**Sequential Dependencies:**
```
Phase 0 (Infrastructure) → Phase 1A (Hot Memory) → Phase 2A (LLM #1 Core)
                        ↓
                   Phase 2A (LLM #1 Core) → Phase 2B (LLM #1 Optimization)
                        ↓
                   Phase 3A (Production Hardening)
```

**Parallel Opportunities:**
```
Phase 1B (Warm Memory) can run parallel to Phase 2A (LLM #1 Core)
Phase 1C (Cold Memory) can run parallel to Phase 2B (LLM #1 Optimization)
```

**Optimized Timeline:** 15-19 days (vs 19-23 days sequential)

---

### PHASE 0: Infrastructure Setup [Day 1-2] (8 hours)

**Goal:** Add all scaffolding with feature flag OFF. Zero behavior change.

**Tasks:**
1. **Module Structure** (1h)
   - Create `api/src/dual_llm/` directory
   - Create `mod.rs`, `config.rs`, `processors.rs`, `prompts.rs`, `memory_tiering.rs`
   - Export module from `api/src/lib.rs`

2. **Configuration System** (1h)
   - Implement `DualLlmConfig` struct
   - Load from environment variables (`.env`)
   - Default: `enabled: false`

3. **Database Schema** (2h)
   - Migration: `20251101000001_add_conversation_turns.sql`
   - Tables: `conversation_turns`, `conversation_sessions`
   - Indexes: user_id, session_id, timestamp

4. **FlowProcess Integration** (2h)
   - Add `FlowProcess::with_config(config)` method
   - Conditional processor selection (classic vs dual-LLM)
   - Keep `FlowProcess::new()` unchanged (uses default config)

5. **Testing & Validation** (2h)
   - Run all 87 tests → verify 100% pass
   - Test `DUAL_LLM_MODE=false` explicitly
   - Test `DUAL_LLM_MODE=true` fails gracefully (not implemented yet)

**Deliverables:**
- ✅ Module structure in place
- ✅ Configuration loading works
- ✅ Database schema applied
- ✅ All 87 tests passing
- ✅ CI/CD green

**Blocking:** None (can start immediately)

---

### PHASE 1A: Hot Memory [Day 3] (8 hours)

**Goal:** Load last 3-5 conversation turns for LLM #2 context.

**Tasks:**
1. **MemoryTierManager** (3h)
   - Implement `load_hot_memory(session_id) -> Vec<ConversationTurn>`
   - Implement `save_conversation_turn(...) -> ConversationTurn`
   - In-memory cache (HashMap: session_id → HotMemory)

2. **VifApi Integration** (2h)
   - Add `memory_tiers: MemoryTierManager` field to VifApi
   - Modify `process_input()` to load/save turns
   - Construct conversation history for LLM #2 prompt

3. **Testing** (3h)
   - Test: `test_hot_memory_loading` (verify last 3 turns loaded)
   - Test: `test_hot_memory_eviction` (verify oldest evicted when limit exceeded)
   - Test: `test_session_creation` (verify session boundaries)

**Deliverables:**
- ✅ Hot memory loading works
- ✅ Conversation turns saved to database
- ✅ Last 3-5 turns available for LLM #2
- ✅ 3 new tests passing (90 total)

**Blocking:** Phase 0 complete

---

### PHASE 1B: Warm Memory [Day 4-5] (12 hours)

**Goal:** Load recent session turns (20-50) with recency scoring.

**Tasks:**
1. **Memory Tier Tracking** (2h)
   - Migration: `20251101000002_add_tier_tracking.sql`
   - Add `memory_tier` column to `conversation_turns`
   - Implement hot→warm eviction logic

2. **Warm Memory Loading** (4h)
   - Implement `load_warm_memory(session_id, max_tokens) -> Vec<ConversationTurn>`
   - Greedy token packing algorithm
   - Recency-based retrieval (ORDER BY turn_number DESC)

3. **Session Boundaries** (2h)
   - Detect session end (timeout-based + explicit)
   - Update `conversation_sessions.ended_at`
   - Mark warm turns for compression (Phase 1C)

4. **Testing** (4h)
   - Test: `test_warm_memory_retrieval`
   - Test: `test_hot_to_warm_eviction`
   - Test: `test_session_end_detection`
   - Test: `test_token_budget_enforcement`

**Deliverables:**
- ✅ Warm memory retrieval works
- ✅ Hot→warm transitions working
- ✅ Token budgets enforced
- ✅ 4 new tests passing (94 total)

**Blocking:** Phase 1A complete (hot memory working)

**Parallel Opportunity:** Can run parallel to Phase 2A (LLM #1 implementation)

---

### PHASE 1C: Cold Memory [Day 6-7] (16 hours)

**Goal:** Compress warm→cold, semantic retrieval, identity anchors.

**Tasks:**
1. **Compression Tables** (2h)
   - Migration: `20251101000003_add_compression_tables.sql`
   - Tables: `conversation_summaries`, `memory_tier_transitions`

2. **LLM #1 Compression Prompts** (3h)
   - Design compression prompt (summarize turns to 1-2 sentences)
   - Implement `compress_turn(turn) -> TurnSummary`
   - Identify identity-critical turns (preserve verbatim)

3. **Compression Pipeline** (5h)
   - Implement `compress_session_warm_to_cold(session_id)`
   - Triggered on session end (async job)
   - Batch process warm turns → summaries
   - Delete original warm turns (or mark compressed)

4. **Cold Retrieval** (4h)
   - Implement `load_cold_memory(user_id, user_message, max_tokens)`
   - Keyword-based search (Phase 1C: simple LIKE queries)
   - Recency scoring (exponential decay)
   - Combined relevance scoring (0.5*recency + 0.3*semantic + 0.2*anchor)

5. **Testing** (2h)
   - Test: `test_warm_to_cold_compression`
   - Test: `test_cold_memory_retrieval`
   - Test: `test_identity_anchor_persistence`
   - Test: `test_cross_session_continuity`

**Deliverables:**
- ✅ Warm→cold compression working
- ✅ Cold retrieval by relevance working
- ✅ Identity anchors persist across sessions
- ✅ 4 new tests passing (98 total)

**Blocking:** Phase 1B complete (warm memory working)

**Parallel Opportunity:** Can run parallel to Phase 2B (LLM #1 optimization)

---

### PHASE 2A: LLM #1 Core Implementation [Day 8-12] (5-7 days)

**Goal:** Replace Rust Stages 1-2 with LLM #1 calls.

**Day 8: Types & Prompts** (1 day)
1. **Output Schema** (2h)
   - Add `Llm1Output`, `Llm1BoundaryState` types to `dual_llm/processors.rs`
   - Implement JSON schema validation
   - Unit tests for parsing + validation

2. **Prompt Engineering** (6h)
   - Implement `UnconscciousPromptBuilder::build()`
   - System prompt (~2500 tokens) with domain/boundary definitions
   - 5 few-shot examples (technical, philosophical, mixed, follow-up, edge case)
   - Test prompt construction

**Day 9-10: Processor Implementation** (1.5 days)
3. **UnconscciousLlmProcessor** (8h)
   - Implement `call_llm1_with_retry()` with timeout/retry logic (max 2 retries)
   - Implement `process()` method (StageProcessor trait)
   - Validation + fallback to Rust calculators
   - Unit tests (retry logic, fallback, error handling)

4. **Integration** (4h)
   - Add processor to `FlowProcess::with_config()`
   - Remove Stage 2 (BoundaryDissolutionProcessor) from dual-LLM path
   - Integration tests (mocked LLM #1)

**Day 11-12: Testing & Validation** (1.5-2 days)
5. **Test Suite** (8h)
   - 15 unit tests (prompt building, JSON parsing, validation)
   - 10 integration tests (full dual-LLM flow with mocks)
   - Parity testing (dual-LLM vs Rust: similar outputs)

6. **Bug Fixes & Refinement** (4h)
   - Address edge cases discovered in testing
   - Clippy warnings, code review feedback
   - Documentation

**Deliverables:**
- ✅ LLM #1 prompts working (system + few-shot examples)
- ✅ UnconscciousLlmProcessor passes unit tests with mocks
- ✅ Fallback to Rust works on LLM #1 failure
- ✅ Dual-LLM flow produces valid responses
- ✅ 25 new tests passing (123 total)

**Blocking:** Phase 0 complete, Phase 1A helpful (hot memory available)

**Dependencies:**
| Needs | What | Blocking |
|-------|------|----------|
| Prompt Engineering Expert | LLM #1 prompt template + examples | Yes (Day 8) |
| LLM Architecture Expert | Output schema specification | Yes (Day 8) |

---

### PHASE 2B: LLM #1 Optimization [Day 13-15] (3-4 days)

**Goal:** Optimize latency, cost, reliability.

**Tasks:**
1. **Model Evaluation** (1 day)
   - Test GPT-3.5-turbo, Claude Haiku, GPT-4o-mini
   - Measure latency (100 calls each)
   - Measure accuracy (JSON validity rate, schema compliance)
   - Measure cost (token usage)

2. **Performance Benchmarking** (1 day)
   - Implement `test_performance_dual_llm_vs_rust_stages`
   - Measure P50, P95, P99 latency
   - Compare to Day 9 baseline (<1ms Rust)
   - Token cost per interaction

3. **Caching** (1 day)
   - LRU cache for LLM #1 results (key: user input hash)
   - Cache eviction policy (max 1000 entries, 1h TTL)
   - Test cache hit rate (expect 10-20%)

4. **Config Presets** (0.5 day)
   - Implement `DualLlmConfigPresets::balanced()`, `::fast()`, `::cheap()`, `::reliable()`
   - Test each preset
   - Document trade-offs

**Deliverables:**
- ✅ Model recommendation documented (likely GPT-3.5-turbo)
- ✅ Performance benchmarks show <20% latency increase
- ✅ Caching reduces latency by 10-20% on cache hits
- ✅ Config presets documented

**Blocking:** Phase 2A complete

**Parallel Opportunity:** Can run parallel to Phase 1C (cold memory)

---

### PHASE 3A: Production Hardening [Day 16-17] (2-3 days)

**Goal:** Error handling, observability, documentation.

**Tasks:**
1. **Error Handling & Logging** (1 day)
   - Structured logging (log LLM #1 inputs/outputs)
   - Circuit breaker pattern (disable LLM #1 after 5 consecutive failures)
   - Metrics (call count, failure rate, fallback rate)
   - Test failure scenarios (auth error, rate limit, timeout)

2. **Documentation** (1 day)
   - Update `ARCHITECTURE.md` with dual-LLM section
   - Write migration guide (Rust-only → Dual-LLM)
   - Document config options (presets, custom)
   - Troubleshooting guide (LLM #1 failures, debugging)

3. **Final Validation** (0.5-1 day)
   - Run full test suite (110+ tests)
   - Manual testing (diverse inputs, edge cases)
   - Load testing (100 concurrent requests)
   - Validate fallback scenarios

**Deliverables:**
- ✅ Circuit breaker prevents cascading failures
- ✅ Logging provides visibility into LLM #1 decisions
- ✅ Documentation complete
- ✅ All tests passing (110+ tests)
- ✅ Load testing stable

**Blocking:** Phase 2B complete

---

## INTEGRATED DEPENDENCY MATRIX

### Critical Path Dependencies

**Phase 0 (Infrastructure):**
- Blocks: All subsequent phases
- Needs: None (can start immediately)

**Phase 1A (Hot Memory):**
- Blocks: Phase 1B, Phase 2A (LLM #1 needs memory context)
- Needs: Phase 0 complete

**Phase 1B (Warm Memory):**
- Blocks: Phase 1C
- Needs: Phase 1A complete
- Can run parallel to: Phase 2A

**Phase 1C (Cold Memory):**
- Blocks: None (nice-to-have for Phase 3)
- Needs: Phase 1B complete
- Can run parallel to: Phase 2B

**Phase 2A (LLM #1 Core):**
- Blocks: Phase 2B, Phase 3A
- Needs: Phase 0 complete, Phase 1A helpful (hot memory)
- External dependency: Prompt Engineering Expert (LLM #1 prompts)

**Phase 2B (Optimization):**
- Blocks: Phase 3A
- Needs: Phase 2A complete
- Can run parallel to: Phase 1C

**Phase 3A (Production Hardening):**
- Blocks: None (final phase)
- Needs: Phase 2B complete

### External Dependencies

| Phase | Needs From | What | Blocking | Timeline |
|-------|------------|------|----------|----------|
| 2A (Day 8) | Prompt Engineering Expert | LLM #1 prompt template + 5 examples | Yes | Day 8 start |
| 2A (Day 8) | LLM Architecture Expert | Output JSON schema specification | Yes | Day 8 start |
| 2B (Day 13) | DevOps | API keys (OpenAI, Anthropic) for testing | No (can use mocks) | Day 13 start |
| 3A (Day 16) | QA | Manual testing of LLM #1 outputs | No (nice-to-have) | Day 16 |

**Critical:** Prompt Engineering Expert and LLM Architecture Expert must deliver by Day 8 (start of Phase 2A).

---

## PHASING STRATEGY

### Recommended Path: Parallel Execution

**Week 1 (Days 1-5):**
- Day 1-2: Phase 0 (Infrastructure) [8h]
- Day 3: Phase 1A (Hot Memory) [8h]
- Day 4-5: **PARALLEL START**
  - Track A: Phase 1B (Warm Memory) [12h]
  - Track B: Phase 2A Day 1 (LLM #1 Types & Prompts) [8h]

**Week 2 (Days 6-10):**
- Day 6-7: **PARALLEL CONTINUE**
  - Track A: Phase 1C (Cold Memory) [16h]
  - Track B: Phase 2A Days 2-3 (LLM #1 Processor) [12h]
- Day 8-10: Phase 2A Days 4-5 (Testing & Validation) [12h]

**Week 3 (Days 11-15):**
- Day 11-13: Phase 2B (Optimization) [24h] - can overlap with Phase 1C cleanup
- Day 14-15: Phase 3A (Production Hardening) [16h]

**Total Duration:** 15-17 working days (3 weeks)

**Critical Path:** Infrastructure → Hot Memory → LLM #1 Core → Optimization → Hardening (15 days)

**Parallelizable Work:** Warm Memory + Cold Memory (saves ~4 days vs sequential)

---

## SUCCESS CRITERIA

### Phase-by-Phase Validation

**Phase 0 Success:**
- [ ] All 87 existing tests pass unchanged
- [ ] `DUAL_LLM_MODE=false` works (classic mode)
- [ ] New module compiles without warnings
- [ ] Database schema applied successfully

**Phase 1A Success:**
- [ ] Hot memory loads last 3-5 turns in <1ms
- [ ] Conversation turns persist to database
- [ ] 3 new tests passing (90 total)

**Phase 1B Success:**
- [ ] Warm memory loads session turns in <50ms
- [ ] Hot→warm transitions working
- [ ] Token budgets enforced
- [ ] 4 new tests passing (94 total)

**Phase 1C Success:**
- [ ] Warm→cold compression reduces storage by ~70%
- [ ] Cold retrieval by relevance working
- [ ] Identity anchors persist across sessions
- [ ] 4 new tests passing (98 total)

**Phase 2A Success:**
- [ ] LLM #1 produces valid JSON output (>95% success rate with mocks)
- [ ] Fallback to Rust works on LLM #1 failure
- [ ] Dual-LLM flow produces valid responses
- [ ] 25 new tests passing (123 total)

**Phase 2B Success:**
- [ ] Latency increase <20% vs Rust-only baseline
- [ ] Token cost <$0.002 per LLM #1 call
- [ ] Model recommendation documented
- [ ] Caching improves performance by 10-20%

**Phase 3A Success:**
- [ ] Circuit breaker prevents cascading failures
- [ ] Documentation complete (architecture, migration, troubleshooting)
- [ ] All 110+ tests passing
- [ ] Load testing stable (100 concurrent users)

### Overall System Validation

**Functional:**
1. [ ] Dual-LLM architecture matches `/memory-bank/dual-llm-architecture.md` specification
2. [ ] LLM #1 calculates domain activations from user input + memory context
3. [ ] Memory tiering provides hot/warm/cold context to LLM #1
4. [ ] Identity continuity across sessions (user returns after 7 days, context loaded)
5. [ ] Graceful fallback to Rust calculators on LLM #1 failure

**Quality:**
1. [ ] Test coverage >80% (currently 75%+)
2. [ ] All tests passing (target: 110+ tests)
3. [ ] Zero dead code (all methods used)
4. [ ] Clippy clean (no warnings)

**Performance:**
1. [ ] Total latency <2x Rust-only baseline (target: <1200ms total, currently ~500ms)
2. [ ] Cost increase <10% (LLM #1 adds ~$0.002, LLM #2 is ~$0.02)
3. [ ] Hot memory: <1ms latency
4. [ ] Warm memory: <50ms latency
5. [ ] Cold memory: <200ms latency

**User Experience:**
1. [ ] Seamless continuity (user doesn't notice memory system)
2. [ ] "Remembers" recent context (hot memory always loaded)
3. [ ] "Recalls" past discussions when relevant (cold retrieval)
4. [ ] Identity consistency ("my name is Alex" persists across sessions)

---

## RISK MITIGATION

### Identified Risks & Mitigations

**1. LLM #1 Output Variability (HIGH)**
- **Risk:** Non-deterministic LLM might return invalid JSON or unexpected values
- **Mitigation:**
  - Strict JSON schema validation with retry logic (max 2 retries)
  - Three-tier prompt strategy (standard → simplified → minimal)
  - Fallback to Rust calculators if all retries fail
  - ±15% tolerance on permeability calculations (LLM may not calculate exactly)
- **Detection:** Unit tests with edge cases (malformed JSON, missing fields, out-of-range values)

**2. Performance Degradation (MEDIUM)**
- **Risk:** Adding LLM #1 call increases latency significantly
- **Mitigation:**
  - Use fast model (GPT-3.5-turbo: 50-150ms, Claude Haiku: 80-200ms)
  - Caching for identical inputs (10-20% cache hit rate expected)
  - Parallel execution (future: call LLM #1 + load snapshot in parallel)
- **Detection:** Performance benchmarks in Phase 2B (target: <20% increase)

**3. Breaking Existing Tests (HIGH)**
- **Risk:** Refactoring accidentally breaks 87 passing tests
- **Mitigation:**
  - Feature flag defaults to `false` (classic mode)
  - Zero modifications to existing processors (preserved for fallback)
  - All 87 tests run in classic mode by default
  - New tests explicitly enable dual-LLM mode
- **Detection:** CI/CD runs full test suite after every commit

**4. Database Performance (LOW)**
- **Risk:** Loading 25 snapshots per turn slows down database
- **Mitigation:**
  - Add index on (user_id, timestamp)
  - Limit warm memory to 20 turns (configurable)
  - In-memory cache for hot memory (no DB query)
- **Detection:** Load testing in Phase 3A (100 concurrent users)

**5. Test Flakiness (MEDIUM)**
- **Risk:** Non-deterministic LLM responses cause flaky tests
- **Mitigation:**
  - Use mocks for all automated tests (MockLlm with canned responses)
  - Real LLM calls only for manual validation
  - No real API calls in CI/CD
- **Detection:** Monitor CI/CD flakiness rate (target: <1%)

**6. External Dependencies (MEDIUM)**
- **Risk:** Waiting for Prompt Engineering Expert delays Phase 2A
- **Mitigation:**
  - Phase 0 + Phase 1A can proceed independently (8+8=16 hours of work)
  - Phase 1B can run parallel to Phase 2A (reduces blocking time)
  - Fallback: Use placeholder prompts, iterate later
- **Detection:** Track dependency delivery in daily standups

---

## VALIDATION GATES

### Phase Transition Criteria

**Before Starting Phase 1A:**
- ✅ Phase 0 complete (infrastructure in place)
- ✅ All 87 tests passing in classic mode
- ✅ Database schema applied successfully

**Before Starting Phase 1B:**
- ✅ Phase 1A complete (hot memory working)
- ✅ 90 tests passing (87 + 3 new)
- ✅ Conversation turns saved to database

**Before Starting Phase 2A:**
- ✅ Phase 0 complete (infrastructure in place)
- ✅ Phase 1A complete (hot memory available)
- ✅ Prompt Engineering Expert delivered LLM #1 prompts
- ✅ LLM Architecture Expert delivered output schema

**Before Starting Phase 2B:**
- ✅ Phase 2A complete (LLM #1 core working)
- ✅ 123 tests passing (87 + 36 new)
- ✅ Dual-LLM flow produces valid responses with mocks

**Before Starting Phase 3A:**
- ✅ Phase 2B complete (optimization done)
- ✅ Performance benchmarks within acceptable range (<20% latency increase)
- ✅ Model recommendation finalized

**Before Production Deployment:**
- ✅ Phase 3A complete (production hardening)
- ✅ All 110+ tests passing
- ✅ Load testing successful (100 concurrent users)
- ✅ Documentation complete
- ✅ Manual QA sign-off

---

## TESTING STRATEGY

### Test Pyramid

**Unit Tests (~60 tests):**
- Prompt construction (5 tests)
- JSON parsing + validation (10 tests)
- LLM #1 output validation (5 tests)
- Memory tier loading (10 tests)
- Error handling + fallback (10 tests)
- Existing tests in classic mode (87 tests - no changes)

**Integration Tests (~30 tests):**
- Full dual-LLM flow with mocks (10 tests)
- Memory tiering (hot/warm/cold) (10 tests)
- Parity testing (dual-LLM vs Rust) (5 tests)
- Session boundaries (5 tests)

**End-to-End Tests (~5 tests, manual):**
- Real LLM calls (not in CI/CD)
- Cross-session continuity
- Identity persistence
- Performance validation

**Total Test Count:** ~150 tests (87 existing + 63 new)

**Coverage Target:** >80% (currently 75%+)

### Mocking Strategy

**MockLlm for LLM #1:**
```rust
// Returns deterministic JSON responses
let mock = MockLlm::new(vec![
    r#"{
        "domains": {"CD": 0.9, "SD": 0.8, "CuD": 0.6, "ED": 0.5},
        "boundaries": [
            {"name": "CD-SD", "permeability": 0.85, "status": "Transcendent"}
        ]
    }"#.to_string()
]);
```

**MockMemoryTierLoader:**
```rust
// Returns canned conversation turns
let mock = MockMemoryTierLoader::new(vec![
    ConversationTurn { user_message: "Test", ai_response: "Response", ... }
]);
```

**CI/CD Configuration:**
```yaml
- name: Run classic tests (no API calls)
  run: cargo test --lib
  env:
    DUAL_LLM_MODE: false

- name: Run dual-LLM tests (mocked)
  run: cargo test --test dual_llm_integration
  env:
    DUAL_LLM_MODE: true
    USE_MOCK_LLM: true
```

---

## CONFIGURATION REFERENCE

### Environment Variables

```bash
# =============================================================================
# DUAL-LLM ARCHITECTURE
# =============================================================================
# Enable dual-LLM mode (default: false initially, true after Phase 3)
DUAL_LLM_MODE=false

# LLM #1 (Unconscious): Domain emergence + boundary states
# Options: "openai/gpt-3.5-turbo", "anthropic/claude-3-5-haiku"
UNCONSCIOUS_LLM_MODEL=openai/gpt-3.5-turbo

# LLM #2 (Conscious): Integration + response (uses DEFAULT_MODEL)
CONSCIOUS_LLM_MODEL=anthropic/claude-3-5-sonnet

# Timeout for LLM #1 calls (milliseconds)
LLM1_TIMEOUT_MS=5000

# Max retries for LLM #1 (before fallback to Rust)
LLM1_MAX_RETRIES=2

# Fallback to Rust calculators if LLM #1 fails (default: true)
DUAL_LLM_FALLBACK=true

# =============================================================================
# MEMORY TIERING
# =============================================================================
# Hot memory: Full detail for last N turns (default: 5)
MEMORY_HOT_TURNS=5

# Warm memory: Compressed detail for turns N+1 to M (default: 20)
MEMORY_WARM_TURNS=20

# Cold memory: All history, compressed (no limit)
# (configured automatically based on relevance retrieval)
```

### Configuration Presets

**Balanced (Recommended):**
```rust
DualLlmConfigPresets::balanced()
// LLM #1: GPT-3.5-turbo (fast, cheap)
// LLM #2: Claude 3.5 Sonnet (smart, expensive)
// Timeout: 5s, Retries: 2, Fallback: true
```

**Fast (Minimize Latency):**
```rust
DualLlmConfigPresets::fast()
// LLM #1: GPT-3.5-turbo (fastest major provider)
// LLM #2: GPT-4o
// Timeout: 3s, Retries: 1, Fallback: true
```

**Cheap (Minimize Cost):**
```rust
DualLlmConfigPresets::cheap()
// LLM #1: Claude Haiku (cheapest)
// LLM #2: Claude 3.5 Sonnet
// Timeout: 8s, Retries: 2, Fallback: true
```

**Reliable (Maximize Reliability):**
```rust
DualLlmConfigPresets::reliable()
// LLM #1: GPT-3.5-turbo
// LLM #2: Claude 3.5 Sonnet
// Timeout: 5s, Retries: 3, Fallback: true (always)
```

---

## ROLLOUT PLAN

### Phase-by-Phase Rollout

**Week 1: Internal Testing (Phases 0-1A)**
- Deploy with `DUAL_LLM_MODE=false` to staging
- Enable dual-LLM for internal team testing only
- Collect feedback on LLM #1 output quality

**Week 2-3: Beta Testing (Phases 1B-2B)**
- Invite 10 beta testers
- Enable `DUAL_LLM_MODE=true` for beta cohort
- Monitor performance, API costs, error rates
- Iterate on prompts based on feedback

**Week 4: Production Rollout (Phase 3A)**
- Enable `DUAL_LLM_MODE=true` by default
- Monitor metrics: latency, cost, fallback rate
- Keep classic mode available for rollback

**Week 5+: Optimization**
- Implement Phase 2B optimizations (caching, etc.)
- Gather production data for Phase 1C (cold memory compression)

### Rollback Strategy

**If LLM #1 fails in production:**
1. Set `DUAL_LLM_MODE=false` globally (instant rollback)
2. All users fall back to classic Rust calculators
3. No data loss (all conversation turns saved)
4. Debug LLM #1 issues offline
5. Re-enable once fixed

**If memory system fails:**
1. Disable memory loading (use empty context)
2. LLM #1 still works (degraded quality)
3. Fix database issues
4. Re-enable memory loading

---

## COST-BENEFIT ANALYSIS

### Implementation Cost

**Development Time:**
- Phase 0: 8 hours
- Phase 1A: 8 hours
- Phase 1B: 12 hours
- Phase 1C: 16 hours
- Phase 2A: 40-56 hours (5-7 days)
- Phase 2B: 24-32 hours (3-4 days)
- Phase 3A: 16-24 hours (2-3 days)
- **Total: 124-156 hours (15-19 days)**

**API Costs (Testing):**
- Mocked tests: $0
- Manual validation: ~100 calls × $0.002 = $0.20
- **Total: <$1**

### Operational Cost (Production)

**Per Interaction:**
- LLM #1 (GPT-3.5-turbo): ~$0.002
- LLM #2 (Claude 3.5 Sonnet): ~$0.02
- **Total: ~$0.022** (vs $0.02 currently = 10% increase)

**Per 1000 Users/Day:**
- Assume 10 interactions/user/day = 10,000 interactions
- Cost: 10,000 × $0.022 = **$220/day** = **$6,600/month**
- Current (LLM #2 only): $200/day = $6,000/month
- **Increase: $600/month (10%)**

### Benefits

**Technical:**
1. **Contextual domain activation** - LLM #1 considers user intent, not just keywords
2. **Memory continuity** - Identity persists across sessions ("remembers" user)
3. **Semantic boundary dissolution** - More intelligent integration decisions
4. **Fallback reliability** - Rust calculators provide safety net

**User Experience:**
1. **Personalized responses** - System adapts to user's conversation style
2. **Long-term memory** - User can reference past conversations ("remember when...")
3. **Improved relevance** - Responses consider full conversation context
4. **Identity continuity** - System "knows" user across sessions

**Business:**
1. **Competitive differentiation** - True dual-LLM architecture (novel)
2. **Research validation** - Aligns with VIF specification (not just approximation)
3. **Scalable architecture** - Memory tiering enables long conversations without token explosion
4. **Flexible configuration** - Can tune cost/quality trade-offs per user tier

**ROI:**
- Implementation: 15-19 days (~$10,000-15,000 dev cost)
- Operational: +10% cost ($600/month for 1000 users)
- Break-even: If 10% better UX retains 1 extra user/month ($50 LTV), ROI achieved at 12 retained users
- **Conservative estimate: Positive ROI within 6 months**

---

## NEXT STEPS

### Immediate Actions (Next 24 Hours)

1. **Review & Approval** (2 hours)
   - Team review of this roadmap
   - Product owner approval on timeline + cost
   - Confirm external dependencies (Prompt Engineering Expert availability)

2. **Environment Setup** (1 hour)
   - Obtain API keys (OpenAI, Anthropic) for testing
   - Set up staging environment
   - Create feature branch `feature/dual-llm-implementation`

3. **Begin Phase 0** (8 hours)
   - Create module structure
   - Implement configuration loading
   - Add database schema migration
   - Modify FlowProcess for conditional processor selection

### Week 1 Milestones

- **Day 1-2:** Phase 0 complete, all 87 tests passing
- **Day 3:** Phase 1A complete, hot memory working
- **Day 4-5:** Phase 1B started (warm memory), Phase 2A Day 1 (prompts)

### Week 2 Milestones

- **Day 6-10:** Phase 1C (cold memory) + Phase 2A Days 2-5 (LLM #1 processor + testing)
- **End of Week 2:** Dual-LLM core working with mocks, 123 tests passing

### Week 3 Milestones

- **Day 11-15:** Phase 2B (optimization) + Phase 3A (production hardening)
- **End of Week 3:** Production-ready dual-LLM system, 110+ tests passing, documentation complete

### Week 4+ (Post-Implementation)

- Beta testing with 10 users
- Performance monitoring + iteration
- Production rollout (enable `DUAL_LLM_MODE=true` by default)

---

## QUESTIONS FOR TEAM

### Technical Decisions

1. **Model Selection:** Do we want to support multiple LLM #1 providers (OpenAI + Anthropic) or start with one?
   - **Recommendation:** Start with OpenAI (GPT-3.5-turbo) for Phase 2A, add Anthropic in Phase 2B

2. **Memory Compression:** Should we implement LLM-based compression (Phase 1C) or use simple keyword extraction initially?
   - **Recommendation:** Simple keyword extraction for Phase 1C MVP, LLM compression in Phase 4 (optional)

3. **Feature Flag Strategy:** Compile-time (Cargo features) or runtime-only (env vars)?
   - **Recommendation:** Runtime-only for flexibility (can toggle per-user in future)

4. **Caching Strategy:** Where to cache LLM #1 results (in-memory, Redis, database)?
   - **Recommendation:** In-memory LRU cache for Phase 2B, migrate to Redis in Phase 4 if needed

### Process Decisions

5. **Phasing:** Sequential (19 days) or parallel (15 days)?
   - **Recommendation:** Parallel (start Phase 1B + Phase 2A simultaneously after Phase 1A)

6. **Testing:** What parity threshold between classic/dual-LLM is acceptable?
   - **Recommendation:** 90% output similarity for Phase 2A validation

7. **Rollout:** Beta cohort size and duration?
   - **Recommendation:** 10 users, 1 week beta testing before production rollout

8. **Budget:** API cost budget for testing + production?
   - **Recommendation:** $50 for testing, monitor production costs weekly (expect $600/month for 1000 users)

---

## APPENDIX: FILE CHANGES SUMMARY

### Files to CREATE (8 files)

1. `api/src/dual_llm/mod.rs` - Module root
2. `api/src/dual_llm/config.rs` - Configuration loading
3. `api/src/dual_llm/processors.rs` - UnconscciousLlmProcessor
4. `api/src/dual_llm/prompts.rs` - LLM #1 prompt templates
5. `api/src/dual_llm/memory_tiering.rs` - Memory tier loading
6. `api/tests/dual_llm_integration.rs` - Integration tests
7. `migrations/20251101000001_add_conversation_turns.sql` - Hot memory schema
8. `migrations/20251101000002_add_tier_tracking.sql` - Warm/cold memory schema

### Files to MODIFY (5 files)

1. `api/src/lib.rs` - Export `dual_llm` module
2. `api/src/flow_process.rs` - Conditional processor selection
3. `api/src/memory.rs` - Add `get_recent_snapshots()` method
4. `api/.env.example` - Add dual-LLM configuration variables
5. `api/Cargo.toml` - (Optional) Add feature flags

### Files to DELETE

**NONE** (all existing code preserved for backward compatibility)

---

**END OF ROADMAP**

**Status:** Complete - Ready for team review and approval

**Estimated Total Effort:** 15-19 working days (124-156 hours)

**Next Action:** Schedule team review meeting to approve roadmap and begin Phase 0
