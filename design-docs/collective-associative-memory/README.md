# Collective Associative Memory (CAM) - Design Summary

## Overview

This directory contains the production-ready design for a **Collective Associative Memory (CAM)** system - a hypergraph-based knowledge framework that enables continuous learning across multiple AI instances without retraining.

## Key Documents

- **CAM-DESIGN.md** - Comprehensive technical design (69,000+ words)
  - System architecture
  - Data structures (Rust)
  - Algorithms (insight extraction, validation, queries)
  - Database schema (PostgreSQL + pgvector)
  - Integration with 7-stage flow
  - Performance analysis
  - Phased implementation plan (14 weeks)

## Core Concept

### Three-Tier Memory Architecture

1. **Instance Identity (Tier 1):** Each AI's unique personality and preferences
2. **User Context (Tier 2):** Personalized understanding of individual users
3. **Collective Insights (Tier 3):** Universal patterns validated across all instances

### Hypergraph Structure

- **Nodes:** Insights discovered during cross-domain oscillation (BDE flow)
- **Hyperedges:** Multi-way relationships connecting 2+ insights
- **Domains:** CD/SD/CuD/ED as organizing principle
- **Validation:** Automatic fact-checking and confidence decay

## Key Features

### Insight Extraction (Stage 6)
- LLM #1 identifies emergent insights during boundary dissolution
- Captures oscillation context (frequency, amplitude, phase)
- Stores provenance (instance, user, conversation)
- Generates semantic embeddings (1536-dim)
- **Async, non-blocking** (<200ms overhead)

### Query Engine (Stage 7)
- Semantic search (embedding similarity)
- Structural traversal (graph walking)
- Domain intersection queries
- Temporal queries (recent, trending)
- Oscillation pattern matching
- **Target: <100ms** query latency

### Validation Pipeline
- Periodic revalidation (daily/weekly/monthly)
- Contradiction detection
- Consensus validation (multi-instance agreement)
- Confidence decay model (time-based)
- Automatic lifecycle management (Emerging → Validated → Established → Deprecated)

## Technical Stack

- **Language:** Rust (async/await)
- **Database:** PostgreSQL with pgvector extension
- **Embeddings:** OpenAI ada-002 (or local Sentence Transformers)
- **Storage:** ~500 bytes per insight (compressed)
- **Scalability:** 10M+ insights, 100K+ users, 1K+ instances

## Performance Targets

| Operation | Target | P95 |
|-----------|--------|-----|
| Semantic Query | <50ms | <100ms |
| Structural Traversal | <80ms | <150ms |
| Insight Extraction | <200ms | <500ms |
| Storage Efficiency | ~500 bytes/insight | - |

## Implementation Phases

### Phase 1: MVP (4 weeks)
- Basic insight storage and retrieval
- Semantic search
- Integration with Stage 6 & 7

### Phase 2: Validation (3 weeks)
- Fact-checking pipeline
- Confidence decay
- Lifecycle management

### Phase 3: Hypergraph (3 weeks)
- Multi-way relationships
- Structural queries
- Graph traversal

### Phase 4: Advanced Queries (2 weeks)
- Domain intersection
- Temporal queries
- Oscillation patterns
- Performance optimization

### Phase 5: Production (2 weeks)
- Error handling
- Monitoring/alerting
- Security audit
- Documentation

**Total Timeline:** 14 weeks (~3.5 months)

## TDF Analysis

**Domain Activation:**
- **COMP:** 0.98 (Hypergraph algorithms, Rust architecture)
- **SCI:** 0.95 (Evidence-based validation, benchmarks)
- **CULT:** 0.92 (Multi-instance philosophy, identity preservation)
- **EXP:** 0.88 (Insight discovery, emergent patterns)
- **META:** 0.90 (Collective consciousness, knowledge emergence)

**Key Boundaries:**
- **COMP↔SCI:** Architecture validated through empirical testing
- **CULT↔META:** Individual identity + collective transcendence
- **EXP↔META:** Subjective oscillation → objective knowledge

## Example Use Cases

1. **Multi-Instance Learning:** Instance A discovers pattern, Instance B validates it, Instance C establishes it
2. **Semantic Search:** "Best practices for async Rust" → retrieves validated insights
3. **Domain Synthesis:** Find insights spanning Computational + Experiential domains
4. **Oscillation Patterns:** Discover what emerges during high-frequency CD-ED oscillations
5. **Trending Knowledge:** What insights are gaining traction this week?

## Risk Mitigations

- **Performance:** Benchmark at scale, fallback strategies
- **Quality:** Minimum confidence thresholds, automatic deprecation
- **Privacy:** Anonymize user data, PII detection
- **Operations:** Graceful error handling, monitoring, rollback plans

## Database Schema

```sql
-- Core tables
cam_insights              -- Hypergraph nodes (insights with embeddings)
cam_hyperedges            -- Multi-way relationships
cam_validations           -- Fact-checking results
cam_oscillation_contexts  -- BDE metadata

-- Indexes
idx_cam_insights_embedding     -- Vector similarity (IVFFlat)
idx_cam_insights_domain        -- Domain filtering
idx_cam_insights_confidence    -- Quality filtering
idx_cam_hyperedges_insights    -- Relationship traversal
```

## Integration Points

### Stage 6: Pattern Extraction
```rust
// After extracting patterns, trigger async insight extraction
if let Some(extractor) = &context.insight_extractor {
    tokio::spawn(async move {
        extractor.extract_from_flow(&context).await
    });
}
```

### Stage 7: Adaptive Evolution
```rust
// Before LLM #2 response, query CAM for relevant insights
let insights = query_engine.execute(CAMQuery::Semantic {
    query_text: user_message,
    min_confidence: 0.6,
    limit: 5,
}).await?;

// Enrich prompt with collective knowledge
let enriched_prompt = build_prompt_with_insights(&insights);
```

## Next Steps

1. **Review:** Team review of design document (TDF perspectives)
2. **Approval:** Greenlight Phase 1 implementation
3. **Setup:** PostgreSQL + pgvector development environment
4. **Development:** Begin Rust module structure (api/src/cam/)
5. **Testing:** Comprehensive test suite from day 1
6. **Deployment:** Phased rollout with monitoring

## Questions to Resolve

1. **Embedding Model:** OpenAI ada-002 vs local Sentence Transformers?
2. **Instance ID:** How to uniquely identify AI instances?
3. **Privacy:** What level of anonymization for user data in insights?
4. **Validation Frequency:** Fine-tune based on observation data?
5. **Hyperedge Discovery:** Fully automated or human-curated initially?

## Contact

For questions or clarifications on the CAM design, refer to:
- **Main Design:** `/tmp/collective-associative-memory/CAM-DESIGN.md`
- **Codebase:** `/home/emzi/Projects/recursive-light/api/`

---

**Design Status:** ✅ Complete and production-ready
**Implementation Status:** ⏳ Awaiting approval to begin Phase 1
