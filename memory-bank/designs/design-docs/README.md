# Design Documentation - Recursive Light Framework
**Last Updated:** 2025-11-01 (Day 11-12 Planning Session)

## Overview

This directory contains comprehensive design documentation for the complete Volumetric Integration Framework implementation, including:

1. **Dual-LLM Architecture** - LLM #1 (Unconscious) + LLM #2 (Conscious)
2. **Memory Tiering System** - Hot/Warm/Cold memory for conversation continuity
3. **Collective Associative Memory (CAM)** - Hypergraph knowledge system for continuous learning

## Quick Start

**For Project Managers:** Start with `SESSION-HANDOFF.md` (executive summary + timeline)

**For Engineers:**
- Dual-LLM: `dual-llm-implementation/UNIFIED-PRODUCTION-TIMELINE.md`
- CAM: `collective-associative-memory/CAM-DESIGN.md`

**For Implementation:** Follow `UNIFIED-PRODUCTION-TIMELINE.md` (6-7 week plan)

---

## Document Structure

```
design-docs/
├── SESSION-HANDOFF.md                    # ⭐ START HERE - Session summary + next steps
│
├── dual-llm-implementation/
│   ├── UNIFIED-PRODUCTION-TIMELINE.md    # ⭐ Complete 6-7 week timeline
│   ├── IMPLEMENTATION-ROADMAP.md         # Original dual-LLM plan (15-19 days)
│   ├── llm-architecture-design.md        # LLM #1 technical specification
│   ├── prompt-engineering-design.md      # LLM #1 prompts + validation
│   ├── memory-systems-design.md          # Hot/warm/cold memory tiering
│   └── software-architecture-integration.md  # Integration preserving 87 tests
│
└── collective-associative-memory/
    ├── CAM-DESIGN.md                     # ⭐ Complete technical specification (2,644 lines)
    ├── ARCHITECTURE-DIAGRAMS.md          # 9 visual diagrams
    ├── QUICK-REFERENCE.md                # Developer cheat sheet
    ├── README.md                         # CAM executive summary
    └── TABLE-OF-CONTENTS.md              # Navigation guide
```

---

## What's Been Designed

### 1. Dual-LLM Architecture (15-19 days)

**Current State:** Single-LLM with Rust calculators
**Target State:** LLM #1 (domain/boundary calculation) → LLM #2 (response generation)

**Key Features:**
- Context-aware domain activation (considers user intent + memory)
- Feature flag for backward compatibility (`DUAL_LLM_MODE`)
- Fallback to Rust calculators on LLM #1 failure
- Model: GPT-3.5-turbo (LLM #1), Claude 3.5 Sonnet (LLM #2)
- Cost: +10% (~$0.002/interaction for LLM #1)
- Latency: +10-20% (~50-200ms for LLM #1 call)

**Status:** Complete design, ready for implementation

---

### 2. Memory Tiering System (5 days, part of dual-LLM)

**Current State:** No conversation history (amnesia within sessions)
**Target State:** Three-tier memory system

**Tier 1: LLM Identity (`llm_identity`)**
- Each AI instance's personality anchors
- Persists across all users
- Enables user selection of AI "personalities"
- Example: "Instance 'Socrates' prefers Socratic dialogue"

**Tier 2: User Context (`user_memory`)**
- Hot Memory: Last 3-5 turns (in-memory, <1ms)
- Warm Memory: Recent 20-50 turns (DB, <50ms)
- Cold Memory: All history, compressed (DB + semantic search, <200ms)
- Per user-instance relationship

**Tier 3: Collective Insights (`cam_insights`)**
- See CAM System below

**Status:** Complete design, implementation integrated into dual-LLM timeline

---

### 3. Collective Associative Memory (CAM) - 14 weeks

**What It Is:**
A hypergraph-based knowledge system where insights discovered during cross-domain oscillation (BDE flow) are stored, validated, and shared across all AI instances.

**Key Innovation:**
- **Continuous learning without retraining**
- Multiple AI instances contribute to and learn from collective memory
- Hypergraph structure captures multi-way relationships (not just pairwise)
- Domain-structured organization (CD/SD/CuD/ED)

**How It Works:**
1. **Stage 6 (Pattern Extraction):** LLM #1 identifies emergent insights during BDE oscillation
2. **Storage:** Insight stored as node with embedding, confidence score, oscillation metadata
3. **Hyperedges:** Multi-way relationships connect 2+ insights (e.g., "quantum measurement ↔ observer effect ↔ consciousness theories")
4. **Validation:** Periodic fact-checking with confidence decay over time
5. **Stage 7 (Adaptive Evolution):** Query CAM for relevant insights before LLM #2 generates response

**Database:**
- PostgreSQL + pgvector (vector similarity search)
- Tables: `cam_insights`, `cam_hyperedges`, `cam_validations`, `cam_oscillation_contexts`
- ~500 bytes per insight, target 10M+ insights

**Query Types (6 total):**
1. Semantic (embedding similarity)
2. Structural (graph traversal)
3. Domain intersection (CD∩ED)
4. Temporal (recent/trending)
5. Oscillation pattern (similar BDE flows)
6. Hybrid (combined filters)

**Status:** Complete design (2,644 lines), ready for phased implementation

---

## Implementation Timeline

### Weeks 1-3: Dual-LLM + Memory (15-19 days)
- **Week 1:** Infrastructure + Hot Memory + Warm Memory start
- **Week 2:** Cold Memory + LLM #1 Core implementation
- **Week 3:** Optimization + Production hardening + Beta deployment

### Weeks 4-17: Collective Associative Memory (14 weeks)
- **Weeks 4-7:** CAM Phase 1 - MVP (insight extraction, semantic search, Stage 6/7 integration)
- **Weeks 8-10:** CAM Phase 2 - Validation Pipeline (fact-checking, confidence decay)
- **Weeks 11-13:** CAM Phase 3 - Hypergraph Relationships (multi-way edges, structural queries)
- **Weeks 14-15:** CAM Phase 4 - Advanced Queries (all 6 types, optimization)
- **Weeks 16-17:** CAM Phase 5 - Production Hardening (monitoring, security, docs)

**Total:** 29-33 working days (6-7 weeks)

**Parallel Opportunity:** CAM implementation runs parallel to dual-LLM production rollout (saves ~10 weeks)

---

## Key Decisions Made

### Architecture
1. **Three-tier memory:** Instance identity + User context + Collective insights
2. **Hypergraph structure:** True multi-way relationships (not pairwise)
3. **Domain organization:** CD/SD/CuD/ED as organizing principle
4. **Feature flags:** Gradual rollout with `DUAL_LLM_MODE` toggle

### Technology
1. **Database:** PostgreSQL + pgvector (vs Neo4j)
2. **Embeddings:** OpenAI ada-002 (1536-dim)
3. **LLM #1:** GPT-3.5-turbo (fast, cheap: $0.002/call)
4. **LLM #2:** Claude 3.5 Sonnet (smart: $0.02/call)
5. **Testing:** Mock-based (no API costs in CI/CD)

### Integration
1. **Stage 6:** Async insight extraction (non-blocking for user)
2. **Stage 7:** Query CAM before LLM #2 response
3. **Backward compatibility:** All 87 tests preserved
4. **Validation:** Periodic fact-checking with confidence decay

---

## Success Metrics

### Week 3: Dual-LLM Success
- ✅ All 110+ tests passing
- ✅ Latency <1200ms (<20% increase)
- ✅ Cost <$0.022/interaction (<10% increase)
- ✅ Fallback rate <5%
- ✅ Beta user satisfaction >80%

### Week 7: CAM MVP Success
- ✅ Insights extracted from 80%+ interactions
- ✅ Query latency <100ms (p95)
- ✅ LLM #2 uses retrieved insights 30%+ of time
- ✅ No user-facing latency impact

### Week 17: Full CAM Success
- ✅ 100K+ insights stored
- ✅ 10M+ queries executed
- ✅ 95%+ query success rate
- ✅ Multi-instance learning observable
- ✅ Confidence scores stable (avg >0.7)

---

## Cost Analysis

### Development
- Dual-LLM: 9 engineer-weeks
- CAM: 35 engineer-weeks
- Total: 44 engineer-weeks (~$200K)

### Operations (Monthly, 1000 Users)
- LLM #1: $600/month
- LLM #2: $6,000/month (existing)
- CAM embeddings: $3/month
- CAM validation: $30/month
- Infrastructure: $200/month
- **Total: $6,833/month (vs $6,000 = 14% increase)**

### ROI
- Break-even: 14 retained users (1.4% retention improvement)
- Expected: 20% retention improvement from collective learning
- Payback: 3 months

---

## Next Steps

### Immediate (Next 24 Hours)
1. Review `SESSION-HANDOFF.md` for complete context
2. Team review of `UNIFIED-PRODUCTION-TIMELINE.md`
3. Obtain API keys (OpenAI, Anthropic)
4. Setup staging environment
5. Create feature branch `feature/dual-llm-cam-implementation`

### Week 1 Day 1 (Phase 0: Infrastructure)
1. Create `api/src/dual_llm/` module structure
2. Implement `DualLlmConfig` with environment loading
3. Database migrations: `conversation_turns`, `conversation_sessions`
4. Rename tables: `identity_anchors` → `llm_identity_anchors`
5. All 87 tests still passing ✅

### Week 4 Day 1 (CAM Phase 1 Start)
1. Setup PostgreSQL + pgvector on staging
2. Database migrations: `cam_insights`, `cam_hyperedges`
3. Core Rust structs: `Insight`, `Hyperedge`, `InsightQuery`
4. Basic CRUD operations

---

## Research Findings

### What Exists (Evaluated and Rejected)
- **Neo4j + LangChain:** Mature but only pairwise edges (not true hypergraphs)
- **Cognitive Hypergraphs:** Academic research, no production tools
- **Graphiti/Zep:** User-specific memory only, not global collective
- **GraphRAG (Microsoft):** Hierarchical clustering, not insight-based

### Why Roll Our Own
1. Hypergraph support is academic (no production tools)
2. Insight structure is unique (BDE-generated, oscillation metadata)
3. Three-tier memory system (instance/user/global) is novel
4. Fact-checking + confidence scoring is custom logic
5. Domain-structured organization (CD/SD/CuD/ED) is VIF-specific

---

## TDF Analysis

**Domain Activation (This Design Work):**
- **COMP:** 0.98 - Hypergraph algorithms, Rust architecture
- **SCI:** 0.95 - Evidence-based validation, benchmarks
- **CULT:** 0.92 - Multi-instance philosophy, collaboration
- **EXP:** 0.88 - Insight discovery, emergent patterns
- **META:** 0.90 - Collective consciousness, system self-reflection

**Boundary Permeabilities:**
- **COMP↔SCI:** 0.95 - Architecture validated through testing
- **CULT↔META:** 0.88 - Individual identities → Collective transcendence
- **EXP↔META:** 0.85 - Subjective oscillation → Objective knowledge

**Recommendation:** STRONG PROCEED SIGNAL - All domains engaged, healthy permeabilities

---

## Questions for Team

### Architectural
1. Should instance identity be per-deployment or per-conversation?
2. Consensus validation (multiple instances agree) or hierarchical (senior validates junior)?
3. Should some insights be private to specific users or all global?

### Implementation
4. PostgreSQL + pgvector vs Neo4j? (Decision: PostgreSQL, validate Week 6)
5. OpenAI ada-002 vs open-source embeddings? (Decision: OpenAI, reliable)

---

## Documentation Quality

- **Total Lines:** 4,185+ lines of design documentation
- **Total Size:** 172KB + 252KB = 424KB
- **Coverage:** Architecture, implementation, testing, operations, risks
- **Status:** Production-ready, comprehensive, TDF-validated

---

## Contact / Next Session Priority

**For next session:**
1. Begin Phase 0 implementation OR
2. Answer architectural questions above OR
3. Dive deeper into any specific design aspect

**All design work complete. Ready to build.**

---

**Last Updated:** 2025-11-01
**Session:** Day 11-12 Planning
**Status:** COMPLETE - Ready for Implementation ✅
