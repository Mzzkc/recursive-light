# Technical Debt: Phase 2D Implementation
*Created: 2025-11-03*
*Status: Documented, remediation planned for next session*

## Overview

Phase 2D implemented BM25 + temporal decay ranking, but with **significant simplifications and stub code**. This document catalogs all tech debt for proper remediation.

---

## Critical Tech Debt Items

### 1. **BM25 Implementation - Severely Simplified** 🔴

**Location:** `api/src/dual_llm/memory_tiering.rs:726-764`

**Issues:**

#### a) IDF (Inverse Document Frequency) - HARDCODED
```rust
// Line 749: IDF hardcoded to 1.0
let idf = 1.0; // Simplified: treat all terms as equally important
```

**Problem:** IDF should be calculated from corpus statistics:
```
IDF(term) = log((N - df_t + 0.5) / (df_t + 0.5))
where:
  N = total number of documents in corpus
  df_t = number of documents containing term t
```

**Impact:**
- Common words (e.g., "the", "is", "and") scored equally with rare words (e.g., "quantum", "hypergraph")
- No term discrimination - defeats the purpose of BM25
- Essentially reduces to just TF (term frequency) scoring

**Real-world example:**
- Query: "quantum computing optimization"
- Current: "the optimization" scores same as "quantum optimization"
- Correct: "quantum optimization" should score much higher

---

#### b) Average Document Length - HARDCODED
```rust
// Line 737: avgdl hardcoded to 100 tokens
let avgdl = 100.0;
```

**Problem:** Should calculate actual average:
```rust
avgdl = total_tokens_in_corpus / total_documents
```

**Impact:**
- Document length normalization is incorrect
- Short documents under-penalized
- Long documents over-penalized
- BM25's length normalization component (parameter `b`) is meaningless

---

#### c) No Corpus Statistics
**Problem:** BM25 requires maintaining corpus-wide statistics:
- Total document count (N)
- Per-term document frequencies (df_t)
- Average document length (avgdl)
- Total term counts

**Current state:** None of these exist. Algorithm recalculates on every query.

**Impact:**
- Slow: O(n*m) where n=documents, m=query terms
- Inaccurate: No corpus-level term discrimination

---

### 2. **Tokenizer - Too Simplistic** 🟡

**Location:** `api/src/dual_llm/memory_tiering.rs:767-773`

```rust
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .filter(|word| word.len() > 2) // Filter very short words
        .map(|s| s.to_string())
        .collect()
}
```

**Missing features:**

#### a) No Stemming
- "running", "runs", "ran" → should all map to "run"
- "optimization", "optimized", "optimize" → should map to "optim"
- **Impact:** Query "remember optimization" won't match "I remember optimizing"

#### b) No Stop Word Removal
- Common words like "the", "is", "and", "a", "an", "to", "in" are included
- **Impact:** Noise in term frequency calculations, slower matching

#### c) No Punctuation Handling
- "hello," and "hello" treated as different tokens
- "it's" stays as "it's" (should become "its" or split to "it" + "is")
- **Impact:** Missed matches due to punctuation differences

#### d) No Unicode Normalization
- "café" vs "cafe" treated as different
- Accents, diacritics not normalized
- **Impact:** Multi-language support broken

#### e) No Hyphen/Compound Word Handling
- "multi-threaded" → stays as "multi-threaded"
- Should split to ["multi", "threaded"] or keep as single token
- **Impact:** Inconsistent matching

---

### 3. **TurnSignificance - 3/6 Fields Are Stubs** 🟡

**Location:** `api/src/dual_llm/memory_tiering.rs:13-42`

```rust
pub struct TurnSignificance {
    pub recency_score: f32,        // ✅ IMPLEMENTED
    pub semantic_relevance: f32,   // ⚠️ SIMPLIFIED (see BM25 issues)
    pub identity_criticality: f32, // ❌ HARDCODED TO 0.5 (stub)
    pub emotional_weight: f32,     // ❌ ALWAYS 0.0 (stub)
    pub factual_density: f32,      // ❌ ALWAYS 0.0 (stub)
    pub narrative_importance: f32, // ❌ ALWAYS 0.0 (stub)
}
```

**Combined score weights:**
```rust
0.5 * recency_score +
0.35 * semantic_relevance +  // Broken due to IDF=1.0
0.15 * identity_criticality  // Hardcoded, not checked
```

#### a) Identity Criticality - Hardcoded
```rust
// Line 689
let identity_criticality = 0.5; // TODO: Check snapshot.is_identity_critical
```

**Should do:**
```rust
let identity_criticality = if turn.snapshot_id.is_some() {
    // Query database to check if snapshot is identity-critical
    check_identity_critical(turn.snapshot_id).await.unwrap_or(0.5)
} else {
    0.3 // Default: not identity-critical
};
```

**Impact:** Identity-forming moments (name reveals, profession, key facts) not prioritized

---

#### b) Emotional Weight - Not Implemented
**Reserved for Phase 3**

Should detect:
- High emotion: "I'm so excited!", "This is frustrating", "I love this"
- Neutral: "The function returns a value"

**Potential implementation:**
- Sentiment analysis library (e.g., `vader_sentiment` in Python via FFI)
- Simple heuristic: exclamation marks, capitalization, emotion words
- LLM-based: Ask LLM #1 to score emotional intensity

---

#### c) Factual Density - Not Implemented
**Reserved for Phase 3**

Should detect:
- Proper nouns: "Alice", "React", "PostgreSQL"
- Dates: "2025-11-03", "last Tuesday", "three weeks ago"
- Numbers: "15 tests", "3.5 seconds", "$50"
- Specific facts vs vague statements

**Potential implementation:**
- NER (Named Entity Recognition) library
- Regex patterns for dates/numbers
- POS tagging to count nouns

---

#### d) Narrative Importance - Not Implemented
**Reserved for Phase 3**

Should detect:
- First mentions: "I'm starting a new project"
- Decisions: "I decided to use Rust"
- Milestones: "We finished the implementation"
- Breakthroughs: "I finally figured out the bug"

**Potential implementation:**
- Keyword patterns: "decided", "starting", "finished", "finally"
- Turn position: First turn in session = higher importance
- Topic shift detection

---

### 4. **No Inverted Index** 🟡

**Current state:** Linear scan through all documents on every query.

**Performance:**
```
Current: O(n * m) where n=documents, m=query terms
With index: O(m * log(n))
```

**Impact:**
- 100 documents: ~acceptable
- 1,000 documents: slow (100-200ms)
- 10,000 documents: unacceptable (1-2 seconds)

**Required:** Inverted index data structure:
```rust
struct InvertedIndex {
    // term -> [(doc_id, term_frequency, positions)]
    index: HashMap<String, Vec<(Uuid, u32, Vec<usize>)>>,
    // doc_id -> document_length
    doc_lengths: HashMap<Uuid, usize>,
    // Corpus statistics
    total_docs: usize,
    avg_doc_length: f32,
}
```

---

### 5. **Debug Logging in Production Code** 🟢

**Location:** `api/src/lib.rs:497-502`, `532-537`

```rust
eprintln!(
    "[Phase 2D] Warm memory: score={:.3}, recency={:.3}, semantic={:.3}",
    significance.combined_score(),
    significance.recency_score,
    significance.semantic_relevance
);
```

**Issue:** Uses `eprintln!()` for debug output

**Should use:** Proper logging crate (e.g., `tracing`, `log`)
```rust
tracing::debug!(
    score = %significance.combined_score(),
    recency = %significance.recency_score,
    semantic = %significance.semantic_relevance,
    "Warm memory retrieval"
);
```

---

## Remediation Plan

### Phase 1: Fix BM25 (High Priority) 🔴

**Option A: Use existing crate**
- **Recommendation:** `bm25` crate (https://crates.io/crates/bm25)
- Pros: Production-ready, maintains corpus stats, proper IDF
- Cons: Need to adapt to our database model

**Option B: Implement properly from scratch**
- Build inverted index
- Calculate real IDF from conversation_turns table
- Maintain corpus statistics
- Pros: Full control, no dependencies
- Cons: 300-500 LOC, more testing needed

**Estimated effort:** 6-8 hours

---

### Phase 2: Improve Tokenizer (Medium Priority) 🟡

**Option A: Use NLP crate**
- **Recommendation:**
  - `rust-stemmers` for stemming (Porter stemmer)
  - `stop-words` crate for stop word removal
  - `unicode-normalization` for Unicode handling
- Pros: Battle-tested, standard algorithms
- Cons: Multiple dependencies

**Option B: Python FFI with NLTK/spaCy**
- Use PyO3 to call Python NLP libraries
- Pros: Best-in-class NLP
- Cons: Python runtime dependency, slower

**Estimated effort:** 3-4 hours

---

### Phase 3: Implement Identity Criticality (Low Priority) 🟡

**Steps:**
1. Check if `turn.snapshot_id` exists
2. Query `state_snapshots` table for `is_identity_critical` flag
3. Use flag value (0.0 or 1.0) instead of hardcoded 0.5

**Blocker:** Requires async database query in scoring function (or pre-fetch)

**Estimated effort:** 2-3 hours

---

### Phase 4: Add Proper Logging (Low Priority) 🟢

**Steps:**
1. Add `tracing` crate dependency
2. Replace `eprintln!` with `tracing::debug!`
3. Configure log levels

**Estimated effort:** 1 hour

---

## Crate Recommendations

### BM25 / Full-Text Search
1. **`bm25`** (crates.io)
   - Direct BM25 implementation
   - Embedder, Scorer, Search Engine
   - **Recommendation: PRIMARY CHOICE**

2. **`tantivy`**
   - Full Lucene-like search engine
   - Overkill for our use case but production-grade
   - **Use if:** We need faceted search, complex queries

3. **`probly-search`**
   - Fast insertion, full control
   - BM25 built-in
   - **Use if:** Need to optimize for write-heavy workload

### Tokenization / NLP
1. **`rust-stemmers`**
   - Porter stemmer (standard algorithm)
   - **Recommendation: YES, use this**

2. **`stop-words`**
   - Stop word lists for multiple languages
   - **Recommendation: YES, use this**

3. **`unicode-normalization`**
   - Part of Rust standard library ecosystem
   - **Recommendation: YES, use this**

4. **`whatlang`**
   - Language detection
   - **Use if:** Multi-language support needed

### Python FFI (if needed)
1. **`pyo3`**
   - Rust ↔ Python bindings
   - **Use for:** sentiment analysis, NER

2. **Python libraries via PyO3:**
   - `vader_sentiment` (emotional weight)
   - `spacy` (NER for factual density)
   - `nltk` (full NLP toolkit)

---

## Testing Requirements

When remediating tech debt, add these tests:

### BM25 Tests
- [ ] IDF calculation correctness (compare to reference implementation)
- [ ] Corpus statistics update on insert/delete
- [ ] Ranking order matches expected (rare terms > common terms)
- [ ] Performance benchmark (10k documents in <10ms)

### Tokenizer Tests
- [ ] Stemming works ("running" → "run")
- [ ] Stop words removed
- [ ] Punctuation handled correctly
- [ ] Unicode normalized ("café" → "cafe")

### Identity Criticality Tests
- [ ] Loads flag from database correctly
- [ ] Handles missing snapshot_id
- [ ] Caching works (don't re-query same snapshot)

---

## Acceptance Criteria

**BM25 is properly implemented when:**
- [ ] IDF calculated from actual corpus
- [ ] Average document length calculated from corpus
- [ ] Inverted index used for fast lookups
- [ ] Ranking correctness validated against known test cases
- [ ] Performance: P95 latency <15ms for 1k documents

**Tokenizer is properly implemented when:**
- [ ] Stemming applied (configurable algorithm)
- [ ] Stop words removed (configurable list)
- [ ] Punctuation handled
- [ ] Unicode normalized

**TurnSignificance is properly implemented when:**
- [ ] All 6 fields have real implementations (not stubs)
- [ ] Identity criticality queries database
- [ ] Emotional weight scored (even if simple heuristic)
- [ ] Factual density calculated (even if regex-based)

---

## Risk Assessment

| Issue | Impact | Effort | Priority |
|-------|--------|--------|----------|
| IDF hardcoded | High | Medium | 🔴 Critical |
| No inverted index | Medium (scales badly) | High | 🟡 Important |
| Simple tokenizer | Medium | Low | 🟡 Important |
| Identity stub | Low | Low | 🟢 Nice-to-have |
| Debug logging | Low | Low | 🟢 Nice-to-have |
| Emotional weight | Low | Medium | 🟢 Future |
| Factual density | Low | Medium | 🟢 Future |

---

## Session Plan (Next Session)

**Goal:** Eliminate critical tech debt (🔴 items)

**Tasks:**
1. Integrate `bm25` crate or implement proper IDF calculation
2. Build inverted index or use crate's index
3. Add `rust-stemmers` for tokenization
4. Benchmark before/after to validate improvements

**Success Metrics:**
- IDF varies by term (not 1.0 for all)
- Ranking demonstrates term discrimination
- Performance stays <15ms P95

**Estimated Time:** 6-10 hours

---

## References

- **COMP Domain Report:** `/home/emzi/Projects/recursive-light/COMP_MEMORY_RETRIEVAL_ANALYSIS.md`
  - Section: BM25 formulas, hyperparameters, dependencies (Appendix B)
- **Okapi BM25 Paper:** Robertson & Zaragoza (2009)
- **Rust crates:**
  - https://crates.io/crates/bm25
  - https://crates.io/crates/rust-stemmers
  - https://crates.io/crates/stop-words

---

**Status:** Tech debt documented, ready for remediation session.
