# Dual-LLM Architecture - Executive Summary

## What We're Building

Replace the current Rust-based domain activation and boundary permeability calculations (Stages 1-2) with **LLM #1 calls** to implement the dual-LLM architecture specified in the framework documentation.

## Why This Matters

**Current State:** Rust code calculates domain activations using fixed formulas (e.g., `CD activation = 0.8 * autonomy_level`). This is fast and deterministic, but lacks contextual intelligence.

**Proposed State:** LLM #1 analyzes user input and context to calculate domain activations intelligently. Example:
- Input: "How do I implement quicksort?" → CD: 0.95, SD: 0.3, CuD: 0.1, ED: 0.2
- Input: "What does it feel like to understand deeply?" → CD: 0.3, SD: 0.4, CuD: 0.7, ED: 0.95

This enables the framework to respond with contextually appropriate domain integration.

## Key Architectural Decisions

### 1. Model Selection: GPT-3.5-turbo for LLM #1

**Why:** Structured output tasks don't need GPT-4's reasoning. GPT-3.5-turbo is:
- **4-8x faster** (50-150ms vs 500-2000ms)
- **20x cheaper** ($0.0015/1K tokens vs $0.03/1K tokens)
- **Good enough** for JSON schema compliance with few-shot examples

**Alternative:** Claude 3 Haiku (even cheaper at $0.00025/1K tokens, but slightly slower)

### 2. Fallback Strategy: Always Fallback to Rust

**Why:** Reliability > Intelligence for production systems.

**Implementation:**
```rust
match llm1_call().await {
    Ok(state) => use_llm1_state(state),
    Err(e) => {
        eprintln!("LLM #1 failed: {:?}", e);
        use_rust_calculations() // Existing code path
    }
}
```

**Result:** System never fails due to LLM #1 issues (network, auth, rate limit).

### 3. Backward Compatibility: Dual Constructor Pattern

**Existing:** `VifApi::new(llm_provider, ...)` → Rust Stages 1-2 (unchanged)
**New:** `VifApi::new_with_dual_llm(llm1, llm2, ...)` → LLM #1 Stages 1-2

**Migration:** Gradual rollout, existing tests continue to pass.

### 4. Performance Impact: +10-20% Latency

**Current:** ~500-2000ms total (dominated by LLM #2)
**Proposed:** ~550-2200ms total (+50-200ms from LLM #1)

**Mitigation:**
- Use fastest model (GPT-3.5-turbo: ~50ms)
- Implement caching (10-20% hit rate expected)
- Parallel execution (future optimization)

### 5. Cost Impact: +3-7% Total Cost

**Current:** $0.015-0.05 per interaction (LLM #2 only)
**Proposed:** $0.016-0.052 per interaction (LLM #1 + LLM #2)

**Breakdown:**
- LLM #1: $0.0006-0.002 per interaction (GPT-3.5-turbo)
- LLM #2: $0.015-0.05 per interaction (Claude 3.5 Sonnet)

**Negligible cost increase for contextual intelligence.**

## Implementation Plan: 10-14 Days

### Phase 1: Core Implementation (5-7 days)
- Day 1: Setup types, validation tests
- Day 2: Prompt engineering (few-shot examples)
- Day 3: LLM #1 processor + retry logic
- Day 4-5: Integration with FlowProcess
- Day 6: Backward compatibility testing
- Day 7: Code review & bug fixes

**Deliverable:** Dual-LLM flow works, all existing tests pass

### Phase 2: Optimization (3-4 days)
- Day 8: Model evaluation (GPT-3.5 vs Claude Haiku vs GPT-4o-mini)
- Day 9: Performance benchmarking (latency, token cost)
- Day 10: Caching implementation
- Day 11: Config presets (balanced, fast, cheap, reliable)

**Deliverable:** Optimized for production use

### Phase 3: Production Hardening (2-3 days)
- Day 12: Error handling, logging, circuit breaker
- Day 13: Documentation (architecture, migration, troubleshooting)
- Day 14: Final testing, load testing, validation

**Deliverable:** Production-ready dual-LLM system

## Risk Assessment

### High Risk (Mitigated)

**Risk:** LLM #1 failures break user experience
**Mitigation:** Fallback to Rust calculations (always enabled by default)

**Risk:** Latency increase unacceptable
**Mitigation:** Use fastest model (GPT-3.5-turbo), caching, parallel execution

**Risk:** Cost increase unacceptable
**Mitigation:** Use cheap model ($0.0006-0.002 per interaction), only 3-7% total cost increase

### Medium Risk (Monitoring Required)

**Risk:** LLM #1 returns nonsensical values (e.g., CD: 1.5)
**Mitigation:** Strict validation (0.0-1.0 range), retry with clarified prompt, fallback to Rust

**Risk:** JSON parsing fails (LLM #1 returns markdown or prose)
**Mitigation:** Use JSON mode (OpenAI native), few-shot examples, retry logic

### Low Risk (Acceptable)

**Risk:** LLM #1 calculations differ from Rust (non-determinism)
**Mitigation:** Expected behavior (adds intelligence), fallback available for deterministic needs

**Risk:** Complex debugging (two LLM calls vs one)
**Mitigation:** Structured logging (log LLM #1 inputs/outputs), observability metrics

## Success Metrics

**Phase 1 Complete:**
- [ ] Dual-LLM flow produces valid responses
- [ ] All 84 existing tests pass (backward compatibility)
- [ ] +15 new tests added (unit + integration)

**Phase 2 Complete:**
- [ ] Latency increase <20% (P95: <200ms added)
- [ ] Token cost <$0.002 per interaction (LLM #1)
- [ ] Caching works (10-20% latency reduction on hits)

**Phase 3 Complete:**
- [ ] Circuit breaker prevents cascading failures
- [ ] Documentation complete (architecture, migration, troubleshooting)
- [ ] Load testing stable (100 concurrent requests)

**Overall Success:**
- [ ] LLM #1 calculations more intelligent than Rust (qualitative QA assessment)
- [ ] System reliability maintained (fallback ensures no user-facing failures)
- [ ] Cost increase justified (<10% for contextual intelligence)

## Open Questions for Review

1. **Oscillatory Parameters:** Should LLM #1 also calculate boundary oscillation (frequency, amplitude, phase) or keep Rust defaults?
   - **Recommendation:** Start with Rust defaults (simpler), add to LLM #1 in Phase 4 if needed

2. **Multi-Provider Fallback:** Should LLM #1 fallback to secondary provider (OpenAI → Anthropic)?
   - **Recommendation:** Phase 2 feature (adds complexity), start with single provider + Rust fallback

3. **Caching Strategy:** Opt-in or opt-out for LLM #1 result caching?
   - **Recommendation:** Opt-out (enabled by default), configurable via `DualLlmConfig.enable_caching`

4. **User Visibility:** Should LLM #1 state be exposed to users (for debugging/transparency)?
   - **Recommendation:** Phase 3 feature (add to API response metadata), not required for MVP

## Next Steps

1. **Review this design** with team (implementation feasibility, performance assumptions, model selection)
2. **Obtain API keys** for testing (OpenAI, Anthropic)
3. **Approve Phase 1 start** (5-7 days to working dual-LLM implementation)
4. **Begin Day 1:** Setup types, validation tests

---

**Contact:** LLM Architecture Expert
**Date:** 2025-11-01
**Design Document:** `/tmp/dual-llm-implementation/llm-architecture-design.md`
