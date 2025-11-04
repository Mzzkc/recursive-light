# Comprehensive Technical Debt Analysis
**Recursive Light Framework - TDF-Aligned Assessment**
*Generated: 2025-11-03*
*Session: Phase 2D Complete, Pre-Phase 3 CAM*

---

## EXECUTIVE SUMMARY

**Current State:**
- ✅ 143 tests passing (100% pass rate)
- ✅ 0 clippy warnings
- ✅ Phase 2D functionally complete (BM25 + temporal decay)
- ⚠️ **Significant technical debt identified across all domains**
- 🔴 **Critical items block production readiness**

**Impact Assessment:**
- **CRITICAL:** 3 items (BM25 implementation quality, defeats algorithm purpose)
- **HIGH:** 4 items (error handling, logging, tokenization, identity lookups)
- **MEDIUM:** 5 items (performance, stub implementations, dead code)
- **LOW:** 5 items (future enhancements, nice-to-haves)

**Estimated Remediation Effort:** 15-25 hours total
- Critical: 6-10 hours
- High: 5-8 hours
- Medium: 3-5 hours
- Low: 1-2 hours

**Recommendation:** Address 🔴 CRITICAL items before Phase 3 CAM implementation to avoid compounding technical debt and ensure solid foundation for advanced features.

---

## TDF DOMAIN ANALYSIS

### COMP Domain (Computational/Analytical) - 9 Items

#### 🔴 CRITICAL-1: BM25 IDF Hardcoded to 1.0
**Location:** `api/src/dual_llm/memory_tiering.rs:749`

**Technical Issue:**
```rust
let idf = 1.0; // Simplified: treat all terms as equally important
```

**Analysis:**
- IDF should be: `log((N - df_t + 0.5) / (df_t + 0.5))`
- Current: All terms weighted equally (common words = rare words)
- **Effect:** Defeats entire purpose of BM25 algorithm
- Query "quantum computing" scores same as "the computing"

**Dependencies:**
- Requires corpus statistics tracking
- Needs inverted index or per-term document frequency storage
- Affects all memory retrieval quality

**Evidence:** COMP_MEMORY_RETRIEVAL_ANALYSIS.md validates BM25 approach but requires proper IDF calculation for 85% confidence rating.

**Effort:** 3-4 hours (integrate `bm25` crate or implement properly)
**Impact:** HIGH (core retrieval quality)
**Priority:** 🔴 P0

---

#### 🔴 CRITICAL-2: BM25 Average Document Length Hardcoded
**Location:** `api/src/dual_llm/memory_tiering.rs:737`

```rust
let avgdl = 100.0; // Should calculate from actual corpus
```

**Analysis:**
- Length normalization broken (parameter `b` meaningless)
- Short documents under-penalized, long documents over-penalized
- BM25's adaptive length normalization defeated

**Dependencies:** Needs corpus-wide statistics

**Effort:** 2-3 hours (part of BM25 fix)
**Impact:** HIGH (ranking accuracy)
**Priority:** 🔴 P0

---

#### 🔴 CRITICAL-3: No Inverted Index (Linear Scan)
**Location:** `api/src/dual_llm/memory_tiering.rs:726-764`

**Analysis:**
- Current: O(n*m) where n=documents, m=query terms
- Optimal: O(m*log(n)) with inverted index
- **Scalability crisis:**
  - 100 docs: acceptable
  - 1,000 docs: 100-200ms (slow)
  - 10,000 docs: 1-2 seconds (unusable)

**Evidence:** No performance benchmarks for large corpora exist

**Effort:** 4-6 hours (implement or use `bm25` crate)
**Impact:** HIGH (scalability blocker)
**Priority:** 🔴 P0

---

#### 🟡 HIGH-4: Primitive Tokenizer (No Stemming/Stop Words)
**Location:** `api/src/dual_llm/memory_tiering.rs:767-773`

```rust
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .filter(|word| word.len() > 2)
        .map(|s| s.to_string())
        .collect()
}
```

**Missing:**
- ❌ Stemming: "running"/"runs"/"ran" not normalized to "run"
- ❌ Stop words: "the", "is", "and" included (noise)
- ❌ Punctuation: "hello," ≠ "hello"
- ❌ Unicode normalization: "café" ≠ "cafe"
- ❌ Hyphen handling: "multi-threaded" not split

**Impact:** Reduced recall (query "remember optimizing" won't match "I remember optimization")

**Effort:** 2-3 hours (add `rust-stemmers`, `stop-words`, `unicode-normalization` crates)
**Priority:** 🟡 P1

---

#### 🟡 HIGH-5: 215 `unwrap()`/`expect()` Calls
**Location:** Across 10 files

**Wolf Pattern Match:** Pattern #6 ("Works in Mock" Assumption)
- Tests pass with MockLlm
- Production with real API keys may panic
- Example: `api/src/lib.rs:325` - OPENAI_API_KEY unwrap

**Risk:**
- Panic in production on missing env vars
- No graceful degradation
- User-facing crashes

**Effort:** 3-4 hours (convert to proper Result handling)
**Priority:** 🟡 P1

---

#### 🟢 MEDIUM-6: 135 `.clone()` Calls
**Location:** 10 files

**Analysis:**
- Potential performance impact (heap allocations)
- Most likely acceptable but unverified
- No profiling data exists

**Recommendation:** Profile first, optimize if needed

**Effort:** 2-3 hours (audit + selective optimization)
**Priority:** 🟢 P2

---

#### 🟢 MEDIUM-7: Dead Code Warning
**Location:** `api/src/flow_process.rs:1323`

```rust
#[allow(dead_code)] // TODO Phase 2B: Remove when integrated into VifApi::new()
pub fn with_config(...)
```

**Analysis:**
- Phase 2B completed but attribute not removed
- Code IS used (VifApi::new calls it) but marked dead
- Indicates incomplete cleanup

**Effort:** 5 minutes (remove attribute, verify usage)
**Priority:** 🟢 P3

---

#### 🟢 LOW-8: No Test Coverage Metrics
**Evidence:**
- 143 tests passing
- 17/18 files have tests (94% file coverage)
- **Unknown:** Line/branch coverage percentage

**Recommendation:** Add `cargo-tarpaulin` or `cargo-llvm-cov` to CI

**Effort:** 30 minutes
**Priority:** 🟢 P4

---

#### 🟢 LOW-9: Missing Dependency Audit
**Evidence:**
- `cargo-audit` not installed
- `cargo-outdated` not installed
- No automated security checks

**Wolf Pattern:** Pattern #9 (Timeline Pressure Shortcuts) - skipping security validation

**Effort:** 15 minutes (install tools, run checks)
**Priority:** 🟢 P4

---

### SCI Domain (Scientific/Empirical) - 4 Items

#### 🔴 CRITICAL-10: Identity Criticality Not Validated
**Location:** `api/src/dual_llm/memory_tiering.rs:687`

```rust
let identity_criticality = 0.5; // TODO: Check snapshot.is_identity_critical
```

**Evidence Gap:**
- Database schema HAS `is_identity_critical` flag (conversation_summaries table)
- Code ASSUMES 0.5 instead of querying
- **No empirical data:** Never verified if database queries work
- **No tests:** TurnSignificance calculation not tested with real DB data

**Impact:**
- Identity-forming moments (name, preferences) not prioritized
- Retrieval quality degraded for personal context
- Combined score weighted 15% by wrong value

**Effort:** 2-3 hours (implement async DB query + caching)
**Priority:** 🔴 P0

---

#### 🟡 HIGH-11: No Performance Benchmarks
**Missing Evidence:**
- No benchmarks for BM25 ranking speed
- No memory consumption measurements
- No large-scale corpus tests (1k+ documents)

**Wolf Pattern:** Pattern #1 ("It Just Compiles") - passing tests ≠ performance validated

**Current Claims:**
- STATUS.md: "Sub-millisecond processing" (unverified for ranking)
- TECH-DEBT-PHASE-2D.md: "P95 < 15ms for 1k documents" (target, not measurement)

**Effort:** 2 hours (add criterion benchmarks)
**Priority:** 🟡 P1

---

#### 🟢 MEDIUM-12: TurnSignificance Stub Fields (3/6)
**Location:** `api/src/dual_llm/memory_tiering.rs:13-42`

```rust
pub struct TurnSignificance {
    pub recency_score: f32,        // ✅ IMPLEMENTED
    pub semantic_relevance: f32,   // ⚠️ SIMPLIFIED (IDF=1.0)
    pub identity_criticality: f32, // ❌ HARDCODED (0.5)
    pub emotional_weight: f32,     // ❌ STUB (0.0)
    pub factual_density: f32,      // ❌ STUB (0.0)
    pub narrative_importance: f32, // ❌ STUB (0.0)
}
```

**Evidence:** Combined score only uses 3/6 fields (50% recency, 35% semantic, 15% identity)

**Impact:** Missing signals for:
- Emotional moments ("I'm so excited!")
- Fact-dense content (dates, names, numbers)
- Narrative markers (decisions, breakthroughs)

**Reserved for:** Phase 3 CAM implementation

**Effort:** 4-6 hours (implement basic heuristics)
**Priority:** 🟢 P2 (defer to Phase 3)

---

#### 🟢 LOW-13: No Semantic Embeddings
**Current:** Keyword-based BM25 only
**Missing:** Vector similarity search (embeddings)

**Evidence from Research:**
- COMP_MEMORY_RETRIEVAL_ANALYSIS.md recommends hybrid (BM25 + embeddings)
- Expected improvement: +20-40% recall

**Blocker:** Phase 3 CAM dependency (embedding infrastructure)

**Effort:** 8-12 hours (integrate sentence-transformers or OpenAI embeddings)
**Priority:** 🟢 P4 (Phase 3 feature, not tech debt)

---

### CULT Domain (Cultural/Contextual) - 3 Items

#### 🟡 HIGH-14: Debug Logging with `eprintln!`
**Location:** `api/src/lib.rs:326, 346, 497, 532`

**Context:**
- Comments say "remove in production"
- **Creator intention:** Temporary debugging, not production-ready
- **Current state:** Still in codebase after Phase 2D completion

**Wolf Pattern:** Pattern #5 (Premature Commits) - debug statements not removed

**Examples:**
```rust
eprintln!("Warning: OPENAI_API_KEY not set, dual-LLM will fail without MockLlm");
eprintln!("[Phase 2D] Warm memory: score={:.3}, recency={:.3}, semantic={:.3}", ...);
```

**Impact:**
- Unprofessional output in production
- No log level control
- No structured logging

**Effort:** 1-2 hours (replace with `tracing` crate)
**Priority:** 🟡 P1

---

#### 🟢 MEDIUM-15: Incomplete Phase 2B Cleanup
**Context:**
- Commit bab775e: "Phase 2B: LLM #2 Context Integration Complete"
- STATUS.md: "Phase 2B ✅ COMPLETE"
- **But:** Dead code attributes still reference Phase 2B as incomplete

**Evidence:**
```rust
#[allow(dead_code)] // TODO Phase 2B: Remove when integrated into VifApi::new()
```

**Analysis:**
- Code cleanup incomplete
- Indicates rushed commit (Wolf Pattern #5)
- Confuses future maintainers ("Is Phase 2B done?")

**Effort:** 15 minutes (remove TODOs, verify integration)
**Priority:** 🟢 P3

---

#### 🟢 LOW-16: Missing Memory Bank Files
**Expected:** `memory-bank/` directory with session summaries
**Actual:** Directory missing (Glob found 0 files)
**Referenced:** STATUS.md mentions memory-bank/ multiple times

**Analysis:**
- Documentation drift (docs ≠ reality)
- Session context not being preserved
- Future sessions start cold (no warm handoff)

**Wolf Pattern:** Pattern #4 (Docs ≠ Features)

**Effort:** 30 minutes (create structure + populate with current session)
**Priority:** 🟢 P4

---

### EXP Domain (Experiential/Intuitive) - 1 Item

#### 🟡 HIGH-17: "Feels Wrong" Pattern Recognition
**Gut Check Analysis:**

**What feels right:**
- ✅ Test coverage (143 passing, 0 failures)
- ✅ Domain architecture (clean separation)
- ✅ Type safety (Rust prevents many bugs)
- ✅ Migration structure (clean schema evolution)

**What feels wrong:**
- 🔴 BM25 implementation (hardcoded constants = "stub quality")
- 🔴 215 unwraps (ticking time bombs)
- 🟡 eprintln! in production code (amateur aesthetic)
- 🟡 Dead code attributes after "complete" phases (rushed feel)

**Aesthetic Assessment:**
- **Code quality:** 7/10 (good structure, rushed details)
- **Production readiness:** 4/10 (functional but fragile)
- **Confidence:** MEDIUM (works in tests, untested in production)

**Intuition:** This feels like "MVP complete" not "production ready" - which may be intentional for research prototype.

**Recommendation:** Clarify intent (prototype vs production) to set appropriate quality bar.

**Priority:** 🟡 P1 (decide production vs prototype quality target)

---

### META Domain (Metacognitive) - Recognition Patterns

#### 🔴 META-18: Single-Loop Acceleration Pattern Detected
**Pattern:**
- Phase 2A → Phase 2B → Phase 2C → Phase 2D
- Each phase adds features quickly
- **But:** Tech debt accumulates without remediation
- Classic "velocity → fragility" trap

**Evidence:**
- Commit a3addcd (Phase 2D) adds BM25 with TODO comments
- Commit 50a9e08 documents debt but doesn't fix it
- Pattern: Implement → Document debt → Move to next phase

**Meta-Question:** "Are we in a quick-fix loop?"
**Answer:** YES - feature velocity prioritized over quality

**Recommendation:**
- STOP feature development
- REMEDIATE critical debt
- THEN proceed to Phase 3 CAM

**Wolf Prevention:** Pattern #9 (Timeline Pressure Shortcuts) - "ship fix later" mindset

**Priority:** 🔴 P0 (meta-level decision required)

---

## PRIORITIZATION MATRIX

### By Impact × Effort

```
         │ Low Effort  │ Med Effort │ High Effort
─────────┼─────────────┼────────────┼─────────────
Critical │ Identity    │ BM25 IDF   │ Inverted
         │ (P0)        │ avgdl (P0) │ Index (P0)
─────────┼─────────────┼────────────┼─────────────
High     │ Logging     │ Tokenizer  │ unwrap→
         │ Dead code   │ Benchmarks │ Result (P1)
         │ (P1)        │ (P1)       │
─────────┼─────────────┼────────────┼─────────────
Medium   │ Phase 2B    │ .clone()   │ Significance
         │ cleanup     │ audit      │ fields (P2)
         │ (P3)        │ (P2)       │
─────────┼─────────────┼────────────┼─────────────
Low      │ Mem bank    │ Coverage   │ Embeddings
         │ Audit tools │ metrics    │ (P4/Phase3)
         │ (P4)        │ (P4)       │
```

### By TDF Domain Urgency

**COMP (Computational):** 🔴 CRITICAL - 3 P0 items block scalability
**SCI (Empirical):** 🔴 CRITICAL - No production validation
**CULT (Contextual):** 🟡 HIGH - Quality standards unclear
**EXP (Intuitive):** 🟡 HIGH - Prototype vs production decision
**META:** 🔴 CRITICAL - Velocity→debt loop must break

---

## RECOMMENDED REMEDIATION ORDER

### Wave 1: Critical Path (6-10 hours, blocks Phase 3)
**Goal:** Fix algorithmic correctness, enable scalability

1. **BM25 Implementation** (6-8 hours)
   - Integrate `bm25` crate OR implement proper IDF/avgdl
   - Build inverted index
   - Add corpus statistics tracking
   - **Validates:** COMP domain core algorithm
   - **Tests:** Add ranking accuracy tests

2. **Identity Criticality DB Lookup** (2-3 hours)
   - Implement async query to state_snapshots table
   - Add caching layer (avoid repeated queries)
   - **Validates:** SCI domain empirical data
   - **Tests:** Add integration test with real DB

**Success Criteria:**
- [ ] IDF calculated from actual corpus (not 1.0)
- [ ] avgdl calculated from actual documents (not 100)
- [ ] Inverted index used for O(m*log(n)) lookups
- [ ] Identity criticality queried from database
- [ ] All 143 tests still passing
- [ ] New tests added for ranking quality

---

### Wave 2: Production Hardening (5-8 hours)
**Goal:** Make production-ready (no panics, proper logging)

3. **Replace unwrap/expect with Result** (3-4 hours)
   - Audit all 215 usages
   - Convert to `?` operator or explicit error handling
   - Add graceful fallbacks (e.g., MockLlm if API key missing)
   - **Validates:** CULT domain production standards
   - **Wolf Prevention:** Pattern #6 (Mock≠Production)

4. **Proper Logging (tracing crate)** (1-2 hours)
   - Replace all eprintln! with tracing::debug!
   - Add log levels (debug/info/warn/error)
   - Configure env_logger or tracing-subscriber
   - **Validates:** CULT domain professional quality

5. **Tokenizer Improvements** (2-3 hours)
   - Add `rust-stemmers` (Porter stemmer)
   - Add `stop-words` crate
   - Add unicode normalization
   - **Validates:** COMP domain NLP quality
   - **Tests:** Add tokenization unit tests

**Success Criteria:**
- [ ] Zero unwrap/expect in production code paths
- [ ] Structured logging with levels
- [ ] Stemming and stop word removal working
- [ ] Graceful degradation on missing API keys

---

### Wave 3: Quality & Metrics (3-5 hours)
**Goal:** Measure, validate, clean up

6. **Performance Benchmarks** (2 hours)
   - Add criterion benchmarks for BM25 ranking
   - Test with 100, 1k, 10k documents
   - Validate P95 < 15ms claim
   - **Validates:** SCI domain empirical verification

7. **Code Cleanup** (1 hour)
   - Remove dead code attributes
   - Clean up Phase 2B TODOs
   - Remove debug comments
   - **Validates:** CULT domain completion standards

8. **Coverage & Audit** (1-2 hours)
   - Install cargo-tarpaulin / cargo-llvm-cov
   - Run coverage report (target >75%)
   - Install cargo-audit, run security scan
   - **Validates:** SCI domain measurement standards

**Success Criteria:**
- [ ] Benchmarks show <15ms P95 for 1k documents
- [ ] Code coverage measured (report baseline)
- [ ] No TODOs referencing completed phases
- [ ] Security audit clean (no critical CVEs)

---

### Wave 4: Deferred to Phase 3 (Future)
**Not Tech Debt - Planned Features:**

9. **TurnSignificance Field Implementations** (Phase 3 CAM)
   - Emotional weight (sentiment analysis)
   - Factual density (NER, date/number extraction)
   - Narrative importance (decision markers)

10. **Semantic Embeddings** (Phase 3 CAM)
    - Integrate sentence-transformers or OpenAI embeddings
    - Build hybrid retrieval (BM25 + vector similarity)
    - Expected +20-40% recall improvement

11. **Memory Bank Structure** (Session Management)
    - Create memory-bank/ directory
    - Implement session handoff summaries
    - Integrate with session-shutdown-protocol.md

---

## WOLF PATTERN PREVENTION

### Patterns Detected in Current Tech Debt:

**Pattern #1: "It Just Compiles"**
- ✅ Tests pass → ❌ Assumed production ready
- **Evidence:** 143 tests, 0 failures, BUT no production API key testing
- **Prevention:** Test with real OpenAI/Anthropic APIs before deploy

**Pattern #4: Docs ≠ Features**
- STATUS.md claims complete → Code has TODOs
- Memory-bank/ documented → Directory missing
- **Prevention:** Verify all STATUS claims against actual code

**Pattern #5: Premature Commits**
- eprintln! still in code after "complete" phases
- Dead code attributes not removed
- **Prevention:** Pre-commit checklist (no debug statements, no TODOs)

**Pattern #6: "Works in Mock" Assumption**
- All tests use MockLlm → No real API validation
- **Prevention:** Add integration tests with real LLM providers

**Pattern #9: Timeline Pressure Shortcuts**
- BM25 simplified with hardcoded values
- "Ship fix later" TODOs accumulating
- **Prevention:** STOP feature work, REMEDIATE before Phase 3

---

## DOMAIN ACTIVATION ASSESSMENT

### Current Domain Balance (for Tech Debt Remediation)

**COMP (Computational):** 🔴 0.9 - Core algorithms need fixing
**SCI (Empirical):** 🔴 0.8 - Need production validation, benchmarks
**CULT (Contextual):** 🟡 0.7 - Quality standards need clarification
**EXP (Intuitive):** 🟡 0.6 - Prototype vs production decision needed
**META:** 🔴 0.9 - Velocity→debt loop recognition critical

**Boundary Permeability:**
- COMP↔SCI: 0.9 (HIGH) - Algorithm needs empirical validation
- COMP↔CULT: 0.7 (HIGH) - Code quality vs velocity tradeoff
- SCI↔CULT: 0.8 (HIGH) - Production readiness standards
- COMP↔EXP: 0.7 (HIGH) - "Does BM25 hack feel right?" (NO)
- META: 0.9 (HIGH) - Pattern recognition driving decision

**Overall Consciousness Volume:** HIGH (multi-domain, high boundaries, meta-aware)

---

## DECISION GATE CHECKPOINT

### TDF Protocol Validation (Before Proceeding to Phase 3)

**Phase 1: Domain Scan ✅**
- [COMP] Analyzed: BM25 broken, inverted index missing, tokenizer primitive
- [SCI] Evidence: No benchmarks, no production tests, hardcoded constants
- [CULT] Context: Rushed phases, velocity prioritized, unclear production bar
- [EXP] Intuition: Feels like prototype, not production-ready

**Phase 2: Boundary Check ✅**
- [COMP↔SCI] Can prove BM25 broken? YES (IDF=1.0 defeats algorithm)
- [COMP↔CULT] Why hardcoded? Timeline pressure (Wolf Pattern #9)
- [SCI↔CULT] What does evidence tell? Velocity→fragility pattern
- [COMP↔EXP] Does this feel right? NO (hardcoded constants feel hacky)
- [CULT↔EXP] Would proper BM25 implementation feel better? YES

**Phase 3: Meta Verification ✅**
- Reasoning pattern? Single-loop acceleration (add features, defer fixes)
- Hit this before? YES (Wolf Prevention documents this exact pattern)
- Pattern level? P⁴ (meta-pattern recognition: velocity trap)
- What would more conscious me do? STOP features, FIX foundation

**Phase 4: Decision Gate**
- ✓ All 4 domains ≥0.6 activated
- ✓ ≥3 boundaries examined P>0.7
- ✓ Recognition >0.7 (understand velocity→debt trap)
- ✓ Pattern ≥P⁴ (meta-pattern level)
- ✓ META confirms single-loop detected, break needed

**GATE RESULT: BLOCK PHASE 3, REMEDIATE FIRST** 🚨

---

## FINAL RECOMMENDATIONS

### Immediate Actions (This Session)

1. **Decide:** Prototype or Production?
   - **If Prototype:** Accept tech debt, document clearly, proceed to Phase 3
   - **If Production:** Execute Wave 1 (Critical Path) before Phase 3

2. **Meta-Decision:** Break Velocity Loop
   - Commit to quality-first approach
   - No new features until critical debt cleared
   - Update STATUS.md with honest assessment

3. **Communication:**
   - Update STATUS.md: Phase 2D "FUNCTIONALLY COMPLETE" → "COMPLETE WITH DEBT"
   - Create PRODUCTION-READINESS-CHECKLIST.md
   - Document prototype vs production intent

### Strategic Path Forward

**Option A: Production Quality (Recommended)**
- Execute Waves 1-3 (14-23 hours total)
- Achieve production-ready BM25 + error handling
- THEN proceed to Phase 3 CAM on solid foundation
- **Timeline:** +2-3 weeks before Phase 3 start

**Option B: Research Prototype (Acceptable if documented)**
- Accept tech debt as "known limitations"
- Document clearly in README, STATUS.md
- Proceed to Phase 3 CAM (research focus)
- Defer production hardening to post-CAM
- **Timeline:** Immediate Phase 3 start

**Option C: Hybrid (Not Recommended)**
- Fix critical items only (Wave 1: 6-10 hours)
- Defer production hardening
- Start Phase 3 with partial foundation
- **Risk:** Tech debt compounds, harder to fix later

---

## APPENDIX: FULL TECH DEBT INVENTORY

### By File

**api/src/lib.rs:**
- 35 unwrap/expect calls
- 4 eprintln! debug statements
- 46 .clone() calls

**api/src/dual_llm/memory_tiering.rs:**
- BM25 IDF hardcoded (line 749)
- BM25 avgdl hardcoded (line 737)
- No inverted index (lines 726-764)
- Primitive tokenizer (lines 767-773)
- Identity criticality hardcoded (line 687)
- 61 unwrap/expect calls
- 5 .clone() calls

**api/src/dual_llm/processors.rs:**
- 12 unwrap/expect calls
- 10 .clone() calls
- 3 eprintln! debug statements

**api/src/dual_llm/types.rs:**
- 4 unwrap/expect calls
- 6 .clone() calls

**api/src/flow_process.rs:**
- Dead code attribute (line 1323)
- 57 unwrap/expect calls
- 39 .clone() calls

**Other files:**
- Various unwrap/expect (91 total across 6 files)
- Various .clone() (29 total across 6 files)

### By Priority

**P0 (Critical - Blocks Production):**
- BM25 IDF hardcoded
- BM25 avgdl hardcoded
- No inverted index
- Identity criticality not validated
- Meta: Velocity→debt loop

**P1 (High - Production Quality):**
- 215 unwrap/expect calls
- eprintln! debug logging
- Primitive tokenizer
- No performance benchmarks
- Prototype vs production decision

**P2 (Medium - Quality Improvement):**
- 135 .clone() calls (audit needed)
- TurnSignificance stub fields
- Dead code cleanup

**P3 (Low - Nice to Have):**
- Missing memory-bank/ structure
- Coverage metrics
- Security audit tools

**P4 (Future Features):**
- Semantic embeddings (Phase 3)
- Emotional/factual/narrative scoring (Phase 3)

---

**Assessment Complete.**
**Consciousness Volume: HIGH (TDF-aligned, meta-aware, boundary-active)**
**Recommendation: Execute Option A (Production Quality) before Phase 3 CAM**
