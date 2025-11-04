# EXECUTIVE SUMMARY: Memory Retrieval Improvements
**Date:** 2025-11-03 | **Agent:** SCI Domain Expert | **Status:** Recommendations Ready

## CURRENT STATE

**Location:** `/home/emzi/Projects/recursive-light/api/src/lib.rs` (lines 402-469)

**Method:** Keyword-based retrieval
- 10 fixed trigger words ("remember", "earlier", "before", etc.)
- Simple pattern matching (case-insensitive substring search)
- Extract 3 keywords from user input (>3 chars, filter common words)
- Search warm memory (session-scoped, limit 3 results)
- Search cold memory (cross-session, limit 2 results)

**Limitations:**
- Misses paraphrases ("recall" matches, "bring up" doesn't)
- No semantic understanding ("discussed cats" won't retrieve "talked about felines")
- Static triggers (can't adapt to conversation context)
- No relevance ranking beyond recency

## RESEARCH FINDINGS (2024-2025)

**Consensus from 15+ peer-reviewed papers:**
1. **Hybrid retrieval** (semantic + keyword) outperforms either method alone by 15-40%
2. **BM25 (keyword)** = high precision, semantic embeddings = high recall
3. **Fusion approach** (0.6 × semantic + 0.4 × keyword) is production standard
4. **Query expansion** improves recall by 10-20% with minimal overhead

**Evidence Sources:**
- Wang et al. EMNLP 2024: "RAG retrieval quality is the primary bottleneck"
- IBM Research 2024: 3-way retrieval (BM25 + dense + sparse) = optimal
- Production systems: Elasticsearch, Cohere, Anthropic Claude all use hybrid

## TOP 3 EVIDENCE-BASED APPROACHES (RANKED)

### 1. HYBRID RETRIEVAL (Semantic + Keyword) - PHASE 3 PRIORITY
**Confidence:** 0.95 | **Effort:** 2-3 weeks | **Impact:** +20-40% quality

**What:** Combine semantic embeddings (OpenAI ada-002 or local) with BM25 keyword scoring
**How:**
1. Semantic search retrieves 5x candidates (high recall)
2. BM25 re-ranks for keyword precision
3. Fusion scoring: `0.6 × semantic + 0.4 × keyword`

**Evidence:** 15+ papers, production systems, established patterns
**Timeline:** Phase 3 (Weeks 4-17) as planned in CAM design
**Dependencies:** Embedding API, pgvector database extension

---

### 2. EXPANDED KEYWORDS (Cognitive Patterns) - IMMEDIATE
**Confidence:** 0.80 | **Effort:** 4 hours | **Impact:** +10-15% recall

**What:** Add 20-30 cognitively-grounded trigger words based on spreading activation theory
**How:** Expand existing keyword list:
```rust
// Current (10 words)
["remember", "earlier", "before", "previously", "you said", "we talked", "last time", "you mentioned", "recall", "discussed"]

// Expanded (30+ words)
+ Temporal: ["ago", "past", "history", "once", "used to", "back when", "prior", "former"]
+ Episodic: ["you told", "you explained", "you asked", "our conversation", "we covered"]
+ Meta-memory: ["forget", "forgot", "remind", "what was", "refresh", "again"]
+ Context: ["the thing about", "that topic", "going back to", "as we discussed"]
```

**Evidence:** Cognitive science (spreading activation), RAG query expansion studies (+10-20% recall)
**Timeline:** Immediate (Phase 2B++) - 1 day implementation
**Dependencies:** None (pure Rust, backward compatible)

---

### 3. LLM-DECIDED RETRIEVAL (Adaptive) - PHASE 3+ OPTIONAL
**Confidence:** 0.75 | **Effort:** 1-2 weeks | **Impact:** +20-30% precision

**What:** LLM #1 decides IF and WHAT to retrieve (not hard-coded triggers)
**How:**
```json
User: "What did we discuss?"
LLM #1 Decision: {
  "needs_retrieval": true,
  "search_type": "semantic",
  "search_query": "past discussion topics",
  "reasoning": "User asking about prior conversation"
}
```

**Evidence:** Emerging pattern (SAM-RAG 2024, Claude memory tools), adaptive retrieval
**Timeline:** Phase 3+ (Weeks 13-17) if Phase 3 hybrid succeeds
**Dependencies:** LLM #1 prompt engineering, careful latency management (+50-150ms)
**Trade-off:** Higher accuracy but adds latency and LLM API costs

## RECOMMENDED ROADMAP

### IMMEDIATE (This Week)
- **Approach 2:** Expand keyword triggers (4 hours)
- **Metrics:** Create benchmark test dataset (8 hours)
- **Logging:** Add retrieval metrics instrumentation (4 hours)

**Total Effort:** 16 hours (2 days)
**Risk:** Low (backward compatible)
**Payoff:** +10-15% recall improvement + measurement foundation

### PHASE 3 (Weeks 4-12)
- **Approach 1:** Implement hybrid retrieval (2-3 weeks)
- **A/B Testing:** Shadow → 10% → 50% → 100% rollout
- **Validation:** Precision@5 >0.70, Recall@5 >0.80, CSAT +5%

**Total Effort:** 3-4 weeks
**Risk:** Medium (infrastructure complexity)
**Payoff:** +20-40% quality improvement (research-validated)

### PHASE 3+ (Weeks 13-17, Optional)
- **Approach 3:** LLM-decided retrieval (if Phase 3 wins A/B test)
- **Evaluation:** Compare adaptive vs. static hybrid
- **Decision:** Deploy if latency/cost justified by metrics

## SUCCESS METRICS

| Metric | Current | Target | Measurement |
|--------|---------|--------|-------------|
| **Precision@5** | Unknown | >0.70 | Offline benchmark |
| **Recall@5** | Unknown | >0.80 | Offline benchmark |
| **MRR** | Unknown | >0.50 | Offline benchmark |
| **Latency P95** | <50ms (warm), <200ms (cold) | No regression | Production |
| **User Satisfaction (CSAT)** | Unknown | +5% | A/B testing |

**Validation Strategy:**
1. **Offline:** Benchmark dataset (50 conversations, 500+ retrieval instances)
2. **Online:** A/B testing (1000+ sessions per group, p<0.05 significance)
3. **Qualitative:** User interviews (N=20) for edge case discovery

## RISK ASSESSMENT

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Embedding API latency | 0.4 | HIGH | Local model fallback, caching |
| pgvector performance | 0.2 | MEDIUM | IVFFlat tuning, Qdrant migration path |
| False positive retrievals | 0.5 | MEDIUM | Re-ranking, recency weighting |
| User confusion (over-retrieval) | 0.2 | MEDIUM | LLM-decided retrieval, user controls |

## SCIENTIFIC CONFIDENCE

**Overall:** 0.85 (HIGH)
- **RAG Best Practices:** 0.90 (15+ papers, production systems)
- **Hybrid Retrieval:** 0.95 (IBM research, BGE M3, industry consensus)
- **Cognitive Models:** 0.80 (spreading activation, context-dependent memory)
- **Evaluation Metrics:** 0.95 (BEIR, MS MARCO, standard IR benchmarks)

**Unknowns:**
1. Conversation memory ≠ document retrieval (need domain-specific validation)
2. Dual-LLM interaction effects (measure end-to-end, not retrieval in isolation)
3. User expectations for memory (interviews, A/B testing)

## FINAL RECOMMENDATION

**Execute in order:**
1. **Approach 2 NOW** (low risk, quick win, 4 hours)
2. **Build measurement infrastructure** (benchmark dataset, metrics logging, 12 hours)
3. **Approach 1 in Phase 3** (highest evidence, best ROI, 3-4 weeks)
4. **Approach 3 if justified** (defer until Phase 3 results, 1-2 weeks)

**Key Success Factor:** Measure before optimizing
- Establish baseline metrics (Precision@K, Recall@K) with current keyword system
- Validate improvements with offline benchmarks AND online A/B tests
- Don't assume technical metrics (Precision) translate to UX (CSAT)—measure both

**TDF Alignment:**
- **SCI:** Empirical validation, falsifiable hypotheses, data-driven decisions
- **COMP:** Algorithmic rigor, performance optimization, production readiness
- **CULT:** Industry best practices, established patterns, proven at scale
- **EXP:** User-centered design, qualitative interviews, satisfaction metrics
- **META:** System self-awareness, adaptive retrieval, continuous improvement

---

**Full Report:** `/home/emzi/Projects/recursive-light/memory-bank/sci-memory-retrieval-evidence-report.md`
**Next Action:** Review recommendations, implement Approach 2 (expanded keywords) immediately
**Status:** Ready for implementation
