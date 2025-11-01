# ARCHITECTURAL ASSUMPTIONS (PENDING TECH ARCHITECT REVIEW)

**Date:** 2025-11-01
**Phase:** 0-2A (Dual-LLM Infrastructure Complete)
**Status:** Awaiting Tech Architect validation
**Purpose:** Document architectural assumptions made in absence of Tech Architect specialist report

---

## CONTEXT

During multi-agent coordination for dual-LLM implementation planning, the Technical Architect specialist report was not available. Three other specialists (Product Manager, QA Guardian, Workflow Optimizer) proceeded with analysis based on the following architectural assumptions.

**Integration Manager Decision:** Proceed with Path C+ (Enhanced Hybrid: Integration Testing + Phase 1A) while documenting assumptions for later Tech Architect review.

**Risk Mitigation:** Integration tests will validate these assumptions empirically. If assumptions are incorrect, tests will fail early (small debugging surface).

---

## ASSUMPTION 1: FlowContext Integration Design

**Assumption:**
The `UnconscciousLlmProcessor` → `FlowContext` state update pattern (lines 286-318 in `src/modules/dual_llm/processors.rs`) is architecturally sound and correctly integrates LLM #1 output into the flow processing pipeline.

**Specific Details:**
- Domain activations from `Llm1Output.domain_recognitions` correctly map to `FlowContext.domains` (HashMap conversion)
- Boundary states from `Llm1Output.boundary_states` correctly update `FlowContext.boundaries`
- Pattern recognitions from `Llm1Output.pattern_recognitions` storage is DEFERRED (see Assumption 4)

**Validation Method:**
- Integration Test 1 (end-to-end MockLlm call) will validate FlowContext state updates
- Expected outcome: Domain/boundary values from MockLlm JSON correctly populate FlowContext

**Risk if Wrong:**
- **Severity:** MODERATE
- **Impact:** State update logic refactoring required
- **Mitigation:** Integration tests isolate issue to processors.rs (lines 286-318), easy to debug

**Tech Architect Review Needed:**
- ✅ Confirm FlowContext.domains HashMap structure is correct for LLM #1 integration
- ✅ Confirm boundary state update pattern follows architectural intent
- ✅ Review any concerns about state consistency during updates

---

## ASSUMPTION 2: DUAL_LLM_MODE Feature Flag Safety

**Assumption:**
Flipping `DUAL_LLM_MODE=true` in `.env` is safe to enable for testing without introducing breaking changes or unexpected side effects.

**Specific Details:**
- When `DUAL_LLM_MODE=true`: System uses `UnconscciousLlmProcessor` instead of `DomainEmergenceProcessor`
- When `DUAL_LLM_MODE=false`: System uses Rust calculators (classic behavior, current production mode)
- Feature flag toggle does NOT modify database schema, API contracts, or other system components

**Validation Method:**
- Integration Test 3 (feature flag toggle) will validate both modes work correctly
- Expected outcome: DUAL_LLM_MODE=false uses Rust, DUAL_LLM_MODE=true uses LLM #1 (MockLlm)

**Risk if Wrong:**
- **Severity:** LOW
- **Impact:** Feature flag behavior unexpected, easy to revert .env change
- **Mitigation:** Fallback to Rust is always available, feature flag can be toggled instantly

**Tech Architect Review Needed:**
- ✅ Confirm feature flag implementation is safe for testing
- ✅ Review any concerns about pipeline processor swapping (DomainEmergenceProcessor ↔ UnconscciousLlmProcessor)

---

## ASSUMPTION 3: StageProcessor Interface Usage

**Assumption:**
`UnconscciousLlmProcessor` correctly implements the `StageProcessor` trait and can be integrated into the `FlowProcess` pipeline without architectural changes.

**Specific Details:**
- `StageProcessor::process(&mut FlowContext)` signature is correct for LLM #1 integration
- Pipeline stage sequencing: UnconscciousLlmProcessor runs BEFORE LLM #2 call (to populate context)
- Error handling: `StageProcessor::process()` returns `Result<(), ProcessorError>` with proper error propagation

**Validation Method:**
- Integration Test 1 will validate (indirectly) that StageProcessor interface works in pipeline
- Expected outcome: MockLlm processor executes successfully in test harness

**Risk if Wrong:**
- **Severity:** MODERATE
- **Impact:** Interface refactoring required, pipeline integration rework
- **Mitigation:** Integration tests catch interface issues early, isolated to processor implementation

**Tech Architect Review Needed:**
- ✅ Confirm StageProcessor trait is the correct abstraction for LLM #1 integration
- ✅ Review pipeline stage ordering (when does UnconscciousLlmProcessor run relative to other stages?)
- ✅ Validate error handling strategy for LLM failures in pipeline

---

## ASSUMPTION 4: Pattern Storage Deferral (RESOLVED DURING SPRINT)

**Assumption:**
Removing `pattern_recognitions` from `Llm1Output` schema (or leaving it unpersisted) is acceptable for Phase 0-2A. Pattern lifecycle management will be implemented in Phase 3 when collective memory design is complete.

**Specific Details:**
- Line 311 in `processors.rs`: "TODO: Integrate pattern recognitions properly with developmental lifecycle"
- Current behavior: Pattern recognition data from LLM #1 is parsed but NOT stored in FlowContext
- **Recommended Resolution:** Remove `pattern_recognitions` from `Llm1Output` schema OR add `context.patterns` storage

**Decision Required During Sprint:**
- **Option A (RECOMMENDED):** Remove `pattern_recognitions` from schema (5 min, defers to Phase 3)
- **Option B:** Implement basic pattern storage in FlowContext (30 min, partial implementation)

**Validation Method:**
- Integration Test 5 will force resolution of this TODO
- Test will either validate pattern storage works OR validate schema no longer includes patterns

**Risk if Wrong:**
- **Severity:** LOW
- **Impact:** Schema-implementation mismatch creates confusion
- **Mitigation:** Easy to re-add pattern_recognitions in Phase 3 when pattern lifecycle designed

**Tech Architect Review Needed:**
- ✅ Confirm pattern storage can be deferred to Phase 3 (Collective Memory)
- ✅ Review whether partial pattern implementation (Option B) creates technical debt

---

## ASSUMPTION 5: MockLlm Validation Sufficiency

**Assumption:**
Integration testing with `MockLlm` (predetermined JSON responses) is sufficient validation for Phase 1A (Hot Memory) implementation. Real LLM testing (Llama 4 Maverick) can be deferred to Phase 2B when dual-LLM flow is complete.

**Specific Details:**
- MockLlm returns valid `Llm1Output` JSON (predetermined, realistic structure)
- MockLlm simulates timeout, auth errors, invalid JSON for fallback testing
- Real LLM testing deferred until: hot memory + LLM #1 processor + LLM #2 integration complete (Phase 2B)

**Rationale:**
- Product Manager's concern: Real LLM validation creates API dependency, breaks momentum
- QA Guardian's concern: Integration testing needed NOW, but architecture validation ≠ prompt validation
- Workflow Optimizer's concern: MockLlm is fast, no external dependencies, preserves flow state

**Validation Method:**
- Integration tests 1-5 use MockLlm to validate architecture integration
- Phase 2B will flip `DUAL_LLM_MODE=true` with real Llama 4 Maverick for prompt tuning

**Risk if Wrong:**
- **Severity:** MODERATE
- **Impact:** Prompt tuning needs or LLM-specific issues discovered late (Phase 2B instead of Phase 0)
- **Mitigation:** Product Manager's Path C already planned for prompt tuning in Phase 2B; deferral is intentional

**Tech Architect Review Needed:**
- ✅ Confirm MockLlm validation strategy is architecturally sound
- ✅ Review whether any integration concerns require real LLM testing earlier
- ✅ Validate Phase 2B timeline for real LLM validation is acceptable

---

## ASSUMPTION 6: Database Schema Completeness

**Assumption:**
The conversation memory database schema (Phase 0) is complete and correct for Phase 1A (Hot Memory) implementation. No schema migrations required.

**Specific Details:**
- `conversation_sessions` table: Correct for session tracking
- `conversation_turns` table: Correct for turn persistence, includes `memory_tier` column
- `memory_tier_transitions` table: Correct for tier transition tracking (Phase 1B/1C)
- `conversation_summaries` table: Correct for warm memory (Phase 1B)

**Validation Method:**
- Phase 1A will exercise `conversation_sessions` and `conversation_turns` tables
- Database integration tests will validate schema works for hot memory loading/saving

**Risk if Wrong:**
- **Severity:** LOW-MODERATE
- **Impact:** Schema migration required during Phase 1A (disrupts timeline)
- **Mitigation:** Schema designed by Memory Systems specialist (design doc validated), migration system in place

**Tech Architect Review Needed:**
- ✅ Confirm database schema design aligns with overall architecture
- ✅ Review any concerns about schema completeness for Phases 1A-1C

---

## ASSUMPTION 7: Retry/Timeout Configuration

**Assumption:**
The retry logic (3 attempts, exponential backoff: 1s, 2s, 4s) and timeout (5000ms) for LLM #1 calls are appropriate values for production use.

**Specific Details:**
- Max retries: 3 (lines 160-230 in processors.rs)
- Backoff: 1s, 2s, 4s (exponential)
- Timeout: 5000ms per call (configurable via UNCONSCIOUS_LLM_TIMEOUT_MS)
- Auth errors: No retry (fail immediately)

**Validation Method:**
- Integration Test 2 (fallback on timeout) validates timeout behavior
- Integration Test 7 (retry logic, from QA Guardian Appendix A) validates exponential backoff

**Risk if Wrong:**
- **Severity:** LOW
- **Impact:** Timeout/retry values tuned during Phase 2B real LLM testing
- **Mitigation:** Values are configurable via environment variables, easy to adjust

**Tech Architect Review Needed:**
- ✅ Confirm retry/timeout values are reasonable defaults
- ✅ Review whether timeout should be higher for free-tier LLMs (Llama 4 Maverick)

---

## SUMMARY FOR TECH ARCHITECT REVIEW

**Total Assumptions:** 7 (all documented above)

**Critical Assumptions (require review):**
1. FlowContext Integration Design (MODERATE risk if wrong)
2. StageProcessor Interface Usage (MODERATE risk if wrong)
3. MockLlm Validation Sufficiency (MODERATE risk if wrong)

**Low-Risk Assumptions (nice to validate, not blocking):**
4. DUAL_LLM_MODE Feature Flag Safety
5. Pattern Storage Deferral
6. Database Schema Completeness
7. Retry/Timeout Configuration

**Mitigation Strategy:**
- Integration tests (5 tests, 4-6h sprint) will validate assumptions empirically
- If tests pass: Assumptions likely correct
- If tests fail: Issues isolated early (small debugging surface)
- Tech Architect review can happen asynchronously (not blocking critical path)

**Request to Tech Architect:**
When Tech Architect specialist report becomes available, please review these 7 assumptions and flag any concerns. Integration Manager will incorporate feedback and adjust Path C+ execution if needed.

---

**Document Status:** DRAFT (pending Tech Architect review)
**Author:** Integration Manager
**Date:** 2025-11-01
**Related Reports:**
- `/home/emzi/Projects/recursive-light/api/coordination-workspace/integration-report.md`
- `/home/emzi/Projects/recursive-light/api/coordination-workspace/product-manager-report.md`
- `/home/emzi/Projects/recursive-light/api/coordination-workspace/qa-guardian-report.md`
- `/home/emzi/Projects/recursive-light/api/coordination-workspace/workflow-optimizer-report.md`

---

**END OF ARCHITECTURAL ASSUMPTIONS DOCUMENT**
