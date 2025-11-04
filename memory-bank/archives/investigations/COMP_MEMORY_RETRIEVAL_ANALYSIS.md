# Computational Analysis: Intelligent Memory Retrieval for Recursive Light
**Domain:** COMP (Computational/Algorithmic)
**Date:** 2025-11-03
**Analyst:** Computational Domain Expert
**Focus:** Performance, scalability, formal correctness

---

## EXECUTIVE SUMMARY

Current implementation uses keyword matching with SQL LIKE queries. Analysis reveals **THREE viable computational approaches** ranked by TDF alignment, with clear implementation paths and performance trade-offs.

**Top Recommendation:** Hybrid BM25 + Temporal Decay (medium complexity, high ROI)

---

## CURRENT SYSTEM ANALYSIS

### Implementation Review
**Location:** `/home/emzi/Projects/recursive-light/api/src/dual_llm/memory_tiering.rs` (lines 335-377, 573-616)

**Current Algorithm:**
1. Trigger detection: 10 hardcoded keywords ("remember", "earlier", etc.)
2. Keyword extraction: Words >3 chars, excluding trigger words, limit 3
3. Search execution: SQL `LOWER(column) LIKE LOWER('%keyword%')` per keyword
4. Result merging: First non-empty result only (stops after first match)

**SQL Query Pattern:**
```sql
SELECT ... FROM conversation_turns
WHERE session_id = ?1
  AND ai_response IS NOT NULL
  AND (LOWER(user_message) LIKE LOWER(?2) OR LOWER(ai_response) LIKE LOWER(?2))
ORDER BY turn_number DESC
LIMIT ?3
```

### Performance Characteristics

**Current Latency Budget:**
- Memory operations target: <50ms (P95)
- Full pipeline target: <500ms (P95)
- Source: `flow_process.rs` lines 3494-3499

**Measured Performance (143 tests passing):**
- Database operations: Fast (<30ms load, <50ms save)
- No LIKE query benchmarks found in test suite

**Scaling Analysis:**
- Warm memory: Session-scoped (~50 turns, bounded)
- Cold memory: Cross-session (unbounded growth)
- Search pattern: Sequential keyword iteration (stops at first match)
- Index usage: `idx_turns_session`, `idx_turns_user`, `idx_turns_tier` present

**Critical Issues:**
1. **LIKE queries don't use indexes** - Full table scan on text columns
2. **Sequential keyword search** - N queries per retrieval
3. **No relevance ranking** - Returns arbitrary matches (ORDER BY recency only)
4. **Stops at first match** - May miss more relevant results
5. **No semantic understanding** - "quantum computing" won't match "QC" or "qubit algorithms"

---

## COMPUTATIONAL APPROACHES (Ranked by TDF Alignment)

### RANK 1: Hybrid BM25 + Temporal Decay Model ⭐ RECOMMENDED

**Algorithm:** Statistical term frequency with time-weighted scoring

**Core Formula:**
```
score(d, q) = BM25(d, q) × temporal_weight(age(d))

BM25(d, q) = Σ IDF(qᵢ) × (f(qᵢ, d) × (k₁ + 1)) / (f(qᵢ, d) + k₁ × (1 - b + b × |d| / avgdl))

temporal_weight(t) = exp(-λ × t)  where λ = decay_rate
```

**Implementation Strategy:**

1. **Inverted Index Structure:**
```rust
// In-memory index (rebuild on startup, update incrementally)
struct InvertedIndex {
    term_to_turns: HashMap<String, Vec<(Uuid, f64)>>,  // term -> [(turn_id, tf_idf)]
    turn_metadata: HashMap<Uuid, TurnMetadata>,
    doc_freq: HashMap<String, usize>,
    total_turns: usize,
    avg_turn_length: f64,
}

struct TurnMetadata {
    turn_id: Uuid,
    created_at: DateTime<Utc>,
    token_count: usize,
    tier: MemoryTier,
}
```

2. **Indexing Pipeline:**
```rust
impl MemoryTierManager {
    async fn index_turn(&mut self, turn: &ConversationTurn) {
        // Tokenize: split + lowercase + stem
        let tokens = tokenize(&format!("{} {}", turn.user_message, turn.ai_response));

        // Update term frequencies
        for token in tokens {
            self.index.add_term(token, turn.id, tf);
        }

        // Update metadata
        self.index.add_metadata(turn.id, TurnMetadata { ... });
    }
}
```

3. **Search Algorithm:**
```rust
async fn search_with_bm25(
    &self,
    query: &str,
    limit: usize,
    decay_lambda: f64,
) -> Vec<(ConversationTurn, f64)> {
    // Tokenize query
    let query_terms = tokenize(query);

    // Calculate BM25 scores
    let mut scores: HashMap<Uuid, f64> = HashMap::new();
    for term in query_terms {
        if let Some(postings) = self.index.term_to_turns.get(&term) {
            let idf = self.index.idf(&term);
            for (turn_id, tf) in postings {
                let bm25_component = self.index.bm25_term_score(tf, idf, turn_id);
                *scores.entry(*turn_id).or_default() += bm25_component;
            }
        }
    }

    // Apply temporal decay
    let now = Utc::now();
    for (turn_id, score) in scores.iter_mut() {
        let metadata = self.index.turn_metadata.get(turn_id).unwrap();
        let age_hours = (now - metadata.created_at).num_hours() as f64;
        *score *= f64::exp(-decay_lambda * age_hours);
    }

    // Sort by score and fetch top-k
    let mut ranked: Vec<_> = scores.into_iter().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Fetch full turns from database
    let turn_ids: Vec<Uuid> = ranked.iter().take(limit).map(|(id, _)| *id).collect();
    self.fetch_turns_by_ids(&turn_ids).await
}
```

**Dependencies:**
```toml
rust-stemmers = "1.2"  # Porter stemmer for tokenization
unicode-segmentation = "1.10"  # Proper text segmentation
```

**Performance Characteristics:**
- **Indexing:** O(|D| × |T|) where D=document, T=terms (one-time + incremental)
- **Search:** O(|Q| × |P|) where Q=query terms, P=avg postings length
- **Memory:** ~100KB per 1000 turns (sparse index)
- **Latency:** 5-15ms for typical queries (in-memory lookup)

**Persistence Strategy:**
```sql
-- Materialized index table (optional, for cold starts)
CREATE TABLE IF NOT EXISTS memory_search_index (
    term TEXT NOT NULL,
    turn_id BLOB NOT NULL,
    tf_idf REAL NOT NULL,
    PRIMARY KEY (term, turn_id)
);
CREATE INDEX idx_search_term ON memory_search_index(term);
```

**TDF Alignment:**
- **COMP:** ✅✅✅ Classical IR algorithm, proven convergence, measurable precision/recall
- **SCI:** ✅ Evidence-based (BM25 beats TF-IDF in TREC benchmarks)
- **EXP:** ✅✅ Temporal decay models human memory forgetting curves
- **CULT:** ⚠️ Language-agnostic (current tokenization English-centric)
- **META:** ✅ Self-tuning via k₁, b, λ hyperparameters

**Implementation Complexity:** MEDIUM
- Core algorithm: 300-400 LOC
- Testing: 15 tests (precision@k, recall@k, temporal weighting)
- Integration: Parallel to existing LIKE search (A/B testable)

**Migration Path:**
1. Phase 1: Build index in-memory, parallel to LIKE queries
2. Phase 2: A/B test retrieval quality (log precision/recall)
3. Phase 3: Switch to BM25 as primary, LIKE as fallback
4. Phase 4: Persist index to database for cold starts

---

### RANK 2: Semantic Embeddings with Vector Search

**Algorithm:** Neural embeddings + cosine similarity in vector space

**Core Concept:**
```
embed(text) → ℝⁿ  (typically n=384 or 768)
similarity(q, d) = cos(embed(q), embed(d))
```

**Implementation Strategy:**

1. **Embedding Generation:**
```rust
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;

struct EmbeddingIndex {
    model: SentenceEmbeddingsModel,
    vectors: Vec<(Uuid, Vec<f32>)>,  // (turn_id, embedding)
}

impl EmbeddingIndex {
    async fn embed_turn(&self, turn: &ConversationTurn) -> Vec<f32> {
        let text = format!("{} {}", turn.user_message, turn.ai_response);
        self.model.encode(&[text])[0].clone()
    }
}
```

2. **Vector Search (Exact):**
```rust
fn search_knn(&self, query: &str, k: usize) -> Vec<(Uuid, f64)> {
    let query_vec = self.embed_text(query);

    let mut scores: Vec<_> = self.vectors
        .iter()
        .map(|(id, vec)| {
            let similarity = cosine_similarity(&query_vec, vec);
            (*id, similarity)
        })
        .collect();

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scores.truncate(k);
    scores
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    (dot / (norm_a * norm_b)) as f64
}
```

3. **Approximate Vector Search (HNSW for scale):**
```rust
use instant_distance::{Builder, Search};

struct ApproximateIndex {
    hnsw: Hnsw<Uuid, Vec<f32>>,
}

impl ApproximateIndex {
    fn build(vectors: Vec<(Uuid, Vec<f32>)>) -> Self {
        let mut builder = Builder::default();
        for (id, vec) in vectors {
            builder.add(id, vec);
        }
        Self { hnsw: builder.build() }
    }

    fn search(&self, query: &[f32], k: usize) -> Vec<Uuid> {
        let mut search = Search::default();
        self.hnsw.search(query, &mut search);
        search.iter().take(k).map(|item| item.pid).collect()
    }
}
```

**Database Integration (pgvector):**
```sql
-- Requires PostgreSQL + pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

ALTER TABLE conversation_turns
ADD COLUMN embedding vector(384);

CREATE INDEX ON conversation_turns USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);

-- Query
SELECT *, embedding <=> '[...]' AS distance
FROM conversation_turns
WHERE user_id = ?
ORDER BY distance
LIMIT 10;
```

**Dependencies:**
```toml
rust-bert = "0.21"  # 1.5GB model download, CPU/GPU inference
instant-distance = "0.6"  # HNSW for approximate search
pgvector = "0.4"  # PostgreSQL vector extension (if using PG)

# OR lightweight alternative:
ort = "2.0"  # ONNX runtime (smaller models, faster)
```

**Performance Characteristics:**
- **Embedding generation:** 50-200ms per turn (CPU), 5-20ms (GPU)
- **Exact search (1000 turns):** 10-30ms (cosine similarity)
- **HNSW search (100k turns):** 1-5ms (approximate, 95%+ recall)
- **Memory:** ~1.5KB per turn (384-dim floats)
- **Latency breakdown:**
  - Query embedding: 50-200ms (one-time per query)
  - Vector search: 1-30ms (depends on index type)
  - **Total: 60-230ms** (exceeds current budget if CPU-only)

**TDF Alignment:**
- **COMP:** ✅✅ Well-defined vector operations, HNSW provably efficient
- **SCI:** ✅✅✅ State-of-art for semantic search (BERT/Sentence-BERT benchmarks)
- **EXP:** ✅✅ Captures semantic similarity ("quantum computing" ≈ "QC algorithms")
- **CULT:** ✅✅ Multilingual models available (mBERT, XLM-R)
- **META:** ⚠️ Black-box embeddings (hard to debug why results match)

**Implementation Complexity:** HIGH
- Core algorithm: 200-300 LOC
- Model integration: 100-200 LOC (download, cache, inference)
- Testing: 20 tests (semantic similarity, multilingual, edge cases)
- Infrastructure: GPU recommended for <100ms latency

**Critical Trade-offs:**
1. **Latency:** Embedding generation dominates (50-200ms CPU)
   - Mitigation: Precompute embeddings, cache aggressively
2. **Model size:** 300MB-1.5GB on disk
   - Mitigation: Use distilled models (MiniLM, TinyBERT)
3. **Cold start:** Model load time 2-5 seconds
   - Mitigation: Lazy load on first query

**Migration Path:**
1. Phase 1: Add embedding column to DB, backfill async
2. Phase 2: Hybrid search (BM25 for exact terms + embeddings for semantic)
3. Phase 3: A/B test pure semantic vs hybrid
4. Phase 4: Add GPU inference if latency requires

---

### RANK 3: Graph-Based Conversational Flow Retrieval

**Algorithm:** Conversation graph with PageRank-style relevance propagation

**Core Concept:**
```
Nodes: ConversationTurns
Edges: (temporal_succession, topic_similarity, user_reference)
Ranking: Modified PageRank with query-specific teleportation
```

**Graph Structure:**
```rust
struct ConversationGraph {
    nodes: HashMap<Uuid, GraphNode>,
    edges: Vec<Edge>,
}

struct GraphNode {
    turn_id: Uuid,
    keywords: HashSet<String>,
    timestamp: DateTime<Utc>,
    pagerank: f64,
}

struct Edge {
    from: Uuid,
    to: Uuid,
    weight: f64,
    edge_type: EdgeType,
}

enum EdgeType {
    Temporal,           // Next turn in session
    TopicSimilarity,    // Shared keywords
    ExplicitReference,  // User says "like we discussed earlier"
}
```

**Graph Construction:**
```rust
impl ConversationGraph {
    fn build(turns: &[ConversationTurn]) -> Self {
        let mut graph = Self::new();

        // Add temporal edges (within session)
        for window in turns.windows(2) {
            if window[0].session_id == window[1].session_id {
                graph.add_edge(
                    window[0].id,
                    window[1].id,
                    1.0,
                    EdgeType::Temporal
                );
            }
        }

        // Add topic similarity edges (cross-session)
        for (i, turn_a) in turns.iter().enumerate() {
            let keywords_a = extract_keywords(&turn_a.user_message);
            for turn_b in &turns[i+1..] {
                let keywords_b = extract_keywords(&turn_b.user_message);
                let overlap = keywords_a.intersection(&keywords_b).count();
                if overlap >= 2 {
                    let weight = overlap as f64 / keywords_a.len().max(keywords_b.len()) as f64;
                    graph.add_edge(turn_a.id, turn_b.id, weight, EdgeType::TopicSimilarity);
                }
            }
        }

        graph
    }
}
```

**Query-Biased PageRank:**
```rust
fn search_with_graph_rank(&self, query: &str, damping: f64, iterations: usize) -> Vec<Uuid> {
    let query_keywords = extract_keywords(query);

    // Initialize: High rank for query-matching nodes
    let mut ranks: HashMap<Uuid, f64> = HashMap::new();
    for (id, node) in &self.nodes {
        let overlap = node.keywords.intersection(&query_keywords).count();
        ranks.insert(*id, if overlap > 0 { 1.0 } else { 0.0 });
    }

    // Power iteration
    for _ in 0..iterations {
        let mut new_ranks = HashMap::new();
        for (id, node) in &self.nodes {
            let incoming_score: f64 = self.edges
                .iter()
                .filter(|e| e.to == *id)
                .map(|e| {
                    let from_rank = ranks.get(&e.from).unwrap_or(&0.0);
                    let out_degree = self.out_degree(&e.from);
                    from_rank * e.weight / out_degree as f64
                })
                .sum();

            let teleport_score = if node.keywords.intersection(&query_keywords).count() > 0 {
                1.0
            } else {
                0.0
            };

            let rank = damping * incoming_score + (1.0 - damping) * teleport_score;
            new_ranks.insert(*id, rank);
        }
        ranks = new_ranks;
    }

    // Sort by rank
    let mut ranked: Vec<_> = ranks.into_iter().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    ranked.iter().map(|(id, _)| *id).collect()
}
```

**Performance Characteristics:**
- **Graph construction:** O(|T|²) for topic similarity (quadratic!)
  - Mitigation: Sliding window, approximate nearest neighbors
- **PageRank:** O(|E| × k) where k=iterations (typically 10-20)
- **Search:** 20-100ms for 1000 turns, 10 iterations
- **Memory:** ~50 bytes per edge (sparse, ~10 edges per node)
- **Scalability:** Requires incremental graph updates

**TDF Alignment:**
- **COMP:** ✅✅ Graph algorithms well-studied, convergence guarantees
- **SCI:** ⚠️ Less empirical evidence for conversational retrieval
- **EXP:** ✅✅✅ Models human memory associations (spreading activation theory)
- **CULT:** ⚠️ Language-dependent keyword extraction
- **META:** ✅✅ Captures meta-conversational patterns (callbacks, references)

**Implementation Complexity:** HIGH
- Core algorithm: 400-600 LOC (graph + PageRank)
- Incremental updates: 200 LOC (maintain graph on new turns)
- Testing: 25 tests (graph construction, ranking, edge cases)
- Database: Requires graph persistence (edge table)

**Critical Issues:**
1. **Quadratic graph construction** - Doesn't scale to 100k+ turns
2. **Cold start problem** - New sessions have no edges
3. **Sparse graphs** - Conversations may not interconnect meaningfully

**Use Case:** Best for users with rich, interconnected conversation history (power users, multi-month sessions).

**Migration Path:**
1. Phase 1: Build graph offline, evaluate on historical data
2. Phase 2: Hybrid with BM25 (graph for "deep" queries, BM25 for quick lookups)
3. Phase 3: Incremental graph updates on new turns
4. Phase 4: Visualize conversation graph for debugging

---

## COMPARATIVE ANALYSIS

| Criterion | BM25 + Temporal | Semantic Embeddings | Graph PageRank |
|-----------|-----------------|---------------------|----------------|
| **Latency (1k turns)** | 5-15ms ✅ | 60-230ms ⚠️ | 20-100ms ⚠️ |
| **Memory overhead** | 100KB ✅ | 1.5MB ⚠️ | 500KB ✅ |
| **Semantic understanding** | ❌ | ✅✅✅ | ⚠️ (keyword-based) |
| **Multilingual** | ⚠️ (stemmer-dependent) | ✅✅ | ⚠️ |
| **Temporal awareness** | ✅✅✅ (explicit decay) | ❌ | ⚠️ (via edges) |
| **Cold start** | ✅ (immediate) | ⚠️ (2-5s model load) | ❌ (needs edges) |
| **Scalability to 100k turns** | ✅ (O(log n) with index) | ✅ (HNSW: O(log n)) | ❌ (O(n²) construction) |
| **Implementation complexity** | Medium (400 LOC) | High (500+ LOC) | High (600+ LOC) |
| **Testing burden** | 15 tests | 20 tests | 25 tests |
| **TDF alignment** | ✅✅✅ | ✅✅✅ | ✅✅ |
| **Debuggability** | ✅✅ (inspect scores) | ⚠️ (black box) | ✅✅ (graph viz) |
| **Production readiness** | ✅ (proven at scale) | ✅ (needs GPU) | ⚠️ (experimental) |

---

## RECOMMENDED APPROACH: Staged Hybrid System

### Phase 1: BM25 Foundation (Weeks 1-2)
**Goal:** Replace LIKE queries with BM25 + temporal decay

**Deliverables:**
1. `search_index.rs` module (300 LOC)
   - InvertedIndex struct
   - Tokenization + stemming
   - BM25 scoring
   - Temporal decay weighting
2. Integration tests (15 tests)
   - Precision@k, recall@k
   - Temporal weighting validation
   - Comparison vs LIKE queries
3. Migration: Parallel deployment, A/B test, cutover

**Expected Impact:**
- 10-20x faster than LIKE queries (5-15ms vs 50-200ms)
- Relevance ranking (no more arbitrary matches)
- Temporal decay (recent turns favored)

### Phase 2: Semantic Layer (Weeks 3-4, Optional)
**Goal:** Add semantic search for complex queries

**Deliverables:**
1. `embeddings.rs` module (200 LOC)
   - ONNX model integration (MiniLM-L6: 80MB)
   - Embedding cache (SQLite BLOB column)
   - Async embedding generation
2. Hybrid search strategy:
   ```rust
   fn hybrid_search(query: &str) -> Vec<ConversationTurn> {
       let bm25_results = search_bm25(query, 10);
       let semantic_results = search_semantic(query, 10);

       // Reciprocal rank fusion
       merge_results(bm25_results, semantic_results)
   }
   ```
3. Performance testing (GPU vs CPU inference)

**Expected Impact:**
- Semantic query understanding ("QC" matches "quantum computing")
- Multilingual support (if using mBERT)
- Latency: 60-100ms with GPU, 150-230ms with CPU

**Gating Decision:** Only proceed if:
- User queries show semantic gaps (log analysis)
- GPU available for <100ms latency
- A/B test shows >20% improvement in user satisfaction

### Phase 3: Graph Augmentation (Weeks 5-6, Experimental)
**Goal:** Add conversational flow awareness

**When to implement:** Only if:
- Users have >1000 turns per user (dense history)
- Explicit references frequent ("like last week when...")
- BM25 precision <70% on historical queries

**Not recommended for initial launch** due to complexity vs benefit.

---

## INTEGRATION ARCHITECTURE

### Proposed API Changes

```rust
// api/src/dual_llm/memory_search.rs (NEW MODULE)

pub struct MemorySearchEngine {
    bm25_index: BM25Index,
    embedding_index: Option<EmbeddingIndex>,  // Optional semantic layer
}

impl MemorySearchEngine {
    pub async fn search(
        &self,
        query: &str,
        scope: SearchScope,
        limit: usize,
    ) -> Result<Vec<ScoredTurn>, SearchError> {
        match self.embedding_index {
            Some(ref embeddings) => self.hybrid_search(query, scope, limit).await,
            None => self.bm25_search(query, scope, limit).await,
        }
    }

    fn bm25_search(&self, query: &str, scope: SearchScope, limit: usize)
        -> Result<Vec<ScoredTurn>, SearchError>
    {
        let tokens = tokenize(query);
        let mut scores = self.bm25_index.score_documents(&tokens, scope);

        // Apply temporal decay
        let now = Utc::now();
        for (turn_id, score) in &mut scores {
            let metadata = self.bm25_index.get_metadata(turn_id)?;
            let age_hours = (now - metadata.created_at).num_hours() as f64;
            *score *= temporal_decay(age_hours, DECAY_LAMBDA);
        }

        // Sort and limit
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.truncate(limit);

        Ok(scores.into_iter().map(|(id, score)| ScoredTurn { id, score }).collect())
    }
}

pub struct ScoredTurn {
    pub turn_id: Uuid,
    pub score: f64,
}

pub enum SearchScope {
    Session(Uuid),        // Warm memory
    User(Uuid),           // Cold memory
    Global,               // All memory
}
```

### Integration into MemoryTierManager

```rust
// api/src/dual_llm/memory_tiering.rs (MODIFICATIONS)

impl MemoryTierManager {
    pub async fn search_warm_memory(
        &self,
        session_id: Uuid,
        query: &str,  // Changed from keyword: &str
        limit: usize,
    ) -> Result<Vec<ConversationTurn>, sqlx::Error> {
        // NEW: Use search engine instead of LIKE query
        let scored_turns = self.search_engine
            .search(query, SearchScope::Session(session_id), limit)
            .await?;

        // Fetch full turns by IDs
        let turn_ids: Vec<Uuid> = scored_turns.iter().map(|st| st.turn_id).collect();
        self.fetch_turns_by_ids(&turn_ids).await
    }

    async fn fetch_turns_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ConversationTurn>, sqlx::Error> {
        // Single batched query instead of N queries
        sqlx::query("SELECT * FROM conversation_turns WHERE id IN (?)")
            .bind(ids)
            .fetch_all(&self.db_pool)
            .await
            .map(|rows| rows.into_iter().map(ConversationTurn::from_row).collect())
    }
}
```

### VifApi Integration (Minimal Changes)

```rust
// api/src/lib.rs (lines 435-469 - MINIMAL CHANGES)

// Phase 2B: Keyword-triggered warm/cold memory retrieval
if should_search_memory {
    // OLD: Extract keywords manually
    // let search_keywords: Vec<&str> = user_input.split_whitespace()...

    // NEW: Pass full query to search engine (handles tokenization internally)
    let query = user_input;

    // Search warm memory (unchanged API, smarter backend)
    if let Ok(warm_turns) = self.memory_tier_manager
        .search_warm_memory(session_id, query, 3)  // Now query-based, not keyword-based
        .await
    {
        // ... (rest unchanged)
    }

    // Search cold memory (unchanged API, smarter backend)
    if let Ok(cold_turns) = self.memory_tier_manager
        .search_cold_memory(user_id, query, 2)
        .await
    {
        // ... (rest unchanged)
    }
}
```

**Backward Compatibility:** ✅
- API signature unchanged (`search_warm_memory` still takes `&str`)
- Graceful fallback: If index unavailable, use LIKE queries
- A/B testable: Run both paths, log metrics, compare

---

## PERFORMANCE MODELING

### Latency Budget Analysis

**Current Budget (from tests):**
- Memory operations: <50ms (P95)
- Full pipeline: <500ms (P95)

**BM25 Search Breakdown:**
```
Tokenization:        0.1ms  (split + lowercase + stem)
Index lookup:        2-5ms  (HashMap + Vec scan for 1k turns)
Temporal weighting:  0.5ms  (exp() computation per result)
Turn fetching:       3-8ms  (batched SQL by ID, indexed)
------------------------
Total:               5-15ms ✅ (within budget, 3-10x headroom)
```

**Semantic Search Breakdown:**
```
Query embedding:     50-200ms CPU, 5-20ms GPU
Vector search:       1-30ms (HNSW vs exact)
Turn fetching:       3-8ms
------------------------
Total (CPU):         60-230ms ⚠️ (borderline, needs caching)
Total (GPU):         10-60ms ✅ (within budget with GPU)
```

**Scaling Projections:**

| Turns per User | BM25 Latency | Semantic (CPU) | Semantic (GPU) |
|----------------|--------------|----------------|----------------|
| 100            | 3ms          | 55ms           | 8ms            |
| 1,000          | 8ms          | 75ms           | 15ms           |
| 10,000         | 25ms         | 150ms          | 40ms           |
| 100,000        | 80ms ⚠️      | 400ms ❌       | 120ms ⚠️       |

**Mitigation for 100k+ scale:**
- BM25: Shard index by time period (hot index: last 30 days)
- Semantic: Use HNSW approximate search (5-10ms regardless of size)
- Both: Prefilter by recency before searching (limit corpus to last 10k turns)

---

## TESTING STRATEGY

### Test Suite Structure (BM25 Focus)

```rust
// api/src/dual_llm/search_index.rs tests

#[cfg(test)]
mod tests {
    // UNIT TESTS (5 tests)

    #[test]
    fn test_tokenization_basic() {
        let tokens = tokenize("Hello World! Testing 123.");
        assert_eq!(tokens, vec!["hello", "world", "test"]);  // stemmed
    }

    #[test]
    fn test_bm25_scoring() {
        let index = build_test_index();
        let scores = index.score_documents(&["quantum"], SearchScope::Global);
        assert!(scores[0].1 > scores[1].1);  // relevance ordering
    }

    #[test]
    fn test_temporal_decay() {
        let recent_score = temporal_decay(1.0, 0.01);   // 1 hour ago
        let old_score = temporal_decay(168.0, 0.01);    // 1 week ago
        assert!(recent_score > old_score * 2.0);         // significant decay
    }

    // INTEGRATION TESTS (6 tests)

    #[tokio::test]
    async fn test_search_warm_memory_with_bm25() {
        let manager = setup_test_manager().await;
        // Insert 50 turns with diverse content
        // Query "quantum computing"
        // Assert: Returns turns with "quantum" ranked higher than "computing"
    }

    #[tokio::test]
    async fn test_search_cold_memory_cross_session() {
        // Insert 3 sessions, query across all
        // Assert: Temporal decay favors recent sessions
    }

    #[tokio::test]
    async fn test_search_fallback_to_like_query() {
        // Simulate index unavailable
        // Assert: Falls back to original LIKE query
    }

    // BENCHMARK TESTS (2 tests)

    #[tokio::test]
    async fn bench_bm25_search_1000_turns() {
        let manager = setup_large_test_db(1000).await;
        let start = Instant::now();
        for _ in 0..100 {
            manager.search_warm_memory(session_id, "quantum", 5).await.unwrap();
        }
        let avg_latency = start.elapsed().as_millis() / 100;
        assert!(avg_latency < 15, "BM25 search too slow: {}ms", avg_latency);
    }

    // PRECISION/RECALL TESTS (2 tests)

    #[tokio::test]
    async fn test_precision_at_k() {
        // Golden dataset: manually labeled relevant turns
        // Query, retrieve top 5
        // Assert: Precision@5 > 0.7 (70% relevant results)
    }
}
```

### Evaluation Metrics

**Quantitative Metrics:**
1. **Precision@k:** % of top-k results that are relevant
   - Target: >70% for k=5
2. **Recall@k:** % of relevant results in top-k
   - Target: >50% for k=5 (out of all relevant in corpus)
3. **Mean Reciprocal Rank (MRR):** 1/rank of first relevant result
   - Target: >0.6 (first relevant result in top-2 on average)
4. **Latency P50/P95/P99:** Search time percentiles
   - Target: P95 < 15ms (BM25), P95 < 100ms (semantic)

**Qualitative Evaluation:**
1. **A/B Testing:** Deploy BM25 to 50% of users, log:
   - User engagement (do they use retrieved context?)
   - Follow-up queries (do they re-search or accept results?)
   - Manual review of 100 random queries (human relevance judgment)

2. **Failure Case Analysis:** Collect examples where BM25 fails:
   - Semantic gap (e.g., "ML" not matching "machine learning")
   - Multi-word concepts (e.g., "quantum" vs "quantum computing")
   - Pronoun references (e.g., "that thing we discussed")

---

## RISK ANALYSIS

### BM25 + Temporal Decay Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Index out of sync** | Medium | High | Transactional index updates, validation tests |
| **Stemmer language bias** | High | Medium | Detect language, use appropriate stemmer |
| **Memory growth** | Low | Medium | Periodic index compaction, LRU eviction |
| **Temporal decay too aggressive** | Medium | Medium | A/B test decay rates (λ=0.001, 0.01, 0.1) |
| **BM25 hyperparameters suboptimal** | Medium | Low | Grid search on historical queries (k₁, b) |

### Semantic Embeddings Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Latency budget exceeded (CPU)** | High | High | Require GPU, or cache embeddings aggressively |
| **Model size too large** | Medium | Medium | Use distilled models (MiniLM: 80MB vs BERT: 400MB) |
| **Cold start delay** | Medium | Low | Lazy load model on first query |
| **Embedding drift over time** | Low | Medium | Periodic re-embedding of old turns |
| **Black-box debugging** | High | Medium | Log similarity scores, visualize nearest neighbors |

### General Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Premature optimization** | Medium | High | Start with BM25, only add semantic if data shows need |
| **Test coverage insufficient** | Medium | High | Mandate 90% coverage, precision/recall benchmarks |
| **Breaking backward compat** | Low | High | Parallel deployment, feature flag, graceful fallback |

---

## IMPLEMENTATION CHECKLIST

### BM25 + Temporal Decay (Recommended First Step)

- [ ] **Week 1: Core Implementation**
  - [ ] Create `api/src/dual_llm/search_index.rs` module
  - [ ] Implement `InvertedIndex` struct with HashMap backend
  - [ ] Add tokenization (split, lowercase, stem via `rust-stemmers`)
  - [ ] Implement BM25 scoring (k₁=1.5, b=0.75 defaults)
  - [ ] Add temporal decay function (λ=0.01 default)
  - [ ] Write 5 unit tests (tokenization, scoring, decay)

- [ ] **Week 2: Integration**
  - [ ] Modify `MemoryTierManager::search_warm_memory` to use BM25
  - [ ] Modify `MemoryTierManager::search_cold_memory` to use BM25
  - [ ] Add `fetch_turns_by_ids` batched query method
  - [ ] Implement index build on startup (from existing DB)
  - [ ] Implement incremental index updates on new turns
  - [ ] Write 6 integration tests

- [ ] **Week 3: Testing & Optimization**
  - [ ] Add benchmark tests (1k, 10k turns)
  - [ ] Add precision@k, recall@k tests with golden dataset
  - [ ] Hyperparameter tuning (k₁, b, λ grid search)
  - [ ] Memory profiling (ensure <100KB per 1k turns)
  - [ ] Feature flag for A/B testing

- [ ] **Week 4: Deployment**
  - [ ] Deploy to 10% of users (canary)
  - [ ] Monitor latency metrics (P50/P95/P99)
  - [ ] Log retrieval quality (manual review)
  - [ ] Rollout to 100% if metrics acceptable

### Semantic Embeddings (Optional Phase 2)

- [ ] **Week 5: Feasibility Testing**
  - [ ] Install ONNX runtime (`ort` crate)
  - [ ] Download MiniLM-L6 model (80MB)
  - [ ] Benchmark embedding generation (CPU vs GPU)
  - [ ] **Gate check:** If CPU latency >150ms, requires GPU or skip

- [ ] **Week 6: Implementation**
  - [ ] Add `embedding` BLOB column to `conversation_turns`
  - [ ] Implement async embedding generation
  - [ ] Add embedding cache (avoid recomputation)
  - [ ] Implement hybrid search (BM25 + semantic)
  - [ ] Write 8 tests (semantic similarity, multilingual)

- [ ] **Week 7: A/B Testing**
  - [ ] Deploy hybrid search to 50% of users
  - [ ] Log BM25 vs semantic result differences
  - [ ] User satisfaction survey (5 questions)
  - [ ] **Gate check:** If <20% improvement, rollback

---

## RECOMMENDED NEXT STEPS

### Immediate Actions (This Sprint)

1. **Create Proof-of-Concept BM25 Implementation (4 hours)**
   - Minimal `InvertedIndex` with 3 test queries
   - Compare latency vs current LIKE queries
   - Validate TDF alignment with concrete metrics

2. **Log Current LIKE Query Performance (2 hours)**
   - Add instrumentation to `search_warm_memory`, `search_cold_memory`
   - Collect P50/P95/P99 latency on dev database
   - Identify worst-case queries (latency outliers)

3. **Build Golden Dataset for Evaluation (3 hours)**
   - Create 50 conversation turns with known topics
   - Define 10 test queries with manually labeled relevance
   - Establish baseline precision@5, recall@5 for LIKE queries

### Phase 1 Implementation (Sprint 1-2)

**Goal:** Replace LIKE queries with BM25 + temporal decay

**Deliverables:**
- `search_index.rs` module (300 LOC, 11 tests)
- Integration into `MemoryTierManager` (50 LOC changes)
- Performance benchmarks (P95 < 15ms)
- Precision@5 > 70% on golden dataset

**Definition of Done:**
- [ ] All 143 existing tests pass
- [ ] 11 new search tests pass
- [ ] Latency benchmarks meet targets
- [ ] Code review approved (2 reviewers)
- [ ] Documentation updated (README, STATUS.md)

### Gate Check for Phase 2 (Semantic Embeddings)

**Proceed ONLY if:**
1. User query logs show semantic gaps (e.g., synonyms, abbreviations)
2. BM25 precision <70% on semantic queries
3. GPU available OR CPU latency acceptable (<150ms)
4. Team bandwidth for 3-week implementation

**Do NOT proceed if:**
- BM25 solves 90%+ of cases (YAGNI principle)
- Latency budget at risk
- Higher priority features in backlog

---

## CONCLUSION

**Top Recommendation:** Implement **BM25 + Temporal Decay** as foundation
- **Rationale:** Proven algorithm, low latency, clear TDF alignment
- **Implementation:** 2-3 weeks, medium complexity
- **Expected Impact:** 10-20x faster, relevance ranking, temporal awareness

**Optional Phase 2:** Add **Semantic Embeddings** if data shows need
- **Gating Criteria:** Semantic gaps in user queries, GPU available
- **Hybrid Approach:** BM25 for exact matches, embeddings for semantic
- **Expected Impact:** +20-30% improvement on semantic queries

**Avoid for Now:** Graph-based retrieval
- **Rationale:** High complexity, quadratic scaling, uncertain benefit
- **Reconsider:** Only if users have dense (>10k turns), interconnected history

---

## APPENDICES

### A. BM25 Algorithm Details

**BM25 Formula (Okapi BM25):**

```
score(D, Q) = Σ IDF(qᵢ) × (f(qᵢ, D) × (k₁ + 1)) / (f(qᵢ, D) + k₁ × (1 - b + b × (|D| / avgdl)))
```

Where:
- `D`: Document (conversation turn)
- `Q`: Query (user input)
- `qᵢ`: Query term i
- `f(qᵢ, D)`: Term frequency of qᵢ in D
- `|D|`: Length of document D (token count)
- `avgdl`: Average document length in corpus
- `k₁`: Term frequency saturation (typical: 1.2-2.0)
- `b`: Length normalization (typical: 0.75)
- `IDF(qᵢ)`: Inverse document frequency

**IDF Formula:**

```
IDF(qᵢ) = ln((N - n(qᵢ) + 0.5) / (n(qᵢ) + 0.5))
```

Where:
- `N`: Total documents in corpus
- `n(qᵢ)`: Number of documents containing qᵢ

**Temporal Decay Formula:**

```
temporal_weight(t) = exp(-λ × t)
```

Where:
- `t`: Age of document in hours
- `λ`: Decay rate (typical: 0.001-0.1)
  - λ=0.001: Slow decay (half-life ~700 hours = 1 month)
  - λ=0.01: Medium decay (half-life ~70 hours = 3 days)
  - λ=0.1: Fast decay (half-life ~7 hours)

### B. Hyperparameter Tuning Guide

**BM25 Parameters:**

| Parameter | Range | Default | Tuning Method |
|-----------|-------|---------|---------------|
| k₁ | 1.2 - 2.0 | 1.5 | Grid search on golden dataset |
| b | 0.5 - 0.9 | 0.75 | Grid search on golden dataset |

**Temporal Decay:**

| λ Value | Half-Life | Use Case |
|---------|-----------|----------|
| 0.001 | 29 days | Long-term projects, research |
| 0.01 | 3 days | Daily standup context |
| 0.1 | 7 hours | Real-time conversation |

**Recommended Tuning Process:**
1. Collect 100 historical queries with manual relevance labels
2. Grid search: k₁ ∈ {1.2, 1.5, 1.8, 2.0}, b ∈ {0.5, 0.75, 0.9}
3. For each (k₁, b), compute MAP (Mean Average Precision)
4. Select parameters with highest MAP
5. A/B test in production (1 week)

### C. Rust Dependencies Analysis

**Recommended Crates:**

```toml
[dependencies]
# BM25 Implementation
rust-stemmers = "1.2"         # Porter stemmer: 50KB, pure Rust, no deps
unicode-segmentation = "1.10"  # Unicode-aware tokenization: 200KB

# Optional: Semantic Embeddings
ort = "2.0"                    # ONNX runtime: 5MB, CPU/GPU inference
instant-distance = "0.6"       # HNSW index: 100KB, fast ANN search

# Optional: PostgreSQL Vector (if migrating from SQLite)
pgvector = "0.4"               # pgvector Rust bindings: 50KB
```

**Avoided Crates (and why):**
- `rust-bert`: 1.5GB model download, slow cold start
- `tantivy`: 2MB, full-text search overkill (designed for multi-GB corpora)
- `qdrant-client`: Requires external service (complexity)

### D. Database Schema Changes

**BM25 Index Persistence (Optional):**

```sql
-- Materialized inverted index (for fast cold starts)
CREATE TABLE IF NOT EXISTS search_index_terms (
    term TEXT PRIMARY KEY,
    document_frequency INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS search_index_postings (
    term TEXT NOT NULL,
    turn_id BLOB NOT NULL,
    term_frequency INTEGER NOT NULL,
    PRIMARY KEY (term, turn_id),
    FOREIGN KEY (turn_id) REFERENCES conversation_turns(id) ON DELETE CASCADE
);

CREATE INDEX idx_postings_term ON search_index_postings(term);
```

**Semantic Embeddings (Phase 2):**

```sql
-- Add embedding column
ALTER TABLE conversation_turns
ADD COLUMN embedding BLOB;  -- 384 floats × 4 bytes = 1536 bytes

-- Index for exact search (SQLite doesn't support vector ops, so use external index)
-- For PostgreSQL with pgvector:
-- CREATE EXTENSION vector;
-- ALTER TABLE conversation_turns ADD COLUMN embedding vector(384);
-- CREATE INDEX ON conversation_turns USING ivfflat (embedding vector_cosine_ops);
```

### E. Monitoring & Observability

**Metrics to Track:**

```rust
// Add to VifApi or MemoryTierManager
struct SearchMetrics {
    query_count: AtomicU64,
    latency_histogram: Histogram,
    result_count_histogram: Histogram,
    fallback_count: AtomicU64,  // How often index fails, falls back to LIKE
}

impl SearchMetrics {
    fn record_search(&self, latency_ms: f64, result_count: usize, used_fallback: bool) {
        self.query_count.fetch_add(1, Ordering::Relaxed);
        self.latency_histogram.record(latency_ms);
        self.result_count_histogram.record(result_count);
        if used_fallback {
            self.fallback_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn report(&self) -> String {
        format!(
            "Search Metrics: {} queries, P95 latency: {:.2}ms, fallback rate: {:.1}%",
            self.query_count.load(Ordering::Relaxed),
            self.latency_histogram.percentile(0.95),
            100.0 * self.fallback_count.load(Ordering::Relaxed) as f64
                / self.query_count.load(Ordering::Relaxed) as f64
        )
    }
}
```

**Log Examples:**

```
[INFO] BM25 search: query="quantum computing" session=abc123 latency=8.5ms results=5 scores=[0.82, 0.71, 0.65, 0.43, 0.39]
[WARN] BM25 index unavailable, falling back to LIKE query: session=abc123 latency=145ms
[ERROR] BM25 search failed: IndexCorrupted { term: "quantum", reason: "postings list truncated" }
```

---

**Report Compiled By:** Computational Domain Expert (COMP)
**TDF Alignment Check:**
- ✅ COMP: Formal algorithms, performance modeling, measurable metrics
- ✅ SCI: Evidence-based recommendations, benchmark references
- ✅ EXP: Temporal decay models human memory
- ✅ META: Self-awareness of uncertainty (gating criteria, A/B tests)

**Confidence Level:** 85% (BM25 recommendation), 65% (semantic embeddings), 40% (graph approach)

**Next Action Required:** Stakeholder decision on Phase 1 (BM25) implementation timeline
