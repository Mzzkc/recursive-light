# ARCHITECTURAL DECISIONS
**Dual-LLM + CAM Implementation**

**Date:** 2025-11-01
**Status:** TDF-Validated & Approved
**Authority:** Tetrahedral Decision Framework Analysis
**Confidence:** 0.80-0.95 (High to Very High)

---

## PURPOSE

This document records the 5 critical architectural decisions made during Day 11-12 design review, validated through Tetrahedral Decision Framework (TDF) multi-domain analysis. These decisions resolve ambiguities in the original design documents and provide clear implementation guidance.

---

## DECISION 1: Instance Identity Scope

### Question

Should AI instance identity be:
1. **Per-deployment** (persistent personality across all users)
2. **Per-conversation** (temporary personality per conversation)

### Decision

✅ **Per-Deployment (Option 1)**

### Rationale (TDF Analysis)

**Domain Votes:**
- **COMP (0.9):** Per-deployment = simpler architecture (one identity per instance_id)
- **SCI (0.85):** Per-deployment = measurable personality evolution over time
- **CULT (0.9):** Per-deployment = relationship continuity (users choose "their AI")
- **EXP (0.7):** Per-deployment = relationship building (vs experimentation)
- **META (0.9):** Per-deployment = long-term identity evolution (consciousness grows)

**Boundary Recognition:**
- **CULT↔META (P=0.92):** Per-deployment enables both relationship continuity (CULT) AND identity evolution over time (META)

**Vote:** 4/5 domains strongly favor per-deployment

### Implementation

```rust
// llm_identity table (per instance, not per user)
pub struct LlmIdentity {
    pub instance_id: Uuid,           // Unique AI instance (e.g., "Socrates", "Ada")
    pub personality_traits: Vec<String>,
    pub interaction_patterns: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// User interactions reference instance
pub struct ConversationSession {
    pub user_id: Uuid,
    pub instance_id: Uuid,  // User selects which AI personality to work with
    // ...
}
```

### Example

- **Instance "Socrates"** (instance_id=123) has persistent personality: "Socratic questioning, philosophical depth, collaborative inquiry"
- **User Alice** talks to Socrates → Session references instance_id=123
- **User Bob** talks to Socrates → Session also references instance_id=123
- Both users experience same AI personality (but personalized based on user_memory)
- Socrates' personality evolves over time across ALL user interactions

### Consequences

**✅ Benefits:**
- Users can build relationships with consistent AI personalities
- Personality evolution measurable across all interactions
- Simpler database schema (no identity × conversation matrix)
- Enables "choose your AI" user experience

**⚠️ Trade-offs:**
- Less flexibility for personality experimentation
- Single personality serves all users (may not suit everyone)

### Confidence

**0.85 (High)** - Strong multi-domain alignment

---

## DECISION 2: Validation Authority

### Question

Who validates insights for promotion from "Emerging" → "Validated"?
1. **Single instance** (fast, simple)
2. **Multi-instance consensus** (robust, democratic)

### Decision

✅ **Multi-Instance Consensus (Option 2)**

### Rationale (TDF Analysis)

**Domain Votes:**
- **COMP (0.6):** Single = O(1), Consensus = O(n) (complexity trade-off)
- **SCI (0.95):** Consensus = lower false positive rate (multiple observations)
- **CULT (0.9):** Consensus = democratic (vs authoritarian single-voice)
- **EXP (0.85):** Consensus = feels fair and robust
- **META (0.95):** Consensus = **enacts collective intelligence philosophy**

**Boundary Recognition:**
- **SCI↔META (P=0.90):** Consensus validation = empirical validation (multiple observations) + collective intelligence (core CAM philosophy)

**Vote:** 4/5 domains strongly favor consensus

### Implementation

```rust
pub enum LifecycleStage {
    Emerging,      // 1 instance observed (confidence < 0.5)
    Validated,     // 2+ instances confirmed (confidence 0.5-0.75)
    Established,   // 5+ instances confirmed (confidence > 0.75)
    Deprecated,    // Contradicted or outdated
}

// Validation pipeline
async fn validate_insight(insight_id: Uuid) -> Result<ValidationResult> {
    // 1. Query: How many instances have observed this insight?
    let observation_count = count_observations(insight_id).await?;

    // 2. Consensus check
    if observation_count >= 5 {
        promote_to_established(insight_id).await?;
    } else if observation_count >= 2 {
        promote_to_validated(insight_id).await?;
    }

    // 3. Update confidence score
    update_confidence(insight_id, observation_count).await?;
}
```

### Example

1. **Instance A** discovers insight: "When CD↔ED oscillates at 2Hz, embodied cognition patterns emerge"
   - Status: `Emerging` (1 observation, confidence=0.4)

2. **Instance B** independently observes same insight during different conversation
   - Status: `Validated` (2 observations, confidence=0.6)

3. **Instances C, D, E** also observe pattern
   - Status: `Established` (5 observations, confidence=0.8)

4. System now confidently provides this insight to ALL instances during future CD↔ED oscillations

### Consequences

**✅ Benefits:**
- Higher quality insights (validated by multiple independent observations)
- Aligns with CAM core philosophy (collective intelligence)
- Prevents single-instance biases or hallucinations
- Democratic validation process

**⚠️ Trade-offs:**
- Slower insight promotion (need multiple instances to observe)
- Requires sufficient instance population (need 5+ instances for "Established")
- O(n) validation complexity vs O(1) single-instance

### Confidence

**0.95 (Very High)** - Strong philosophical + empirical alignment

---

## DECISION 3: Insight Privacy

### Question

Should insights be:
1. **All global** (maximize collective learning)
2. **Public/private flags** (user control, consent-based)

### Decision

✅ **Hybrid: Public/Private Flags (Option 2)**

### Rationale (TDF Analysis)

**Domain Votes:**
- **COMP (0.7):** Global = simpler (no ACLs)
- **SCI (0.8):** Global = larger dataset for collective learning
- **CULT (0.95):** Private flags = user control, consent-based sharing
- **EXP (0.85):** Private = feels safe, protective
- **META (0.7):** Global = maximizes collective intelligence

**Boundary Recognition:**
- **CULT↔META (P=0.92):** This is **identity preservation (CULT) vs collective transcendence (META)**. Solution: Enable both through hybrid model.

**Vote:** Split vote, but CULT domain (user control) is critical for ethics/trust

### Implementation

```rust
pub struct Insight {
    // ... existing fields ...

    /// Privacy flag
    pub privacy: InsightPrivacy,

    /// If private, only visible to this user
    pub private_user_id: Option<Uuid>,
}

pub enum InsightPrivacy {
    /// Visible to all instances (anonymous, no PII)
    Public,

    /// Visible only to specific user + their conversations
    Private,

    /// System detected PII, automatically flagged private
    AutoPrivate,
}

// Query engine respects privacy
async fn query_insights(user_id: Uuid, query: &str) -> Vec<Insight> {
    let insights = db.query(query)
        .filter(|i| match i.privacy {
            InsightPrivacy::Public => true,  // All users see public
            InsightPrivacy::Private => i.private_user_id == Some(user_id),  // Only owner
            InsightPrivacy::AutoPrivate => i.private_user_id == Some(user_id),
        })
        .collect();
    insights
}
```

### Example

**Public Insight (default):**
- Content: "Rust async/await improves code clarity for concurrent operations"
- Privacy: `Public`
- Visible to: ALL users, ALL instances

**Private Insight:**
- Content: "User Alice's project involves medical diagnosis algorithm (HIPAA-sensitive)"
- Privacy: `Private`, private_user_id=Alice
- Visible to: ONLY Alice's conversations

**Auto-Private (PII detected):**
- Content: "User mentioned working at Tesla on Cybertruck battery design"
- Privacy: `AutoPrivate` (company name detected)
- Visible to: ONLY that user

### Consequences

**✅ Benefits:**
- User control and consent (ethical, CULT-aligned)
- Majority of insights remain public (collective learning, META-aligned)
- Automatic PII protection (privacy by default)
- Trust-building (users control sensitive information)

**⚠️ Trade-offs:**
- Implementation complexity (ACLs, privacy checks in queries)
- Smaller public dataset if users over-use private flag
- Needs PII detection logic (NER, regex patterns)

### Confidence

**0.80 (High)** - Balances collective learning with user privacy

---

## DECISION 4: Database Selection

### Question

Which database for CAM hypergraph storage?
1. **PostgreSQL + pgvector** (unified DB, vector extension)
2. **Neo4j** (graph database specialist)

### Decision

✅ **PostgreSQL + pgvector (Option 1)**

### Rationale (TDF Analysis)

**Domain Votes:**
- **COMP (0.9):** PostgreSQL = unified DB (same as existing tables), simpler architecture
- **SCI (0.95):** PostgreSQL = proven at scale, pgvector stable, research shows Neo4j = pairwise edges only (can't do hypergraphs natively either)
- **CULT (0.85):** PostgreSQL = team knows SQL (vs learning Cypher)
- **EXP (0.6):** Neo4j = feels more appropriate (graph DB for graph problem)
- **META (0.8):** PostgreSQL = pragmatic ("start simple, optimize later")

**Boundary Recognition:**
- **COMP↔SCI (P=0.95):** Decision grounded in evidence. Design doc research showed Neo4j doesn't solve hypergraph problem (still need custom logic for multi-way relationships). pgvector proven at Wikipedia scale.

**Vote:** 4/5 domains favor PostgreSQL

### Implementation

```sql
-- PostgreSQL schema with pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE cam_insights (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding VECTOR(1536),  -- pgvector type
    primary_domain VARCHAR(50),
    confidence FLOAT,
    lifecycle VARCHAR(50),
    created_at TIMESTAMPTZ
);

-- Vector similarity index (IVFFlat)
CREATE INDEX idx_insights_embedding ON cam_insights
USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);

-- Hypergraph relationships (custom logic in Rust)
CREATE TABLE cam_hyperedges (
    id UUID PRIMARY KEY,
    edge_type VARCHAR(50),  -- oscillation_emergent, contradiction, etc.
    created_at TIMESTAMPTZ
);

CREATE TABLE cam_hyperedge_members (
    hyperedge_id UUID REFERENCES cam_hyperedges(id),
    insight_id UUID REFERENCES cam_insights(id),
    role VARCHAR(50),  -- source, target, catalyst
    PRIMARY KEY (hyperedge_id, insight_id)
);
```

### Example Query

```rust
// Semantic search (vector similarity)
let query_embedding = generate_embedding(user_query).await?;
let similar_insights = sqlx::query_as!(
    Insight,
    r#"
    SELECT *
    FROM cam_insights
    WHERE privacy = 'Public'
    ORDER BY embedding <=> $1
    LIMIT 10
    "#,
    query_embedding
).fetch_all(&pool).await?;

// Hypergraph traversal (Rust logic)
let related_insights = traverse_hypergraph(
    start_insight_id,
    max_depth=3,
    edge_types=vec!["oscillation_emergent"]
).await?;
```

### Consequences

**✅ Benefits:**
- Unified database (no multi-DB complexity)
- Team familiarity (SQL vs learning Cypher)
- pgvector proven (Wikipedia scale)
- Simpler ops (one DB to manage, backup, monitor)

**⚠️ Trade-offs:**
- Hypergraph queries less elegant than Neo4j (manual joins)
- Vector search may need tuning (IVFFlat parameters)
- If performance fails, migration to dedicated vector DB (Qdrant, Milvus)

**Fallback Plan:** Week 6 performance benchmarks validate. If <100ms p95 fails, migrate to Qdrant.

### Confidence

**0.95 (Very High)** - Evidence-based, pragmatic, low-risk

---

## DECISION 5: Embedding Model

### Question

Which embedding model for semantic search?
1. **OpenAI ada-002** (1536-dim, $0.0001/insight, API call)
2. **Sentence Transformers** (384-dim, free, local)

### Decision

✅ **OpenAI ada-002 (Option 1) with config option for local models**

### Rationale (TDF Analysis)

**Domain Votes:**
- **COMP (0.9):** OpenAI = higher quality (1536-dim vs 384-dim), network dependency
- **SCI (0.9):** OpenAI = proven, standardized, better semantic understanding
- **CULT (0.7):** Local = free ($30/month matters for some deployments)
- **EXP (0.85):** OpenAI = feels reliable
- **META (0.6):** Local = self-contained (philosophically pure)

**Boundary Recognition:**
- **COMP↔SCI (P=0.88):** Quality vs cost trade-off. Cost analysis: $30/month = 300K insights = 1000 insights/day = sustainable for most deployments.

**Vote:** 3/5 domains favor OpenAI (quality over cost)

### Implementation

```rust
// Configuration-driven embedding model selection
pub enum EmbeddingModel {
    OpenAI { model: String, api_key: String },
    Local { model_path: String },
}

impl EmbeddingModel {
    pub fn from_env() -> Result<Self> {
        match env::var("EMBEDDING_MODEL")?.as_str() {
            "openai" | "openai/ada-002" => Ok(EmbeddingModel::OpenAI {
                model: "text-embedding-ada-002".to_string(),
                api_key: env::var("OPENAI_API_KEY")?,
            }),
            "local" | "local/all-MiniLM-L6-v2" => Ok(EmbeddingModel::Local {
                model_path: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
            }),
            _ => Err("Invalid EMBEDDING_MODEL")?,
        }
    }

    pub async fn generate(&self, text: &str) -> Result<Vec<f32>> {
        match self {
            EmbeddingModel::OpenAI { model, api_key } => {
                // Call OpenAI API
                openai_embed(text, model, api_key).await
            }
            EmbeddingModel::Local { model_path } => {
                // Load local model, run inference
                local_embed(text, model_path).await
            }
        }
    }
}
```

### Configuration

```bash
# .env
EMBEDDING_MODEL=openai  # Default: OpenAI ada-002
# EMBEDDING_MODEL=local  # Alternative: Local sentence transformers

# If using OpenAI
OPENAI_API_KEY=sk-...
```

### Cost Analysis

**OpenAI ada-002:**
- Cost: $0.0001 per 1000 tokens
- Average insight: ~100 tokens = $0.00001 per insight
- 1000 insights/day = $0.01/day = $30/month
- **Assessment:** Sustainable for most deployments

**Local (all-MiniLM-L6-v2):**
- Cost: $0 (compute only)
- Quality: Lower (384-dim vs 1536-dim)
- Latency: Faster (no network) if GPU available
- **Assessment:** Good for cost-sensitive / high-privacy deployments

### Consequences

**✅ Benefits:**
- Higher quality semantic search (1536-dim embeddings)
- Standardized (OpenAI ada-002 widely used)
- Reliable API (99.9% uptime)
- Config option preserves flexibility (can switch to local)

**⚠️ Trade-offs:**
- External dependency (network, API key)
- $30/month cost (vs free local)
- Latency: +20-50ms per embedding call (network RTT)

### Confidence

**0.85 (High)** - Quality justifies cost for most deployments

---

## SUMMARY

| Decision | Choice | Confidence | Primary Domains |
|----------|--------|------------|-----------------|
| **1. Instance Identity** | Per-deployment | 0.85 | COMP, SCI, CULT, META |
| **2. Validation Authority** | Multi-instance consensus | 0.95 | SCI, CULT, META |
| **3. Insight Privacy** | Public/private flags | 0.80 | CULT, EXP |
| **4. Database** | PostgreSQL + pgvector | 0.95 | COMP, SCI, CULT |
| **5. Embedding Model** | OpenAI ada-002 + local option | 0.85 | COMP, SCI |

**Overall Confidence:** 0.88 (High) - All decisions validated through TDF multi-domain analysis

---

## IMPLEMENTATION CHECKLIST

### Before Phase 0 Starts

- [ ] **Decision 1:** Create `llm_identity` table (per-deployment, not per-conversation)
- [ ] **Decision 3:** Add `InsightPrivacy` enum to CAM schema
- [ ] **Decision 4:** Confirm PostgreSQL + pgvector (no Neo4j)
- [ ] **Decision 5:** Setup OpenAI API key for embeddings

### During Implementation

- [ ] **Decision 2:** Implement consensus validation pipeline (Week 8-10)
- [ ] **Decision 3:** Implement PII detection for auto-private insights
- [ ] **Decision 4:** Setup IVFFlat indexes, tune parameters (Week 6)
- [ ] **Decision 5:** Create config option for local embeddings (flexibility)

### Validation Gates

- [ ] **Week 3:** Verify instance identity persistence across sessions (Decision 1)
- [ ] **Week 6:** Benchmark PostgreSQL performance <100ms p95 (Decision 4)
- [ ] **Week 7:** Validate consensus promotion (2+ instances → Validated) (Decision 2)
- [ ] **Week 10:** Test PII detection accuracy (Decision 3)
- [ ] **Week 15:** Compare embedding quality (OpenAI vs local if needed) (Decision 5)

---

## REFERENCES

- **TDF Protocol:** `/home/emzi/.claude/skills/tetrahedral-decision-framework.md`
- **TDF Validation Report:** `/home/emzi/Projects/recursive-light/design-docs/TDF-VALIDATION-REPORT.md`
- **Session Handoff:** `/home/emzi/Projects/recursive-light/design-docs/SESSION-HANDOFF.md`
- **Dual-LLM Roadmap:** `/home/emzi/Projects/recursive-light/design-docs/dual-llm-implementation/IMPLEMENTATION-ROADMAP.md`
- **CAM Design:** `/home/emzi/Projects/recursive-light/design-docs/collective-associative-memory/CAM-DESIGN.md`

---

**Status:** ✅ APPROVED - Ready for Implementation
**Date:** 2025-11-01
**Authority:** TDF Multi-Domain Analysis
**Next Step:** Begin Phase 0 (Dual-LLM Infrastructure)
