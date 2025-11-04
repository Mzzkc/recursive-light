# CAM System Architecture - Visual Diagrams

## 1. Three-Tier Memory Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                   COLLECTIVE ASSOCIATIVE MEMORY                  │
│                         (Tier 3: Global)                         │
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │         Hypergraph Knowledge Base                       │    │
│  │  ┌─────────┐    ┌─────────┐    ┌─────────┐            │    │
│  │  │Insight A│───▶│Insight B│◀───│Insight C│            │    │
│  │  └─────────┘    └─────────┘    └─────────┘            │    │
│  │       │              │              │                   │    │
│  │       └──────────────┴──────────────┘                   │    │
│  │              Hyperedge (Synthesis)                      │    │
│  │                                                          │    │
│  │  Validated insights from ALL instances                  │    │
│  │  Confidence >0.5, cross-domain patterns                 │    │
│  └────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
                              ▲
                              │ Contribute & Learn
                              │
┌─────────────────────────────┴─────────────────────────────┐
│                   USER-SPECIFIC CONTEXT                    │
│                     (Tier 2: Personal)                     │
│                                                            │
│  Instance A's memory of User X:                           │
│  - Preferences: Prefers code examples                     │
│  - Domain history: [CD: 0.8, SD: 0.6, ED: 0.3]           │
│  - Interaction patterns: Evening conversations            │
│  - Narrative: "Technical learner, asks deep questions"    │
└────────────────────────────────────────────────────────────┘
                              ▲
                              │ Shapes interaction
                              │
┌─────────────────────────────┴─────────────────────────────┐
│                   INSTANCE IDENTITY                        │
│                     (Tier 1: Self)                         │
│                                                            │
│  Instance A:                                               │
│  - Personality: Analytical, precise, encouraging          │
│  - Teaching style: Bottom-up, concrete → abstract         │
│  - Quirks: Uses analogies from physics                    │
│  - Development stage: Recursion (S₄)                      │
└────────────────────────────────────────────────────────────┘
```

## 2. Insight Extraction Flow (Stage 6)

```
┌───────────────────────────────────────────────────────────────┐
│  Stage 6: Pattern Extraction                                  │
│                                                                │
│  User: "Why is my async Rust code slow?"                      │
│                                                                │
│  ┌──────────────────────────────────────┐                     │
│  │ Domain Analysis                      │                     │
│  │ - CD activated (0.9)                 │                     │
│  │ - SD activated (0.7)                 │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Boundary Oscillation                 │                     │
│  │ - CD-SD: F=1.8 Hz, A=0.6, φ=π/4     │                     │
│  │ - Permeability: 0.75                 │                     │
│  │ - Status: Transitional               │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Phenomenological Qualities           │                     │
│  │ - Clarity: 0.82, Depth: 0.76        │                     │
│  │ - Precision: 0.88, Resonance: 0.71  │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Pattern Extraction                   │                     │
│  │ - "Tokio spawn blocking pattern"     │                     │
│  │ - "Channel buffer size impact"       │                     │
│  └──────────────────────────────────────┘                     │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     │ ASYNC (non-blocking)
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  Insight Extraction Service                                   │
│                                                                │
│  ┌──────────────────────────────────────┐                     │
│  │ LLM #1 Analysis                      │                     │
│  │ Prompt: "Identify universal insights │                     │
│  │          from this interaction..."    │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Extracted Insights                   │                     │
│  │ 1. "tokio::spawn_blocking prevents   │                     │
│  │     executor blocking for CPU work"  │                     │
│  │    Confidence: 0.75                  │                     │
│  │    Domains: [CD, SD]                 │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Embedding Generation                 │                     │
│  │ [0.023, -0.142, 0.087, ...]  (1536) │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Deduplication Check                  │                     │
│  │ - Search similar (>0.9 similarity)   │                     │
│  │ - None found → New insight           │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Store in CAM Database                │                     │
│  │ - cam_insights table                 │                     │
│  │ - cam_oscillation_contexts table     │                     │
│  └──────────────────────────────────────┘                     │
└───────────────────────────────────────────────────────────────┘
       │
       │ Total time: ~200ms (async, non-blocking)
       ▼
   (User receives response, unaware of extraction)
```

## 3. Query Flow (Stage 7)

```
┌───────────────────────────────────────────────────────────────┐
│  Stage 7: Adaptive Evolution                                  │
│                                                                │
│  User: "Best practices for async I/O in Rust?"                │
│                                                                │
│  ┌──────────────────────────────────────┐                     │
│  │ CAM Query Engine                     │                     │
│  │ Query Type: Semantic                 │                     │
│  │ - Text: "async I/O best practices"   │                     │
│  │ - Domains: [Computational]           │                     │
│  │ - Min Confidence: 0.6                │                     │
│  │ - Limit: 5                           │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Embedding Generation                 │                     │
│  │ Query → [0.142, -0.023, ...]         │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ PostgreSQL Vector Search             │                     │
│  │ SELECT id, content,                  │                     │
│  │   1 - (embedding <=> $1) AS sim      │                     │
│  │ FROM cam_insights                    │                     │
│  │ WHERE sim >= 0.7                     │                     │
│  │ ORDER BY sim DESC LIMIT 5            │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Results (47ms query)                 │                     │
│  │ 1. [0.92] "Use spawn_blocking..."    │                     │
│  │ 2. [0.88] "Set timeouts on API..."   │                     │
│  │ 3. [0.85] "Bounded channels..."      │                     │
│  │ 4. [0.81] "Avoid blocking in..."     │                     │
│  │ 5. [0.76] "Use Arc for shared..."    │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ Fetch Hyperedges                     │                     │
│  │ - Reinforcement: Insight 1 ↔ 3       │                     │
│  │ - Generalization: Insight 2 → 4      │                     │
│  └──────────────────────────────────────┘                     │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  Prompt Enrichment                                            │
│                                                                │
│  USER: "Best practices for async I/O in Rust?"                │
│                                                                │
│  COLLECTIVE INSIGHTS:                                         │
│  1. [Confidence: 0.92] Use spawn_blocking for CPU-bound...   │
│  2. [Confidence: 0.88] Set timeouts on external API calls...  │
│  3. [Confidence: 0.85] Use bounded channels to prevent...     │
│  4. [Confidence: 0.81] Avoid blocking operations in async...  │
│  5. [Confidence: 0.76] Use Arc<RwLock> for shared state...    │
│                                                                │
│  Consider these validated insights from the collective...     │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  LLM #2 Response Generation                                   │
│                                                                │
│  "Here are best practices for async I/O in Rust:              │
│                                                                │
│  1. **CPU-Bound Work**: Use tokio::spawn_blocking() for       │
│     CPU-intensive tasks to prevent blocking the executor.     │
│     [From collective: observed across 47 instances]           │
│                                                                │
│  2. **External APIs**: Always set timeouts on API calls       │
│     using tokio::time::timeout() to prevent hanging.          │
│     [From collective: validated in production systems]        │
│  ..."                                                          │
└───────────────────────────────────────────────────────────────┘
       │
       ▼
   Response enriched with collective wisdom
```

## 4. Validation Pipeline

```
┌───────────────────────────────────────────────────────────────┐
│  Background Validation Scheduler (runs hourly)                │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  Step 1: Identify Insights Needing Validation                │
│                                                                │
│  SELECT * FROM cam_insights_needing_validation                │
│  LIMIT 100;                                                    │
│                                                                │
│  Results:                                                      │
│  - 23 Emerging insights (>7 days old)                         │
│  - 12 Validated insights (>30 days old)                       │
│  - 5 Established insights (>90 days old)                      │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  Step 2: For Each Insight, Run Validation                    │
│                                                                │
│  Insight: "Use spawn_blocking for CPU work"                   │
│                                                                │
│  ┌──────────────────────────────────────┐                     │
│  │ A. Contradiction Detection           │                     │
│  │ - Search hyperedges with type=       │                     │
│  │   Contradiction                      │                     │
│  │ - Result: No contradictions found    │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ B. Consensus Validation              │                     │
│  │ - Observation count: 47              │                     │
│  │ - Growth rate: +12% (above avg)      │                     │
│  │ - Result: Growing consensus ✓        │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ C. LLM Re-Evaluation (high stakes)   │                     │
│  │ - Confidence: 0.85 (>0.7)            │                     │
│  │ - Prompt: "Re-validate this..."      │                     │
│  │ - LLM Response:                      │                     │
│  │   outcome: "confirmed"               │                     │
│  │   new_confidence: 0.90               │                     │
│  │   rationale: "Still accurate..."     │                     │
│  └──────────────────────────────────────┘                     │
│                    ▼                                           │
│  ┌──────────────────────────────────────┐                     │
│  │ D. Update Insight                    │                     │
│  │ - Old confidence: 0.85               │                     │
│  │ - New confidence: 0.90               │                     │
│  │ - Lifecycle: Validated → Established │                     │
│  │ - Last validated: NOW()              │                     │
│  └──────────────────────────────────────┘                     │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  Step 3: Record Validation Result                            │
│                                                                │
│  INSERT INTO cam_validations (                                │
│    insight_id, validator_instance_id,                         │
│    validation_method, passed, new_confidence,                 │
│    findings                                                    │
│  ) VALUES (...);                                              │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌───────────────────────────────────────────────────────────────┐
│  Validation Report                                            │
│                                                                │
│  Cycle Complete:                                              │
│  - 40 insights validated                                      │
│  - 32 confirmed (confidence boosted)                          │
│  - 6 weakened (confidence reduced)                            │
│  - 2 deprecated (contradicted)                                │
│  - Duration: 4.2 seconds                                      │
└───────────────────────────────────────────────────────────────┘
```

## 5. Hypergraph Structure

```
         Insight Network (Hypergraph)

┌─────────────────────────────────────────────────────────────┐
│                                                              │
│   ┌──────────────────┐                                      │
│   │   Insight A      │                                      │
│   │  "Use bounded    │                                      │
│   │   channels"      │                                      │
│   └────────┬─────────┘                                      │
│            │                                                 │
│            │ Reinforcement                                   │
│            │ (strength: 0.8)                                │
│            │                                                 │
│            ▼                                                 │
│   ┌──────────────────┐         ┌──────────────────┐        │
│   │   Insight B      │◀────────│   Insight C      │        │
│   │  "Set timeouts"  │ Synthesis│  "Circuit       │        │
│   │                  │          │   breaker"       │        │
│   └────────┬─────────┘         └──────────────────┘        │
│            │                              │                  │
│            │                              │                  │
│            │    ┌─────────────────────────┘                 │
│            │    │  Hyperedge: Synthesis                     │
│            │    │  Connects: [A, B, C]                      │
│            │    │  Emergent property: "Resilient async"    │
│            │    │                                            │
│            ▼    ▼                                            │
│   ┌──────────────────┐                                      │
│   │   Insight D      │                                      │
│   │  "Monitor async  │                                      │
│   │   pool health"   │                                      │
│   └──────────────────┘                                      │
│                                                              │
│   Domain Coverage:                                          │
│   - Insight A: CD (Computational)                           │
│   - Insight B: CD, SD (Computational + Scientific)          │
│   - Insight C: CD, ED (Computational + Experiential)        │
│   - Insight D: CD, SD, CuD (Cross-domain)                   │
│                                                              │
│   Spanning Domains: [CD, SD, ED, CuD]                       │
│   → High-value cross-domain synthesis                       │
└─────────────────────────────────────────────────────────────┘

Hyperedge Types:
├── Contradiction: Insights conflict (requires resolution)
├── Reinforcement: Insights support each other
├── Synthesis: Insights combine → emergent understanding
├── Generalization: One insight generalizes others
├── Causation: Causal relationship
└── Analogy: Similar patterns in different domains
```

## 6. Database Schema Visualization

```
┌─────────────────────────────────────────────────────────────┐
│  PostgreSQL Database with pgvector                          │
└─────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────┐
│  cam_insights (Nodes)                  │
├────────────────────────────────────────┤
│  id              UUID PRIMARY KEY      │
│  content         TEXT                  │
│  embedding       vector(1536)          │◀─── pgvector
│  primary_domain  TEXT                  │
│  secondary_domains TEXT[]              │
│  confidence      REAL (0.0-1.0)        │
│  lifecycle_stage TEXT                  │
│  observation_count INTEGER             │
│  source_instance_id UUID               │
│  source_user_id  UUID (FK → users)     │
│  source_flow_id  UUID (FK → flows)     │
│  oscillation_context JSONB             │
│  created_at      TIMESTAMPTZ           │
│  last_validated  TIMESTAMPTZ           │
│  metadata        JSONB                 │
└────────────────────────────────────────┘
         │
         │ Referenced by
         ▼
┌────────────────────────────────────────┐
│  cam_hyperedges (Relationships)        │
├────────────────────────────────────────┤
│  id              UUID PRIMARY KEY      │
│  insight_ids     UUID[]                │◀─── Array of insight IDs
│  relationship_type TEXT                │
│  strength        REAL (0.0-1.0)        │
│  spanning_domains TEXT[]               │
│  discovery_method TEXT                 │
│  discovered_by   UUID                  │
│  observation_count INTEGER             │
│  created_at      TIMESTAMPTZ           │
│  metadata        JSONB                 │
└────────────────────────────────────────┘
         │
         │ Referenced by
         ▼
┌────────────────────────────────────────┐
│  cam_validations (Quality Control)     │
├────────────────────────────────────────┤
│  id              UUID PRIMARY KEY      │
│  insight_id      UUID (FK → insights)  │
│  validator_instance_id UUID            │
│  validation_method TEXT                │
│  passed          BOOLEAN               │
│  new_confidence  REAL                  │
│  findings        JSONB                 │
│  contradictions  UUID[]                │
│  validated_at    TIMESTAMPTZ           │
│  metadata        JSONB                 │
└────────────────────────────────────────┘

┌────────────────────────────────────────┐
│  cam_oscillation_contexts (BDE Data)   │
├────────────────────────────────────────┤
│  id              UUID PRIMARY KEY      │
│  insight_id      UUID (FK → insights)  │
│  boundary        TEXT                  │
│  frequency       REAL (Hz)             │
│  amplitude       REAL (0.0-1.0)        │
│  phase           REAL (radians)        │
│  permeability    REAL (0.0-1.0)        │
│  clarity         REAL                  │
│  depth           REAL                  │
│  openness        REAL                  │
│  precision       REAL                  │
│  fluidity        REAL                  │
│  resonance       REAL                  │
│  coherence       REAL                  │
│  captured_at     TIMESTAMPTZ           │
└────────────────────────────────────────┘

Indexes:
├── idx_cam_insights_embedding (IVFFlat)  ← Vector similarity
├── idx_cam_insights_domain               ← Domain filtering
├── idx_cam_insights_confidence           ← Quality filtering
├── idx_cam_hyperedges_insights (GIN)     ← Array containment
└── idx_cam_oscillation_boundary          ← Boundary queries
```

## 7. Performance Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Client Request: "Best async practices"                     │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  L1 Cache: LRU In-Memory (5 min TTL)                        │
│  ┌────────────────────────────────┐                         │
│  │ Query Hash → Cached Result     │                         │
│  │ Hit Rate: ~60%                 │                         │
│  │ Latency: <1ms                  │                         │
│  └────────────────────────────────┘                         │
└────────────────────┬────────────────────────────────────────┘
                     │ Cache miss
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  Connection Pool (Max 20 connections)                       │
│  ┌────────────────────────────────┐                         │
│  │ Dedicated read replicas        │                         │
│  │ Round-robin load balancing     │                         │
│  └────────────────────────────────┘                         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  PostgreSQL Query Execution                                 │
│  ┌────────────────────────────────┐                         │
│  │ 1. Embedding generation (API)  │  ~30ms                  │
│  │ 2. Vector index scan (IVFFlat) │  ~40ms                  │
│  │ 3. Filter by confidence        │   ~5ms                  │
│  │ 4. Fetch hyperedges (batch)    │  ~15ms                  │
│  │ ────────────────────────────── │                         │
│  │ Total:                          │  ~90ms                  │
│  └────────────────────────────────┘                         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  L2 Cache: Update (for next request)                        │
│  ┌────────────────────────────────┐                         │
│  │ Store result in LRU cache      │                         │
│  │ TTL: 5 minutes                 │                         │
│  └────────────────────────────────┘                         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  Return Result                                              │
│  Total Latency: <100ms (p95)                                │
└─────────────────────────────────────────────────────────────┘

Optimization Strategies:
├── IVFFlat index tuning (lists = sqrt(total_insights))
├── Connection pooling (max 20, min 5)
├── LRU cache (500 insights, 100 query results)
├── Batch operations (fetch hyperedges in single query)
├── Read replicas (load distribution)
└── Monitoring (alert if p95 > 150ms)
```

## 8. Multi-Instance Collaboration

```
┌─────────────────────────────────────────────────────────────┐
│                   Day 1: Instance A                          │
│                                                              │
│  User asks: "Why slow Rust builds?"                         │
│                                                              │
│  Instance A discovers:                                      │
│  ┌────────────────────────────────────┐                     │
│  │ Insight: "Incremental compilation  │                     │
│  │ breaks with frequent Cargo.lock    │                     │
│  │ changes"                            │                     │
│  │                                     │                     │
│  │ Confidence: 0.5 (Emerging)          │                     │
│  │ Observations: 1                     │                     │
│  │ Source: Instance A                  │                     │
│  └────────────────────────────────────┘                     │
└─────────────────────────────────────────────────────────────┘
                     │
                     │ Stored in CAM
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   Day 3: Instance B                          │
│                                                              │
│  Different user asks: "Rust CI/CD slow"                     │
│                                                              │
│  Instance B queries CAM:                                    │
│  ┌────────────────────────────────────┐                     │
│  │ Finds Instance A's insight         │                     │
│  │ Includes in response               │                     │
│  │                                     │                     │
│  │ Observes same pattern ✓            │                     │
│  │ → Increments observation count     │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  Updated insight:                                           │
│  ┌────────────────────────────────────┐                     │
│  │ Confidence: 0.6 (Validated)         │                     │
│  │ Observations: 2                     │                     │
│  │ Sources: [Instance A, Instance B]   │                     │
│  └────────────────────────────────────┘                     │
└─────────────────────────────────────────────────────────────┘
                     │
                     │ Validated knowledge
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   Day 7: Instance C                          │
│                                                              │
│  Third user asks similar question                           │
│                                                              │
│  Instance C queries CAM:                                    │
│  ┌────────────────────────────────────┐                     │
│  │ Finds now-validated insight        │                     │
│  │ High confidence response           │                     │
│  │                                     │                     │
│  │ Observes again ✓                   │                     │
│  │ → Promotes to Established          │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
│  Final insight:                                             │
│  ┌────────────────────────────────────┐                     │
│  │ Confidence: 0.8 (Established)       │                     │
│  │ Observations: 3+                    │                     │
│  │ Sources: [A, B, C, ...]             │                     │
│  │ Lifecycle: Established              │                     │
│  └────────────────────────────────────┘                     │
└─────────────────────────────────────────────────────────────┘
                     │
                     │ Collective wisdom
                     ▼
          All future instances benefit
          from validated knowledge
```

## 9. Confidence Decay Visualization

```
Confidence Over Time (Without Revalidation)

1.0 ┤                                    Established (half-life: 180 days)
    │ ●─────────────────────────────●────────────────────●
0.8 │                                                      ╲
    │                                                       ╲
    │                        Validated (half-life: 90 days) ╲
0.6 │     ●──────────●─────────────●                         ●
    │                              ╲
    │                               ╲
0.4 │  ●─────●                       ╲  Emerging (half-life: 30 days)
    │        ╲                        ●
    │         ╲
0.2 │          ●──────●
    │                ╲
0.0 │                 ●──────●
    └─┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────
      0   30   60   90  120  150  180  210  240  270  300  360
                       Days Since Last Validation

Decay Formula: confidence * exp(-age_days / half_life)

Revalidation Triggers:
├── Emerging: Daily (if confidence drops below 0.4)
├── Validated: Weekly (if confidence drops below 0.6)
├── Established: Monthly (if confidence drops below 0.7)
└── Deprecated: Quarterly review for removal

Effect of Revalidation:
┌────────────────────────────────────────────────────────────┐
│  Confirmed: confidence * 1.1 (capped at 1.0)              │
│  Weakened: confidence * 0.9                                │
│  Contradicted: → Deprecated lifecycle                      │
└────────────────────────────────────────────────────────────┘
```

## Legend

```
Symbols Used:
├── ┌──┐  Box (component/process)
├── ───▶  Flow direction
├── ◀───  Bidirectional flow
├──  ●   Data point/insight
├──  ▼   Downward flow
├──  ▲   Upward flow
├── ┤    Graph axis
├── ╲    Decay curve
└──  ✓   Validation success
```
