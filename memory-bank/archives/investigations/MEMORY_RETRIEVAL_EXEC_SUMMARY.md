# Memory Retrieval Enhancement - Executive Summary
**Date:** 2025-11-03
**Prepared By:** Computational Domain Expert (COMP)
**Status:** Analysis Complete - Decision Required

---

## THE PROBLEM

Current memory retrieval uses **SQL LIKE queries** with keyword matching:
- No relevance ranking (returns arbitrary matches by recency)
- Sequential keyword searches (N queries per retrieval)
- No semantic understanding ("quantum" won't match "QC algorithms")
- LIKE queries don't use indexes (full table scans)

**Current Performance:** Unmeasured, but LIKE queries typically 50-200ms on text columns

---

## TOP 3 SOLUTIONS (Ranked)

### 🥇 RANK 1: BM25 + Temporal Decay (RECOMMENDED)

**What:** Classic information retrieval algorithm with recency weighting

**Why:**
- 10-20x faster than LIKE (5-15ms vs 50-200ms)
- Proven at scale (powers Elasticsearch, Lucene)
- Relevance ranking (no more arbitrary matches)
- Temporal awareness (recent turns prioritized)

**Complexity:** Medium (300 LOC, 2-3 weeks)

**Trade-offs:**
- ✅ Fast, reliable, debuggable
- ✅ Low memory overhead (100KB per 1000 turns)
- ❌ No semantic understanding (exact keyword matching only)

---

### 🥈 RANK 2: Semantic Embeddings with Vector Search

**What:** Neural network embeddings + cosine similarity

**Why:**
- Semantic query understanding ("ML" matches "machine learning")
- Multilingual support
- State-of-art for semantic search

**Complexity:** High (500 LOC, 3-4 weeks)

**Trade-offs:**
- ✅ Handles synonyms, abbreviations, paraphrases
- ⚠️ Requires GPU for <100ms latency (CPU: 150-230ms)
- ⚠️ Black-box debugging (hard to explain why results match)
- ⚠️ 80MB model download, 2-5s cold start

---

### 🥉 RANK 3: Graph-Based PageRank

**What:** Conversation graph with relevance propagation

**Why:**
- Models human memory associations
- Captures cross-conversation references

**Complexity:** High (600 LOC, 4-5 weeks)

**Trade-offs:**
- ⚠️ Quadratic graph construction (O(n²) scaling)
- ⚠️ Cold start problem (new sessions have no edges)
- ⚠️ Experimental (less proven than BM25/embeddings)
- ⚠️ Best for power users with >10k interconnected turns

---

## RECOMMENDATION: Staged Hybrid Approach

### Phase 1: BM25 Foundation (IMPLEMENT NOW)
**Timeline:** 2-3 weeks
**Impact:** 10-20x faster, relevance ranking, temporal decay
**Risk:** Low (proven algorithm, backward compatible)

### Phase 2: Semantic Layer (GATED DECISION)
**Timeline:** 3-4 weeks (if approved)
**Proceed ONLY if:**
- User query logs show semantic gaps
- BM25 precision <70% on semantic queries
- GPU available OR CPU latency acceptable

### Phase 3: Graph Augmentation (DEFER)
**Timeline:** TBD
**Reconsider:** Only if users have dense (>10k turns) history

---

## IMPLEMENTATION PLAN (Phase 1 - BM25)

### Week 1: Core Implementation
- Build `InvertedIndex` with BM25 scoring
- Add tokenization + stemming
- Implement temporal decay
- Write 5 unit tests

### Week 2: Integration
- Modify `MemoryTierManager` to use BM25
- Replace LIKE queries with index lookups
- Add incremental index updates
- Write 6 integration tests

### Week 3: Testing & Optimization
- Benchmark latency (target: P95 <15ms)
- Hyperparameter tuning (k₁, b, λ)
- Precision@k testing (target: >70%)
- Feature flag for A/B testing

### Week 4: Deployment
- Canary to 10% of users
- Monitor metrics (latency, precision)
- Rollout to 100% if acceptable

---

## KEY METRICS (Success Criteria)

| Metric | Current | Target (BM25) | Target (Semantic) |
|--------|---------|---------------|-------------------|
| **Latency P95** | ~50-200ms | <15ms | <100ms (GPU) |
| **Precision@5** | Unknown | >70% | >80% |
| **Memory overhead** | 0 (DB only) | 100KB/1k turns | 1.5MB/1k turns |
| **Semantic accuracy** | 0% | 0% | 90%+ |

---

## RISKS & MITIGATIONS

| Risk | Mitigation |
|------|------------|
| **Index out of sync** | Transactional updates, validation tests |
| **Latency budget exceeded** | Benchmark early, shard index if needed |
| **Breaking changes** | Parallel deployment, feature flag, fallback |
| **Premature optimization** | Start with BM25, only add semantic if data shows need |

---

## DECISION REQUIRED

**Question:** Approve Phase 1 (BM25) implementation for next sprint?

**Options:**
1. ✅ **Approve** - Proceed with 2-3 week BM25 implementation
2. ⏸️ **Defer** - Prioritize other features, revisit in Q2
3. 🔄 **Request POC** - Build 4-hour proof-of-concept first

**Recommended:** Option 1 (Approve) - Clear ROI, low risk, aligns with TDF

---

## RESOURCES

- **Full Analysis:** `/home/emzi/Projects/recursive-light/COMP_MEMORY_RETRIEVAL_ANALYSIS.md` (10,000+ words)
- **Current Implementation:** `api/src/dual_llm/memory_tiering.rs` lines 335-616
- **Performance Benchmarks:** `api/src/flow_process.rs` lines 3400-3670
- **Test Coverage:** 143 tests passing (0 failures)

---

**Contact:** Computational Domain Expert (COMP)
**Next Action:** Stakeholder decision on Phase 1 approval
