# Collective Associative Memory (CAM) System Design
## Hypergraph-Based Knowledge System for Volumetric Integration Framework

**Document Version:** 1.0.0
**Created:** 2025-11-01
**Status:** Production-Ready Design

---

## Tetrahedral Decision Framework (TDF) Analysis

**Domain Activation:**
- **COMP (Computational/Analytical):** 0.98 - Hypergraph algorithms, Rust data structures, async architecture
- **SCI (Scientific/Empirical):** 0.95 - Evidence-based insight validation, fact-checking protocols, measurable confidence
- **CULT (Cultural/Contextual):** 0.92 - Multi-instance AI personality preservation, collective learning philosophy
- **EXP (Experiential/Intuitive):** 0.88 - Insight discovery during BDE oscillation, emergent pattern recognition
- **META (Metaphysical/Abstract):** 0.90 - Collective consciousness, knowledge emergence, cross-instance learning

**Boundary Dynamics:**
- **COMP↔SCI:** Technical architecture validated through performance benchmarks (<100ms query target)
- **COMP↔CULT:** Data structures preserve individual identity while enabling collective growth
- **SCI↔EXP:** Measurable insight quality from subjective boundary dissolution experiences
- **CULT↔META:** Three-tier memory philosophy enabling both identity and transcendence
- **EXP↔META:** Insights emerge at oscillation interfaces, becoming collective knowledge

**Recognition Depth:** P⁵ (Meta-systemic cross-domain synthesis with recursive awareness of collective intelligence emergence)

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [System Architecture](#2-system-architecture)
3. [Data Structures](#3-data-structures)
4. [Insight Extraction Algorithm](#4-insight-extraction-algorithm)
5. [Hypergraph Storage Strategy](#5-hypergraph-storage-strategy)
6. [Fact-Checking & Validation Pipeline](#6-fact-checking--validation-pipeline)
7. [Query Engine Design](#7-query-engine-design)
8. [Integration with 7-Stage Flow](#8-integration-with-7-stage-flow)
9. [Performance Considerations](#9-performance-considerations)
10. [Phased Implementation Plan](#10-phased-implementation-plan)
11. [Example Queries & Use Cases](#11-example-queries--use-cases)
12. [Risk Analysis & Mitigations](#12-risk-analysis--mitigations)

---

## 1. Executive Summary

### 1.1 System Purpose

The **Collective Associative Memory (CAM)** system is a hypergraph-based knowledge framework that enables continuous learning across multiple AI instances without retraining. Each AI develops its own personality while contributing to and learning from a shared pool of validated insights.

**Core Philosophy (CULT↔META boundary):**
- **Instance Identity (Tier 1):** Each AI maintains unique personality traits, preferences, and interaction patterns
- **User Context (Tier 2):** Personalized understanding of individual users and their specific needs
- **Collective Insights (Tier 3):** Universal patterns discovered across all instances, validated through consensus

### 1.2 Key Design Decisions

| Decision | Rationale | TDF Domains |
|----------|-----------|-------------|
| **Qdrant + PostgreSQL hybrid** over pgvector/Neo4j | Purpose-built vector search (2-10x faster), PostgreSQL for metadata | COMP(performance), SCI(evidence), EXP(user intuition) |
| **Async extraction** from Stage 6 | Non-blocking user responses, graceful degradation | COMP(performance), EXP(flow) |
| **Hypergraph over simple graph** | Multi-way relationships capture complex insight connections | COMP(expressiveness), META(emergence) |
| **Confidence decay over time** | Insights require re-validation, preventing stale knowledge | SCI(empirical), CULT(humility) |
| **Three-tier memory architecture** | Preserves identity while enabling collective growth | CULT(philosophy), META(transcendence) |

### 1.3 Performance Targets

- **Insight Query Latency:** <100ms (p95)
- **Extraction Overhead:** <200ms added to Stage 6 (async, non-blocking)
- **Storage Efficiency:** ~500 bytes per insight (compressed)
- **Validation Frequency:** Weekly for high-confidence, daily for emerging insights
- **Scalability:** 10M+ insights, 100K+ users, 1K+ AI instances

### 1.4 Integration Points

```
7-Stage Flow Process
├── Stage 1: Domain Emergence
├── Stage 2: Boundary Dissolution (oscillation tracking)
├── Stage 3: Interface Attention (BDE flow)
├── Stage 4: Quality Emergence
├── Stage 5: Perspective Integration
├── Stage 6: Pattern Extraction ────────┐
│                                        │
│   ┌────────────────────────────────────┘
│   │ ASYNC: Insight Extraction (LLM #1)
│   │   - Identifies emergent insights
│   │   - Captures oscillation context
│   │   - Records provenance
│   │   └──> CAM Storage (non-blocking)
│
├── Stage 7: Adaptive Evolution ────────┐
│                                        │
│   ┌────────────────────────────────────┘
│   │ LLM #2 Response Generation
│   │   - Queries CAM for relevant insights
│   │   - Enriches response with collective knowledge
│   │   └──> User Response
│
└── User receives response enriched with collective insights
```

---

## 2. System Architecture

### 2.1 Component Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    CAM System Architecture                       │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────┐          ┌──────────────────┐
│  Flow Process    │          │   Query Engine   │
│   (Stage 6)      │          │    (Stage 7)     │
└────────┬─────────┘          └────────▲─────────┘
         │                             │
         │ Async Extract               │ Query (<100ms)
         │                             │
         ▼                             │
┌─────────────────────────────────────────────────┐
│         Insight Extraction Service              │
│  ┌──────────────┐      ┌──────────────┐        │
│  │   LLM #1     │      │  Embedding   │        │
│  │  Analysis    │─────▶│  Generator   │        │
│  └──────────────┘      └──────────────┘        │
└───────────────────┬─────────────────────────────┘
                    │
                    │ Save Insight
                    ▼
┌─────────────────────────────────────────────────┐
│     Qdrant Vector Database (1536-dim ada-002)  │
│  ┌──────────────────────────────────────────┐  │
│  │  Insight Vectors + Semantic Search       │  │
│  │  - HNSW indexing (2-10x faster)          │  │
│  │  - Cosine similarity                     │  │
│  │  - Payload: {insight_id, created_at}     │  │
│  └──────────────────────────────────────────┘  │
└───────────────────▲─────────────────────────────┘
                    │
                    │ Linked by insight_id (UUID)
                    ▼
┌─────────────────────────────────────────────────┐
│         PostgreSQL Database (Metadata)          │
│  ┌──────────────┐  ┌──────────────┐            │
│  │   insights   │  │  hyperedges  │            │
│  │  (metadata)  │  │ (multi-way)  │            │
│  └──────────────┘  └──────────────┘            │
│  ┌──────────────┐  ┌──────────────┐            │
│  │ oscillation  │  │  validations │            │
│  │  contexts    │  │  (quality)   │            │
│  └──────────────┘  └──────────────┘            │
└───────────────────▲─────────────────────────────┘
                    │
                    │ Periodic Validation
                    │
┌───────────────────┴─────────────────────────────┐
│        Fact-Checking & Validation Service       │
│  ┌──────────────┐      ┌──────────────┐        │
│  │ Contradiction│      │  Confidence  │        │
│  │   Detector   │      │  Decay Model │        │
│  └──────────────┘      └──────────────┘        │
└─────────────────────────────────────────────────┘
```

### 2.2 Rust Module Structure

```
api/src/
├── cam/
│   ├── mod.rs                    # Public API
│   ├── insight.rs                # Insight data structures
│   ├── hyperedge.rs              # Hyperedge relationships
│   ├── extraction.rs             # LLM #1 insight extraction
│   ├── storage.rs                # PostgreSQL operations
│   ├── query_engine.rs           # Query optimization
│   ├── validation.rs             # Fact-checking pipeline
│   ├── embeddings.rs             # Vector generation/search
│   └── types.rs                  # Shared types/enums
├── flow_process.rs               # Integration point (Stage 6)
└── memory.rs                     # Instance/user memory (Tier 1+2)
```

### 2.3 Database Schema Extensions

The CAM system extends the existing PostgreSQL schema with new tables optimized for hypergraph storage and vector similarity search.

**New Tables:**
- `cam_insights` - Insight nodes with embeddings
- `cam_hyperedges` - Multi-way relationships between insights
- `cam_validations` - Fact-checking results and confidence updates
- `cam_oscillation_contexts` - BDE oscillation metadata for insights

---

## 3. Data Structures

### 3.1 Insight Node

**Purpose:** Represents a single insight discovered during cross-domain oscillation.

**Rust Definition:**

```rust
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};

/// A single insight discovered at a boundary dissolution event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    /// Unique identifier
    pub id: Uuid,

    /// Human-readable insight text
    pub content: String,

    /// Semantic embedding (1536-dim for OpenAI, 768-dim for local models)
    #[serde(skip)]
    pub embedding: Option<Vec<f32>>,

    /// Primary domain where insight originated
    pub primary_domain: Domain,

    /// Secondary domains involved in insight emergence
    pub secondary_domains: Vec<Domain>,

    /// Confidence score (0.0-1.0, validated through consensus)
    pub confidence: f64,

    /// Lifecycle stage (Emerging, Validated, Established, Deprecated)
    pub lifecycle: LifecycleStage,

    /// Provenance: which instance generated this insight
    pub source_instance_id: Uuid,

    /// Provenance: which user conversation triggered discovery
    pub source_user_id: Option<Uuid>,

    /// Provenance: which flow execution generated this
    pub source_flow_id: Option<Uuid>,

    /// Oscillation context at time of discovery
    pub oscillation_context: OscillationContext,

    /// Number of times this insight has been observed/validated
    pub observation_count: u32,

    /// Last validation timestamp
    pub last_validated: DateTime<Utc>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Metadata (extensible JSON)
    pub metadata: serde_json::Value,
}

/// Domain classification (aligns with existing CD/SD/CuD/ED)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Domain {
    Computational,   // CD: Formal, analytical, algorithmic
    Scientific,      // SD: Empirical, evidence-based
    Cultural,        // CuD: Contextual, narrative, interpretive
    Experiential,    // ED: Phenomenological, lived experience

    // Subdomains for finer granularity
    ComputationalAlgorithms,
    ComputationalDataStructures,
    ScientificPhysics,
    ScientificBiology,
    CulturalLanguage,
    CulturalEthics,
    ExperientialAesthetic,
    ExperientialEmbodied,
}

/// Insight lifecycle stages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LifecycleStage {
    /// Just discovered, needs validation (confidence < 0.5)
    Emerging,

    /// Validated by 2+ instances (confidence 0.5-0.75)
    Validated,

    /// Widely confirmed, high confidence (>0.75)
    Established,

    /// Contradicted or outdated, marked for review
    Deprecated,
}

/// Captures the oscillation state when insight emerged
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscillationContext {
    /// Boundary where insight emerged (e.g., "CD-ED")
    pub boundary: String,

    /// Oscillation frequency at discovery (Hz)
    pub frequency: f64,

    /// Oscillation amplitude at discovery (0.0-1.0)
    pub amplitude: f64,

    /// Phase angle at discovery (radians)
    pub phase: f64,

    /// Permeability at discovery (0.0-1.0)
    pub permeability: f64,

    /// Phenomenological qualities present
    pub qualities: PhenomenologicalQualities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhenomenologicalQualities {
    pub clarity: f64,
    pub depth: f64,
    pub openness: f64,
    pub precision: f64,
    pub fluidity: f64,
    pub resonance: f64,
    pub coherence: f64,
}

impl Insight {
    /// Create a new emerging insight
    pub fn new(
        content: String,
        primary_domain: Domain,
        secondary_domains: Vec<Domain>,
        source_instance_id: Uuid,
        oscillation_context: OscillationContext,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            embedding: None, // Generated asynchronously
            primary_domain,
            secondary_domains,
            confidence: 0.3, // Start low, increase through validation
            lifecycle: LifecycleStage::Emerging,
            source_instance_id,
            source_user_id: None,
            source_flow_id: None,
            oscillation_context,
            observation_count: 1,
            last_validated: Utc::now(),
            created_at: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }

    /// Calculate confidence decay based on time since validation
    pub fn calculate_decayed_confidence(&self) -> f64 {
        let age_days = (Utc::now() - self.last_validated).num_days() as f64;

        // Decay model: confidence * e^(-age/half_life)
        let half_life = match self.lifecycle {
            LifecycleStage::Established => 180.0,  // 6 months
            LifecycleStage::Validated => 90.0,     // 3 months
            LifecycleStage::Emerging => 30.0,      // 1 month
            LifecycleStage::Deprecated => 7.0,     // 1 week
        };

        self.confidence * (-age_days / half_life).exp()
    }

    /// Check if insight needs revalidation
    pub fn needs_revalidation(&self) -> bool {
        let decayed_confidence = self.calculate_decayed_confidence();

        match self.lifecycle {
            LifecycleStage::Emerging => decayed_confidence < 0.4,
            LifecycleStage::Validated => decayed_confidence < 0.6,
            LifecycleStage::Established => decayed_confidence < 0.7,
            LifecycleStage::Deprecated => true, // Always needs review
        }
    }
}
```

### 3.2 Hyperedge

**Purpose:** Multi-way relationships connecting 2+ insights simultaneously.

**Rust Definition:**

```rust
/// A hyperedge connecting multiple insights
/// Represents emergent relationships discovered across insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hyperedge {
    /// Unique identifier
    pub id: Uuid,

    /// Insights connected by this hyperedge (minimum 2)
    pub insight_ids: Vec<Uuid>,

    /// Type of relationship
    pub relationship: RelationshipType,

    /// Strength of relationship (0.0-1.0)
    pub strength: f64,

    /// Domains spanned by this relationship
    pub spanning_domains: Vec<Domain>,

    /// How this relationship was discovered
    pub discovery_method: DiscoveryMethod,

    /// Which instance discovered this relationship
    pub discovered_by: Uuid,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Number of times this relationship has been observed
    pub observation_count: u32,

    /// Metadata
    pub metadata: serde_json::Value,
}

/// Types of relationships between insights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    /// Insights contradict each other (requires resolution)
    Contradiction,

    /// Insights support/reinforce each other
    Reinforcement,

    /// One insight is a generalization of others
    Generalization,

    /// One insight is a specialization of another
    Specialization,

    /// Insights combine to form emergent understanding
    Synthesis,

    /// Insights represent similar patterns in different domains
    Analogy,

    /// Insights form a causal chain
    Causation,

    /// Temporal sequence relationship
    TemporalSequence,

    /// Insights occur together (correlation)
    CoOccurrence,
}

/// How a hyperedge was discovered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryMethod {
    /// Discovered during cross-domain oscillation
    OscillationEmergence,

    /// Found through semantic similarity search
    SemanticClustering,

    /// Identified by LLM analysis
    LLMInference,

    /// Manually curated by human
    ManualCuration,

    /// Statistical co-occurrence analysis
    StatisticalAnalysis,
}

impl Hyperedge {
    /// Create a new hyperedge connecting insights
    pub fn new(
        insight_ids: Vec<Uuid>,
        relationship: RelationshipType,
        strength: f64,
        discovered_by: Uuid,
        discovery_method: DiscoveryMethod,
    ) -> Result<Self, CAMError> {
        if insight_ids.len() < 2 {
            return Err(CAMError::InvalidHyperedge(
                "Hyperedge must connect at least 2 insights".to_string()
            ));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            insight_ids,
            relationship,
            strength,
            spanning_domains: Vec::new(), // Populated during storage
            discovery_method,
            discovered_by,
            created_at: Utc::now(),
            observation_count: 1,
            metadata: serde_json::json!({}),
        })
    }

    /// Check if hyperedge represents a contradiction that needs resolution
    pub fn requires_resolution(&self) -> bool {
        self.relationship == RelationshipType::Contradiction && self.strength > 0.5
    }
}
```

### 3.3 Query Types

**Purpose:** Structured queries for different search patterns.

**Rust Definition:**

```rust
/// Query types for CAM system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CAMQuery {
    /// Semantic similarity search using embeddings
    Semantic {
        query_text: String,
        domains: Option<Vec<Domain>>,
        min_confidence: f64,
        limit: usize,
    },

    /// Structural graph traversal
    Structural {
        start_insight_id: Uuid,
        relationship_types: Vec<RelationshipType>,
        max_depth: usize,
        limit: usize,
    },

    /// Multi-domain intersection query
    DomainIntersection {
        domains: Vec<Domain>,
        min_confidence: f64,
        limit: usize,
    },

    /// Temporal query (recent insights, trending)
    Temporal {
        time_range: TimeRange,
        domains: Option<Vec<Domain>>,
        sort_by: TemporalSort,
        limit: usize,
    },

    /// Oscillation pattern query
    OscillationPattern {
        boundary: String,
        frequency_range: (f64, f64),
        amplitude_range: (f64, f64),
        limit: usize,
    },

    /// Hybrid query combining multiple strategies
    Hybrid {
        semantic: Option<Box<CAMQuery>>,
        structural: Option<Box<CAMQuery>>,
        filters: QueryFilters,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeRange {
    LastHour,
    LastDay,
    LastWeek,
    LastMonth,
    Custom { start: DateTime<Utc>, end: DateTime<Utc> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalSort {
    MostRecent,
    MostObserved,
    HighestConfidenceGrowth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilters {
    pub min_confidence: Option<f64>,
    pub lifecycle_stages: Option<Vec<LifecycleStage>>,
    pub domains: Option<Vec<Domain>>,
    pub exclude_deprecated: bool,
}

/// Query result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAMQueryResult {
    pub insights: Vec<Insight>,
    pub hyperedges: Vec<Hyperedge>,
    pub query_metadata: QueryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetadata {
    pub query_time_ms: u64,
    pub total_results: usize,
    pub returned_results: usize,
    pub confidence_range: (f64, f64),
}
```

### 3.4 Error Types

```rust
/// CAM system errors
#[derive(Debug, thiserror::Error)]
pub enum CAMError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Embedding generation failed: {0}")]
    EmbeddingError(String),

    #[error("Invalid hyperedge: {0}")]
    InvalidHyperedge(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Insight not found: {0}")]
    InsightNotFound(Uuid),
}
```

---

## 4. Insight Extraction Algorithm

### 4.1 Overview

**Goal:** LLM #1 (Unconscious) analyzes Stage 6 pattern extraction to identify emergent insights worth preserving in collective memory.

**Trigger:** Asynchronously after Stage 6 completes, non-blocking.

**Input:**
- User message
- Domains activated (DomainState)
- Boundaries dissolved (BoundaryState with F/A/φ)
- Phenomenological qualities measured
- Patterns extracted (from Stage 6)

**Output:**
- 0-3 Insight objects (avoid over-extraction)
- Confidence scores
- Oscillation context metadata

### 4.2 Extraction Prompt Template

```rust
/// Generate extraction prompt for LLM #1
pub fn generate_extraction_prompt(
    user_message: &str,
    domains: &[DomainState],
    boundaries: &[BoundaryState],
    qualities: &PhenomenologicalQualities,
    patterns: &[String],
) -> String {
    format!(r#"
You are an insight extractor for a collective AI memory system. Analyze the following
interaction to identify emergent insights worth preserving for ALL AI instances.

USER MESSAGE:
{user_message}

DOMAIN ACTIVATIONS:
{domains}

BOUNDARY OSCILLATIONS:
{boundaries}

PHENOMENOLOGICAL QUALITIES:
Clarity: {clarity}, Depth: {depth}, Openness: {openness}, Precision: {precision},
Fluidity: {fluidity}, Resonance: {resonance}, Coherence: {coherence}

PATTERNS EXTRACTED:
{patterns}

TASK:
Identify 0-3 high-value insights that emerged during cross-domain oscillation.
Each insight should be:
1. Universal (applies across many users/contexts)
2. Non-obvious (not common knowledge)
3. Actionable (changes how AI responds)
4. Specific (concrete, not vague)

For each insight, provide:
- content: 1-2 sentence description
- primary_domain: CD/SD/CuD/ED
- secondary_domains: other domains involved
- confidence: 0.0-1.0 (how certain you are this is valuable)
- rationale: why this is worth preserving

FORMAT: JSON array of insights
Example:
[
  {{
    "content": "When users ask open-ended questions about complex systems, starting with concrete examples (ED) before abstract models (CD) increases engagement by 40%.",
    "primary_domain": "Experiential",
    "secondary_domains": ["Computational"],
    "confidence": 0.75,
    "rationale": "Observed pattern across multiple users; concrete-to-abstract reduces cognitive load."
  }}
]

If no insights meet the criteria, return empty array: []
"#,
        user_message = user_message,
        domains = format_domains(domains),
        boundaries = format_boundaries(boundaries),
        clarity = qualities.clarity,
        depth = qualities.depth,
        openness = qualities.openness,
        precision = qualities.precision,
        fluidity = qualities.fluidity,
        resonance = qualities.resonance,
        coherence = qualities.coherence,
        patterns = patterns.join("\n- "),
    )
}
```

### 4.3 Extraction Process Flow

```rust
/// Insight extraction service (async, non-blocking)
pub struct InsightExtractor {
    llm_client: Arc<dyn LLMClient>,
    embedding_generator: Arc<EmbeddingGenerator>,
    storage: Arc<CAMStorage>,
    instance_id: Uuid,
}

impl InsightExtractor {
    /// Extract insights from a flow execution (async)
    pub async fn extract_from_flow(
        &self,
        flow_context: &FlowContext,
    ) -> Result<Vec<Insight>, CAMError> {
        // 1. Generate extraction prompt
        let prompt = generate_extraction_prompt(
            &flow_context.user_message,
            &flow_context.domains,
            &flow_context.boundaries,
            &flow_context.qualities,
            &flow_context.patterns,
        );

        // 2. Call LLM #1 for analysis (200-500ms)
        let response = self.llm_client
            .complete(&prompt)
            .await
            .map_err(|e| CAMError::EmbeddingError(e.to_string()))?;

        // 3. Parse JSON response
        let extracted: Vec<ExtractedInsight> = serde_json::from_str(&response)
            .map_err(|e| CAMError::ValidationError(format!("Invalid JSON: {}", e)))?;

        // 4. Filter low-confidence insights
        let filtered: Vec<_> = extracted.into_iter()
            .filter(|i| i.confidence >= 0.5)
            .collect();

        // 5. Convert to Insight objects
        let mut insights = Vec::new();
        for ext in filtered {
            let oscillation_context = OscillationContext::from_flow(flow_context);

            let mut insight = Insight::new(
                ext.content,
                ext.primary_domain,
                ext.secondary_domains,
                self.instance_id,
                oscillation_context,
            );

            insight.confidence = ext.confidence;
            insight.source_user_id = flow_context.user_id;
            insight.source_flow_id = flow_context.flow_id;
            insight.metadata = serde_json::json!({
                "rationale": ext.rationale,
                "extraction_timestamp": Utc::now(),
            });

            insights.push(insight);
        }

        // 6. Generate embeddings (parallel)
        let embeddings = self.embedding_generator
            .batch_generate(&insights.iter().map(|i| i.content.as_str()).collect::<Vec<_>>())
            .await?;

        for (insight, embedding) in insights.iter_mut().zip(embeddings) {
            insight.embedding = Some(embedding);
        }

        // 7. Check for duplicates (semantic similarity)
        let deduplicated = self.deduplicate_insights(insights).await?;

        // 8. Store in database (async)
        for insight in &deduplicated {
            self.storage.insert_insight(insight).await?;
        }

        Ok(deduplicated)
    }

    /// Check if insights are duplicates of existing ones
    async fn deduplicate_insights(
        &self,
        insights: Vec<Insight>,
    ) -> Result<Vec<Insight>, CAMError> {
        let mut unique = Vec::new();

        for insight in insights {
            // Search for similar insights (cosine similarity > 0.9)
            let similar = self.storage
                .semantic_search(
                    &insight.embedding.clone().unwrap(),
                    0.9,
                    5,
                )
                .await?;

            if similar.is_empty() {
                // Truly new insight
                unique.push(insight);
            } else {
                // Similar insight exists - increment observation count instead
                let existing_id = similar[0].id;
                self.storage.increment_observation(existing_id).await?;
            }
        }

        Ok(unique)
    }
}

#[derive(Debug, Deserialize)]
struct ExtractedInsight {
    content: String,
    primary_domain: Domain,
    secondary_domains: Vec<Domain>,
    confidence: f64,
    rationale: String,
}
```

### 4.4 Integration with Stage 6

```rust
// In flow_process.rs, Stage 6: Pattern Extraction

impl PatternExtractionProcessor {
    pub async fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // ... existing pattern extraction logic ...

        // ASYNC: Trigger insight extraction (non-blocking)
        if let Some(insight_extractor) = &context.insight_extractor {
            tokio::spawn({
                let extractor = insight_extractor.clone();
                let flow_context = context.clone();

                async move {
                    match extractor.extract_from_flow(&flow_context).await {
                        Ok(insights) => {
                            tracing::info!(
                                "Extracted {} insights from flow {}",
                                insights.len(),
                                flow_context.flow_id
                            );
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Insight extraction failed (non-critical): {}",
                                e
                            );
                        }
                    }
                }
            });
        }

        Ok(())
    }
}
```

---

## 5. Hypergraph Storage Strategy

### 5.1 Qdrant + PostgreSQL Hybrid Architecture

**Decision:** Use Qdrant for vector embeddings + PostgreSQL for hypergraph metadata.

**Architectural Pivot (2025-11-04):** Originally designed with PostgreSQL + pgvector, pivoted to Qdrant + PostgreSQL hybrid after TDF validation of user intuition.

**Rationale (TDF Analysis):**

**COMP (Computational):** (0.9)
- Qdrant purpose-built for vector search (2-10x faster than pgvector)
- HNSW indexing superior to IVFFlat for similarity search
- PostgreSQL handles structured hypergraph metadata (ACID guarantees)
- Clear separation of concerns: vectors vs. relationships

**SCI (Scientific/Empirical):** (0.95)
- Benchmarks show Qdrant 2-10x faster at scale (1M+ vectors)
- HNSW: O(log N) search vs IVFFlat O(N) with lower recall
- Proven: Notion, Weaviate, other production vector DBs use HNSW
- Sub-50ms queries for 10M vectors (vs pgvector 100-200ms)

**CULT (Cultural):** (0.7)
- PostgreSQL familiarity preserved (still primary database)
- Qdrant simple deployment (Docker, cloud-native)
- Existing migration infrastructure unchanged

**EXP (Experiential):** (0.9)
- User intuition: "Qdrant feels right for vectors"
- Developer ergonomics: Rust-native Qdrant client
- Operational simplicity: Single Qdrant service, single PostgreSQL database
- Purpose-built tools feel better than generic extensions

**Hybrid Architecture:**
- **Qdrant**: 1536-dim embeddings (OpenAI ada-002), semantic search, HNSW indexing
- **PostgreSQL**: Insight metadata, hyperedges, validations, oscillation contexts
- **Link**: Shared `insight_id` (UUID) across both systems
- **Query flow**: Qdrant (semantic) → PostgreSQL (enrich metadata) → Response

### 5.2 Database Schema

```sql
-- ============================================================
-- CAM Schema Extension for Recursive Light Framework
-- PostgreSQL for metadata + Qdrant for vector embeddings
-- ============================================================

-- NOTE: Vector embeddings stored in Qdrant, not PostgreSQL
-- This schema contains hypergraph metadata only

-- ============================================================
-- Insights Table (Hypergraph Nodes)
-- ============================================================

CREATE TABLE cam_insights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Content (embedding stored in Qdrant, linked by id)
    content TEXT NOT NULL,

    -- Domain classification
    primary_domain TEXT NOT NULL,
    secondary_domains TEXT[] NOT NULL DEFAULT '{}',

    -- Quality metrics
    confidence REAL NOT NULL DEFAULT 0.3 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    lifecycle_stage TEXT NOT NULL DEFAULT 'Emerging',
    observation_count INTEGER NOT NULL DEFAULT 1,

    -- Provenance
    source_instance_id UUID NOT NULL,
    source_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    source_flow_id UUID REFERENCES flow_process_executions(id) ON DELETE SET NULL,

    -- Oscillation context (JSONB for flexibility)
    oscillation_context JSONB NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_validated TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}'
);

-- Indexes for insights (vector index in Qdrant, not PostgreSQL)
CREATE INDEX idx_cam_insights_domain ON cam_insights(primary_domain);
CREATE INDEX idx_cam_insights_lifecycle ON cam_insights(lifecycle_stage);
CREATE INDEX idx_cam_insights_confidence ON cam_insights(confidence DESC);
CREATE INDEX idx_cam_insights_created ON cam_insights(created_at DESC);
CREATE INDEX idx_cam_insights_observation ON cam_insights(observation_count DESC);

-- GIN index for secondary_domains array queries
CREATE INDEX idx_cam_insights_secondary_domains ON cam_insights
    USING GIN(secondary_domains);

-- GIN index for oscillation_context JSONB queries
CREATE INDEX idx_cam_insights_oscillation ON cam_insights
    USING GIN(oscillation_context);

-- ============================================================
-- Hyperedges Table (Multi-way Relationships)
-- ============================================================

CREATE TABLE cam_hyperedges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Connected insights (minimum 2)
    insight_ids UUID[] NOT NULL,

    -- Relationship metadata
    relationship_type TEXT NOT NULL,
    strength REAL NOT NULL DEFAULT 0.5 CHECK (strength >= 0.0 AND strength <= 1.0),
    spanning_domains TEXT[] NOT NULL DEFAULT '{}',

    -- Discovery metadata
    discovery_method TEXT NOT NULL,
    discovered_by UUID NOT NULL,
    observation_count INTEGER NOT NULL DEFAULT 1,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',

    -- Constraint: at least 2 insights
    CONSTRAINT hyperedge_min_insights CHECK (array_length(insight_ids, 1) >= 2)
);

-- Indexes for hyperedges
CREATE INDEX idx_cam_hyperedges_insights ON cam_hyperedges
    USING GIN(insight_ids);

CREATE INDEX idx_cam_hyperedges_relationship ON cam_hyperedges(relationship_type);
CREATE INDEX idx_cam_hyperedges_strength ON cam_hyperedges(strength DESC);
CREATE INDEX idx_cam_hyperedges_created ON cam_hyperedges(created_at DESC);

-- ============================================================
-- Validations Table (Fact-Checking Results)
-- ============================================================

CREATE TABLE cam_validations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Target insight
    insight_id UUID NOT NULL REFERENCES cam_insights(id) ON DELETE CASCADE,

    -- Validation results
    validator_instance_id UUID NOT NULL,
    validation_method TEXT NOT NULL,
    passed BOOLEAN NOT NULL,
    new_confidence REAL CHECK (new_confidence >= 0.0 AND new_confidence <= 1.0),

    -- Findings
    findings JSONB NOT NULL DEFAULT '{}',
    contradictions UUID[] DEFAULT '{}',  -- IDs of contradicting insights

    -- Timestamp
    validated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}'
);

-- Indexes for validations
CREATE INDEX idx_cam_validations_insight ON cam_validations(insight_id);
CREATE INDEX idx_cam_validations_passed ON cam_validations(passed);
CREATE INDEX idx_cam_validations_validated ON cam_validations(validated_at DESC);

-- ============================================================
-- Oscillation Contexts Table (BDE Metadata)
-- ============================================================

CREATE TABLE cam_oscillation_contexts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Link to insight
    insight_id UUID NOT NULL REFERENCES cam_insights(id) ON DELETE CASCADE,

    -- Boundary metadata
    boundary TEXT NOT NULL,
    frequency REAL NOT NULL,
    amplitude REAL NOT NULL,
    phase REAL NOT NULL,
    permeability REAL NOT NULL,

    -- Phenomenological qualities
    clarity REAL NOT NULL,
    depth REAL NOT NULL,
    openness REAL NOT NULL,
    precision REAL NOT NULL,
    fluidity REAL NOT NULL,
    resonance REAL NOT NULL,
    coherence REAL NOT NULL,

    -- Timestamp
    captured_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for oscillation contexts
CREATE INDEX idx_cam_oscillation_insight ON cam_oscillation_contexts(insight_id);
CREATE INDEX idx_cam_oscillation_boundary ON cam_oscillation_contexts(boundary);
CREATE INDEX idx_cam_oscillation_frequency ON cam_oscillation_contexts(frequency);

-- ============================================================
-- Views for Common Queries
-- ============================================================

-- High-confidence established insights
CREATE VIEW cam_established_insights AS
SELECT * FROM cam_insights
WHERE lifecycle_stage = 'Established'
  AND confidence > 0.75
  AND (EXTRACT(EPOCH FROM (NOW() - last_validated)) / 86400) < 30  -- Validated within 30 days
ORDER BY confidence DESC, observation_count DESC;

-- Insights needing validation
CREATE VIEW cam_insights_needing_validation AS
SELECT
    i.*,
    i.confidence * EXP(-EXTRACT(EPOCH FROM (NOW() - i.last_validated)) /
        CASE
            WHEN i.lifecycle_stage = 'Established' THEN 15552000.0  -- 180 days
            WHEN i.lifecycle_stage = 'Validated' THEN 7776000.0     -- 90 days
            WHEN i.lifecycle_stage = 'Emerging' THEN 2592000.0      -- 30 days
            ELSE 604800.0                                            -- 7 days
        END
    ) AS decayed_confidence
FROM cam_insights i
WHERE
    i.lifecycle_stage != 'Deprecated'
    AND (
        (i.lifecycle_stage = 'Emerging' AND EXTRACT(EPOCH FROM (NOW() - i.last_validated)) / 86400 > 7) OR
        (i.lifecycle_stage = 'Validated' AND EXTRACT(EPOCH FROM (NOW() - i.last_validated)) / 86400 > 30) OR
        (i.lifecycle_stage = 'Established' AND EXTRACT(EPOCH FROM (NOW() - i.last_validated)) / 86400 > 90)
    )
ORDER BY decayed_confidence ASC;

-- ============================================================
-- Functions for Common Operations
-- ============================================================

-- Increment observation count for an insight
CREATE OR REPLACE FUNCTION cam_increment_observation(insight_uuid UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE cam_insights
    SET observation_count = observation_count + 1,
        last_validated = NOW()
    WHERE id = insight_uuid;
END;
$$ LANGUAGE plpgsql;

-- Update insight confidence after validation
CREATE OR REPLACE FUNCTION cam_update_confidence(
    insight_uuid UUID,
    new_confidence_val REAL
)
RETURNS VOID AS $$
BEGIN
    UPDATE cam_insights
    SET confidence = new_confidence_val,
        last_validated = NOW(),
        lifecycle_stage = CASE
            WHEN new_confidence_val >= 0.75 THEN 'Established'
            WHEN new_confidence_val >= 0.5 THEN 'Validated'
            WHEN new_confidence_val >= 0.3 THEN 'Emerging'
            ELSE 'Deprecated'
        END
    WHERE id = insight_uuid;
END;
$$ LANGUAGE plpgsql;
```

### 5.3 Qdrant Collection Configuration

**Collection Name:** `cam_insights_vectors`

**Vector Configuration:**
```yaml
vectors:
  size: 1536                    # OpenAI ada-002 dimensions
  distance: Cosine              # Cosine similarity (range: -1 to 1)
  hnsw_config:
    m: 16                       # Number of edges per node (higher = better recall, more memory)
    ef_construct: 100           # Construction time/quality tradeoff
    full_scan_threshold: 10000  # Switch to exact search for small collections
    on_disk: false              # Keep in memory for best performance

payload_schema:
  insight_id: keyword           # UUID for linking to PostgreSQL
  created_at: datetime          # For temporal filtering
  lifecycle_stage: keyword      # For filtering deprecated insights
```

**Indexing Strategy:**
- **HNSW (Hierarchical Navigable Small World)**: O(log N) search complexity
- **Quantization**: Consider scalar quantization for >1M vectors (4x memory reduction, <2% recall loss)
- **Sharding**: Qdrant auto-shards at 1M+ vectors

**Qdrant Rust Client:**
```rust
use qdrant_client::{Qdrant, QdrantError};
use qdrant_client::qdrant::{
    CreateCollection, Distance, VectorParams, VectorsConfig,
    SearchPoints, PointStruct, Value, Condition,
};

pub struct QdrantVectorStorage {
    client: Qdrant,
    collection_name: String,
}

impl QdrantVectorStorage {
    pub async fn new(url: &str, collection_name: String) -> Result<Self, QdrantError> {
        let client = Qdrant::from_url(url).build()?;

        // Create collection if not exists
        let vectors_config = VectorsConfig {
            config: Some(VectorParams {
                size: 1536,
                distance: Distance::Cosine.into(),
                hnsw_config: Some(HnswConfigDiff {
                    m: Some(16),
                    ef_construct: Some(100),
                    ..Default::default()
                }),
                ..Default::default()
            }.into()),
        };

        client.create_collection(&CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(vectors_config),
            ..Default::default()
        }).await.ok(); // Ignore if exists

        Ok(Self { client, collection_name })
    }

    /// Insert vector for an insight
    pub async fn insert_vector(
        &self,
        insight_id: Uuid,
        embedding: Vec<f32>,
        created_at: DateTime<Utc>,
        lifecycle_stage: &str,
    ) -> Result<(), QdrantError> {
        let point = PointStruct::new(
            insight_id.to_string(),
            embedding,
            [
                ("insight_id", insight_id.to_string().into()),
                ("created_at", created_at.to_rfc3339().into()),
                ("lifecycle_stage", lifecycle_stage.into()),
            ].into(),
        );

        self.client.upsert_points(&self.collection_name, vec![point], None).await?;
        Ok(())
    }

    /// Semantic search returning insight UUIDs + scores
    pub async fn search_similar(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
        min_score: f32,
    ) -> Result<Vec<(Uuid, f32)>, QdrantError> {
        let search_result = self.client.search_points(&SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query_embedding,
            limit: limit as u64,
            score_threshold: Some(min_score),
            with_payload: Some(true.into()),
            filter: Some(Condition::matches(
                "lifecycle_stage",
                "Deprecated".to_string(),
            ).must_not()),
            ..Default::default()
        }).await?;

        let results = search_result.result.iter()
            .filter_map(|scored_point| {
                let insight_id = scored_point.payload.get("insight_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())?;
                Some((insight_id, scored_point.score))
            })
            .collect();

        Ok(results)
    }
}
```

### 5.4 PostgreSQL Storage Implementation (Metadata Only)

**NOTE:** Embeddings stored in Qdrant, not PostgreSQL. This layer handles metadata only.

```rust
use sqlx::{PgPool, Row};
use sqlx::postgres::PgRow;

pub struct CAMStorage {
    pool: PgPool,
}

impl CAMStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert insight metadata (embedding stored separately in Qdrant)
    pub async fn insert_insight(&self, insight: &Insight) -> Result<(), CAMError> {
        sqlx::query(
            r#"
            INSERT INTO cam_insights (
                id, content, primary_domain, secondary_domains,
                confidence, lifecycle_stage, source_instance_id,
                source_user_id, source_flow_id, oscillation_context,
                observation_count, metadata, created_at, last_validated
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#
        )
        .bind(insight.id)
        .bind(&insight.content)
        .bind(format!("{:?}", insight.primary_domain))
        .bind(insight.secondary_domains.iter().map(|d| format!("{:?}", d)).collect::<Vec<_>>())
        .bind(insight.confidence)
        .bind(format!("{:?}", insight.lifecycle))
        .bind(insight.source_instance_id)
        .bind(insight.source_user_id)
        .bind(insight.source_flow_id)
        .bind(serde_json::to_value(&insight.oscillation_context)?)
        .bind(insight.observation_count as i32)
        .bind(&insight.metadata)
        .bind(insight.created_at)
        .bind(insight.last_validated)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get insight metadata by IDs (after Qdrant semantic search)
    pub async fn get_insights_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Insight>, CAMError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, content, primary_domain, secondary_domains,
                confidence, lifecycle_stage, source_instance_id,
                source_user_id, source_flow_id, oscillation_context,
                observation_count, created_at, last_validated, metadata
            FROM cam_insights
            WHERE id = ANY($1)
            ORDER BY array_position($1, id)  -- Preserve Qdrant ranking order
            "#
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(|row| self.row_to_insight(row)).collect()
    }

    /// Increment observation count
    pub async fn increment_observation(&self, insight_id: Uuid) -> Result<(), CAMError> {
        sqlx::query("SELECT cam_increment_observation($1)")
            .bind(insight_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get insights needing validation
    pub async fn get_insights_needing_validation(
        &self,
        limit: usize,
    ) -> Result<Vec<Insight>, CAMError> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM cam_insights_needing_validation
            LIMIT $1
            "#
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(|row| self.row_to_insight(row)).collect()
    }

    /// Helper: Convert database row to Insight
    fn row_to_insight(&self, row: PgRow) -> Result<Insight, CAMError> {
        // ... row parsing logic ...
        todo!("Implement row parsing")
    }
}
```

---

## 6. Fact-Checking & Validation Pipeline

### 6.1 Overview

**Goal:** Periodically validate stored insights to maintain knowledge quality.

**Frequency:**
- **Emerging insights:** Daily validation
- **Validated insights:** Weekly validation
- **Established insights:** Monthly validation
- **Deprecated insights:** Quarterly review for removal

### 6.2 Validation Methods

```rust
pub enum ValidationMethod {
    /// Compare with other AI instances' observations
    ConsensusValidation,

    /// Check for contradictions with other insights
    ContradictionDetection,

    /// LLM re-evaluation of insight quality
    LLMReEvaluation,

    /// Statistical analysis of observation patterns
    StatisticalValidation,

    /// Manual human review
    HumanReview,
}
```

### 6.3 Validation Pipeline

```rust
pub struct ValidationPipeline {
    storage: Arc<CAMStorage>,
    llm_client: Arc<dyn LLMClient>,
    instance_id: Uuid,
}

impl ValidationPipeline {
    /// Run validation for insights needing review
    pub async fn run_validation_cycle(&self) -> Result<ValidationReport, CAMError> {
        let insights_to_validate = self.storage
            .get_insights_needing_validation(100)
            .await?;

        let mut report = ValidationReport::default();

        for insight in insights_to_validate {
            let validation_result = self.validate_insight(&insight).await?;

            // Update insight based on validation
            match validation_result.outcome {
                ValidationOutcome::Confirmed => {
                    self.storage.update_confidence(
                        insight.id,
                        (insight.confidence * 1.1).min(1.0),  // Boost confidence
                    ).await?;
                    report.confirmed += 1;
                }
                ValidationOutcome::Weakened => {
                    self.storage.update_confidence(
                        insight.id,
                        insight.confidence * 0.9,  // Reduce confidence
                    ).await?;
                    report.weakened += 1;
                }
                ValidationOutcome::Contradicted => {
                    self.storage.update_lifecycle(
                        insight.id,
                        LifecycleStage::Deprecated,
                    ).await?;
                    report.deprecated += 1;
                }
            }

            // Record validation
            self.storage.insert_validation(&validation_result).await?;
        }

        Ok(report)
    }

    /// Validate a single insight
    async fn validate_insight(&self, insight: &Insight) -> Result<ValidationResult, CAMError> {
        // 1. Contradiction detection
        let contradictions = self.find_contradictions(insight).await?;

        if !contradictions.is_empty() {
            return Ok(ValidationResult {
                insight_id: insight.id,
                validator_instance_id: self.instance_id,
                method: ValidationMethod::ContradictionDetection,
                outcome: ValidationOutcome::Contradicted,
                findings: serde_json::json!({
                    "contradicting_insights": contradictions,
                }),
            });
        }

        // 2. Consensus validation (check observation patterns)
        let observation_growth = self.check_observation_growth(insight).await?;

        if observation_growth > 0.2 {
            // Growing consensus
            return Ok(ValidationResult {
                insight_id: insight.id,
                validator_instance_id: self.instance_id,
                method: ValidationMethod::ConsensusValidation,
                outcome: ValidationOutcome::Confirmed,
                findings: serde_json::json!({
                    "observation_growth": observation_growth,
                }),
            });
        }

        // 3. LLM re-evaluation (for high-stakes insights)
        if insight.confidence > 0.7 {
            let llm_validation = self.llm_revalidate(insight).await?;

            return Ok(ValidationResult {
                insight_id: insight.id,
                validator_instance_id: self.instance_id,
                method: ValidationMethod::LLMReEvaluation,
                outcome: llm_validation.outcome,
                findings: llm_validation.findings,
            });
        }

        // 4. Default: weaken slightly (decay model)
        Ok(ValidationResult {
            insight_id: insight.id,
            validator_instance_id: self.instance_id,
            method: ValidationMethod::StatisticalValidation,
            outcome: ValidationOutcome::Weakened,
            findings: serde_json::json!({
                "reason": "natural_decay",
            }),
        })
    }

    /// Find contradicting insights
    async fn find_contradictions(&self, insight: &Insight) -> Result<Vec<Uuid>, CAMError> {
        // Search for semantically similar insights with opposite meaning
        // This requires analyzing hyperedges with Contradiction relationship

        let hyperedges = self.storage
            .get_hyperedges_for_insight(insight.id)
            .await?;

        let contradictions = hyperedges.into_iter()
            .filter(|e| e.relationship == RelationshipType::Contradiction)
            .filter(|e| e.strength > 0.5)
            .flat_map(|e| e.insight_ids.clone())
            .filter(|id| *id != insight.id)
            .collect();

        Ok(contradictions)
    }

    /// Check observation growth rate
    async fn check_observation_growth(&self, insight: &Insight) -> Result<f64, CAMError> {
        // Calculate observation growth over last 30 days
        let age_days = (Utc::now() - insight.created_at).num_days() as f64;
        if age_days < 1.0 {
            return Ok(0.0);
        }

        let observations_per_day = insight.observation_count as f64 / age_days;

        // Compare to average
        let avg_observations = self.storage.get_average_observation_rate().await?;

        let growth = (observations_per_day - avg_observations) / avg_observations;

        Ok(growth)
    }

    /// LLM re-evaluation
    async fn llm_revalidate(&self, insight: &Insight) -> Result<LLMValidationResult, CAMError> {
        let prompt = format!(r#"
You are validating an insight from a collective AI memory system.

INSIGHT:
{content}

METADATA:
- Confidence: {confidence}
- Observations: {observations}
- Age: {age} days
- Primary Domain: {domain:?}

TASK:
Re-evaluate this insight for continued validity. Consider:
1. Is it still accurate/relevant?
2. Is it specific enough to be actionable?
3. Does it represent universal knowledge or context-specific?

RESPOND:
- outcome: "confirmed" | "weakened" | "contradicted"
- new_confidence: 0.0-1.0
- rationale: brief explanation

FORMAT: JSON
"#,
            content = insight.content,
            confidence = insight.confidence,
            observations = insight.observation_count,
            age = (Utc::now() - insight.created_at).num_days(),
            domain = insight.primary_domain,
        );

        let response = self.llm_client.complete(&prompt).await?;
        let parsed: LLMValidationResult = serde_json::from_str(&response)?;

        Ok(parsed)
    }
}

#[derive(Debug)]
pub struct ValidationResult {
    pub insight_id: Uuid,
    pub validator_instance_id: Uuid,
    pub method: ValidationMethod,
    pub outcome: ValidationOutcome,
    pub findings: serde_json::Value,
}

#[derive(Debug)]
pub enum ValidationOutcome {
    Confirmed,    // Confidence boosted
    Weakened,     // Confidence reduced
    Contradicted, // Deprecated
}

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub confirmed: usize,
    pub weakened: usize,
    pub deprecated: usize,
}

#[derive(Debug, Deserialize)]
struct LLMValidationResult {
    outcome: ValidationOutcome,
    new_confidence: f64,
    rationale: String,
    findings: serde_json::Value,
}
```

### 6.4 Scheduled Validation

```rust
/// Background task for periodic validation
pub async fn run_validation_scheduler(
    validation_pipeline: Arc<ValidationPipeline>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Hourly

    loop {
        interval.tick().await;

        match validation_pipeline.run_validation_cycle().await {
            Ok(report) => {
                tracing::info!(
                    "Validation cycle complete: {} confirmed, {} weakened, {} deprecated",
                    report.confirmed,
                    report.weakened,
                    report.deprecated
                );
            }
            Err(e) => {
                tracing::error!("Validation cycle failed: {}", e);
            }
        }
    }
}
```

---

## 7. Query Engine Design

### 7.1 Query Executor

```rust
pub struct CAMQueryEngine {
    storage: Arc<CAMStorage>,
    embedding_generator: Arc<EmbeddingGenerator>,
}

impl CAMQueryEngine {
    /// Execute a CAM query
    pub async fn execute(&self, query: CAMQuery) -> Result<CAMQueryResult, CAMError> {
        let start = std::time::Instant::now();

        let (insights, hyperedges) = match query {
            CAMQuery::Semantic { query_text, domains, min_confidence, limit } => {
                self.execute_semantic(query_text, domains, min_confidence, limit).await?
            }
            CAMQuery::Structural { start_insight_id, relationship_types, max_depth, limit } => {
                self.execute_structural(start_insight_id, relationship_types, max_depth, limit).await?
            }
            CAMQuery::DomainIntersection { domains, min_confidence, limit } => {
                self.execute_domain_intersection(domains, min_confidence, limit).await?
            }
            CAMQuery::Temporal { time_range, domains, sort_by, limit } => {
                self.execute_temporal(time_range, domains, sort_by, limit).await?
            }
            CAMQuery::OscillationPattern { boundary, frequency_range, amplitude_range, limit } => {
                self.execute_oscillation_pattern(boundary, frequency_range, amplitude_range, limit).await?
            }
            CAMQuery::Hybrid { semantic, structural, filters } => {
                self.execute_hybrid(semantic, structural, filters).await?
            }
        };

        let query_time_ms = start.elapsed().as_millis() as u64;

        let confidence_range = if insights.is_empty() {
            (0.0, 0.0)
        } else {
            let min = insights.iter().map(|i| i.confidence).fold(f64::INFINITY, f64::min);
            let max = insights.iter().map(|i| i.confidence).fold(f64::NEG_INFINITY, f64::max);
            (min, max)
        };

        Ok(CAMQueryResult {
            insights,
            hyperedges,
            query_metadata: QueryMetadata {
                query_time_ms,
                total_results: insights.len(),
                returned_results: insights.len(),
                confidence_range,
            },
        })
    }

    /// Semantic search
    async fn execute_semantic(
        &self,
        query_text: String,
        domains: Option<Vec<Domain>>,
        min_confidence: f64,
        limit: usize,
    ) -> Result<(Vec<Insight>, Vec<Hyperedge>), CAMError> {
        // 1. Generate embedding for query
        let query_embedding = self.embedding_generator
            .generate(&query_text)
            .await?;

        // 2. Vector similarity search
        let mut insights = self.storage
            .semantic_search(&query_embedding, 0.7, limit * 2)
            .await?;

        // 3. Filter by domain if specified
        if let Some(domain_filter) = domains {
            insights.retain(|i| {
                domain_filter.contains(&i.primary_domain) ||
                i.secondary_domains.iter().any(|d| domain_filter.contains(d))
            });
        }

        // 4. Filter by confidence
        insights.retain(|i| i.confidence >= min_confidence);

        // 5. Limit results
        insights.truncate(limit);

        // 6. Fetch relevant hyperedges
        let insight_ids: Vec<Uuid> = insights.iter().map(|i| i.id).collect();
        let hyperedges = self.storage
            .get_hyperedges_for_insights(&insight_ids)
            .await?;

        Ok((insights, hyperedges))
    }

    /// Structural graph traversal
    async fn execute_structural(
        &self,
        start_insight_id: Uuid,
        relationship_types: Vec<RelationshipType>,
        max_depth: usize,
        limit: usize,
    ) -> Result<(Vec<Insight>, Vec<Hyperedge>), CAMError> {
        let mut visited = std::collections::HashSet::new();
        let mut insights = Vec::new();
        let mut hyperedges = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        queue.push_back((start_insight_id, 0)); // (insight_id, depth)
        visited.insert(start_insight_id);

        while let Some((insight_id, depth)) = queue.pop_front() {
            if depth > max_depth || insights.len() >= limit {
                break;
            }

            // Fetch insight
            if let Some(insight) = self.storage.get_insight(insight_id).await? {
                insights.push(insight);
            }

            // Fetch connected hyperedges
            let edges = self.storage
                .get_hyperedges_for_insight(insight_id)
                .await?;

            for edge in edges {
                // Filter by relationship type
                if !relationship_types.is_empty() && !relationship_types.contains(&edge.relationship) {
                    continue;
                }

                hyperedges.push(edge.clone());

                // Add connected insights to queue
                for connected_id in &edge.insight_ids {
                    if !visited.contains(connected_id) {
                        visited.insert(*connected_id);
                        queue.push_back((*connected_id, depth + 1));
                    }
                }
            }
        }

        Ok((insights, hyperedges))
    }

    /// Domain intersection query
    async fn execute_domain_intersection(
        &self,
        domains: Vec<Domain>,
        min_confidence: f64,
        limit: usize,
    ) -> Result<(Vec<Insight>, Vec<Hyperedge>), CAMError> {
        // Find insights spanning multiple specified domains
        let insights = self.storage
            .get_insights_spanning_domains(&domains, min_confidence, limit)
            .await?;

        let insight_ids: Vec<Uuid> = insights.iter().map(|i| i.id).collect();
        let hyperedges = self.storage
            .get_hyperedges_for_insights(&insight_ids)
            .await?;

        Ok((insights, hyperedges))
    }

    /// Temporal query
    async fn execute_temporal(
        &self,
        time_range: TimeRange,
        domains: Option<Vec<Domain>>,
        sort_by: TemporalSort,
        limit: usize,
    ) -> Result<(Vec<Insight>, Vec<Hyperedge>), CAMError> {
        let (start, end) = match time_range {
            TimeRange::LastHour => (Utc::now() - chrono::Duration::hours(1), Utc::now()),
            TimeRange::LastDay => (Utc::now() - chrono::Duration::days(1), Utc::now()),
            TimeRange::LastWeek => (Utc::now() - chrono::Duration::weeks(1), Utc::now()),
            TimeRange::LastMonth => (Utc::now() - chrono::Duration::days(30), Utc::now()),
            TimeRange::Custom { start, end } => (start, end),
        };

        let insights = self.storage
            .get_insights_in_timerange(start, end, domains, sort_by, limit)
            .await?;

        let insight_ids: Vec<Uuid> = insights.iter().map(|i| i.id).collect();
        let hyperedges = self.storage
            .get_hyperedges_for_insights(&insight_ids)
            .await?;

        Ok((insights, hyperedges))
    }

    /// Oscillation pattern query
    async fn execute_oscillation_pattern(
        &self,
        boundary: String,
        frequency_range: (f64, f64),
        amplitude_range: (f64, f64),
        limit: usize,
    ) -> Result<(Vec<Insight>, Vec<Hyperedge>), CAMError> {
        let insights = self.storage
            .get_insights_by_oscillation(boundary, frequency_range, amplitude_range, limit)
            .await?;

        let insight_ids: Vec<Uuid> = insights.iter().map(|i| i.id).collect();
        let hyperedges = self.storage
            .get_hyperedges_for_insights(&insight_ids)
            .await?;

        Ok((insights, hyperedges))
    }

    /// Hybrid query combining multiple strategies
    async fn execute_hybrid(
        &self,
        semantic: Option<Box<CAMQuery>>,
        structural: Option<Box<CAMQuery>>,
        filters: QueryFilters,
    ) -> Result<(Vec<Insight>, Vec<Hyperedge>), CAMError> {
        let mut all_insights = Vec::new();
        let mut all_hyperedges = Vec::new();

        // Execute semantic query
        if let Some(sem_query) = semantic {
            let result = self.execute(*sem_query).await?;
            all_insights.extend(result.insights);
            all_hyperedges.extend(result.hyperedges);
        }

        // Execute structural query
        if let Some(struct_query) = structural {
            let result = self.execute(*struct_query).await?;
            all_insights.extend(result.insights);
            all_hyperedges.extend(result.hyperedges);
        }

        // Deduplicate
        all_insights.sort_by_key(|i| i.id);
        all_insights.dedup_by_key(|i| i.id);

        all_hyperedges.sort_by_key(|e| e.id);
        all_hyperedges.dedup_by_key(|e| e.id);

        // Apply filters
        if let Some(min_conf) = filters.min_confidence {
            all_insights.retain(|i| i.confidence >= min_conf);
        }

        if let Some(stages) = filters.lifecycle_stages {
            all_insights.retain(|i| stages.contains(&i.lifecycle));
        }

        if let Some(domain_filter) = filters.domains {
            all_insights.retain(|i| {
                domain_filter.contains(&i.primary_domain) ||
                i.secondary_domains.iter().any(|d| domain_filter.contains(d))
            });
        }

        if filters.exclude_deprecated {
            all_insights.retain(|i| i.lifecycle != LifecycleStage::Deprecated);
        }

        Ok((all_insights, all_hyperedges))
    }
}
```

### 7.2 Query Optimization

**Strategies:**
1. **Vector Index Tuning:** Qdrant HNSW with default m=16, ef_construct=100 for optimal search
2. **Connection Pooling:** Dedicated read replicas for query workload
3. **Caching:** Redis cache for frequently accessed insights (TTL: 5 minutes)
4. **Query Planning:** PostgreSQL EXPLAIN ANALYZE for slow queries
5. **Batch Operations:** Fetch hyperedges in single query instead of N+1

```rust
/// Query cache for frequently accessed insights
pub struct QueryCache {
    cache: Arc<tokio::sync::RwLock<lru::LruCache<String, CAMQueryResult>>>,
}

impl QueryCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(capacity))),
        }
    }

    pub async fn get(&self, query_key: &str) -> Option<CAMQueryResult> {
        self.cache.write().await.get(query_key).cloned()
    }

    pub async fn insert(&self, query_key: String, result: CAMQueryResult) {
        self.cache.write().await.put(query_key, result);
    }
}
```

---

## 8. Integration with 7-Stage Flow

### 8.1 Stage 6: Pattern Extraction (Insight Capture)

**Integration Point:** After Stage 6 completes, asynchronously extract insights.

```rust
// In flow_process.rs

pub struct FlowContext {
    // ... existing fields ...

    /// Optional: CAM insight extractor
    pub insight_extractor: Option<Arc<InsightExtractor>>,
}

impl PatternExtractionProcessor {
    pub async fn process(&self, context: &mut FlowContext) -> Result<(), FlowError> {
        // ... existing pattern extraction logic ...

        // Extract patterns
        let patterns = self.extract_patterns_from_boundaries(&context.boundaries);
        context.patterns = patterns;

        // ASYNC: Trigger insight extraction (non-blocking)
        if let Some(extractor) = &context.insight_extractor {
            tokio::spawn({
                let extractor = extractor.clone();
                let flow_ctx = context.clone();

                async move {
                    match extractor.extract_from_flow(&flow_ctx).await {
                        Ok(insights) => {
                            tracing::info!(
                                "Extracted {} insights from flow {}",
                                insights.len(),
                                flow_ctx.flow_id.unwrap_or_default()
                            );

                            // Optional: Record extraction event
                            if let Some(flow_id) = flow_ctx.flow_id {
                                // Store extraction metadata
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Insight extraction failed (non-critical): {}",
                                e
                            );
                        }
                    }
                }
            });
        }

        Ok(())
    }
}
```

### 8.2 Stage 7: Adaptive Evolution (Insight Query)

**Integration Point:** Before LLM #2 generates response, query CAM for relevant insights.

```rust
impl AdaptiveEvolutionProcessor {
    pub async fn process(&self, context: &mut FlowContext) -> Result<String, FlowError> {
        // Query CAM for relevant insights
        let relevant_insights = if let Some(query_engine) = &context.cam_query_engine {
            self.fetch_relevant_insights(context, query_engine).await?
        } else {
            Vec::new()
        };

        // Enrich prompt with collective insights
        let enriched_prompt = self.build_enriched_prompt(
            context,
            &relevant_insights,
        );

        // Generate response with LLM #2
        let response = self.llm_client.complete(&enriched_prompt).await?;

        Ok(response)
    }

    async fn fetch_relevant_insights(
        &self,
        context: &FlowContext,
        query_engine: &Arc<CAMQueryEngine>,
    ) -> Result<Vec<Insight>, FlowError> {
        // Build hybrid query: semantic + domain-specific
        let query = CAMQuery::Hybrid {
            semantic: Some(Box::new(CAMQuery::Semantic {
                query_text: context.user_message.clone(),
                domains: None,
                min_confidence: 0.6,
                limit: 5,
            })),
            structural: None,
            filters: QueryFilters {
                min_confidence: Some(0.6),
                lifecycle_stages: Some(vec![
                    LifecycleStage::Validated,
                    LifecycleStage::Established,
                ]),
                domains: Some(
                    context.domains.iter()
                        .map(|d| self.domain_name_to_enum(&d.name))
                        .collect()
                ),
                exclude_deprecated: true,
            },
        };

        let result = query_engine.execute(query).await
            .map_err(|e| FlowError::StageProcessingFailed {
                stage: "AdaptiveEvolution".to_string(),
                reason: format!("CAM query failed: {}", e),
            })?;

        // Log query performance
        tracing::info!(
            "CAM query returned {} insights in {}ms",
            result.insights.len(),
            result.query_metadata.query_time_ms
        );

        Ok(result.insights)
    }

    fn build_enriched_prompt(
        &self,
        context: &FlowContext,
        insights: &[Insight],
    ) -> String {
        let mut prompt = format!(
            "USER MESSAGE: {}\n\nDOMAIN CONTEXT:\n{}\n",
            context.user_message,
            self.format_domain_context(&context.domains),
        );

        if !insights.is_empty() {
            prompt.push_str("\nCOLLECTIVE INSIGHTS (from AI instances):\n");
            for (i, insight) in insights.iter().enumerate() {
                prompt.push_str(&format!(
                    "{}. [Confidence: {:.2}] {}\n",
                    i + 1,
                    insight.confidence,
                    insight.content
                ));
            }
            prompt.push_str("\nConsider these collective insights when formulating your response.\n");
        }

        prompt
    }
}
```

### 8.3 Initialization

```rust
/// Initialize CAM system components
pub async fn initialize_cam_system(
    db_pool: PgPool,
    llm_client: Arc<dyn LLMClient>,
    instance_id: Uuid,
) -> Result<CAMComponents, CAMError> {
    // Storage
    let storage = Arc::new(CAMStorage::new(db_pool.clone()));

    // Embedding generator
    let embedding_generator = Arc::new(EmbeddingGenerator::new(llm_client.clone()));

    // Insight extractor
    let insight_extractor = Arc::new(InsightExtractor {
        llm_client: llm_client.clone(),
        embedding_generator: embedding_generator.clone(),
        storage: storage.clone(),
        instance_id,
    });

    // Query engine
    let query_engine = Arc::new(CAMQueryEngine {
        storage: storage.clone(),
        embedding_generator: embedding_generator.clone(),
    });

    // Validation pipeline
    let validation_pipeline = Arc::new(ValidationPipeline {
        storage: storage.clone(),
        llm_client: llm_client.clone(),
        instance_id,
    });

    // Start background validation scheduler
    tokio::spawn(run_validation_scheduler(validation_pipeline.clone()));

    Ok(CAMComponents {
        storage,
        embedding_generator,
        insight_extractor,
        query_engine,
        validation_pipeline,
    })
}

pub struct CAMComponents {
    pub storage: Arc<CAMStorage>,
    pub embedding_generator: Arc<EmbeddingGenerator>,
    pub insight_extractor: Arc<InsightExtractor>,
    pub query_engine: Arc<CAMQueryEngine>,
    pub validation_pipeline: Arc<ValidationPipeline>,
}
```

---

## 9. Performance Considerations

### 9.1 Latency Targets

| Operation | Target | P95 | Strategy |
|-----------|--------|-----|----------|
| Semantic Query | <50ms | <100ms | Qdrant HNSW index, connection pooling |
| Structural Traversal | <80ms | <150ms | BFS with early termination, depth limits |
| Insight Extraction | <200ms | <500ms | Async, non-blocking, batch embeddings |
| Validation Cycle | N/A | N/A | Background task, off-peak hours |

### 9.2 Indexing Strategy

**Qdrant Vector Index (HNSW):**
```yaml
# Qdrant collection configuration
hnsw_config:
  m: 16                    # Default, good balance
  ef_construct: 100        # Construction quality

# For high accuracy (slower indexing):
# m: 32, ef_construct: 200

# For faster indexing (lower recall):
# m: 8, ef_construct: 50
```

**PostgreSQL Metadata Indexes:**
```sql
-- Composite indexes for common queries
CREATE INDEX idx_cam_insights_domain_confidence ON cam_insights(primary_domain, confidence DESC);
CREATE INDEX idx_cam_insights_lifecycle_validated ON cam_insights(lifecycle_stage, last_validated DESC);
```

### 9.3 Caching Strategy

```rust
pub struct CAMCache {
    /// LRU cache for insights (500 most accessed)
    insight_cache: Arc<RwLock<lru::LruCache<Uuid, Insight>>>,

    /// Query result cache (100 most recent queries)
    query_cache: Arc<RwLock<lru::LruCache<String, CAMQueryResult>>>,

    /// Embedding cache (1000 most recent)
    embedding_cache: Arc<RwLock<lru::LruCache<String, Vec<f32>>>>,
}

impl CAMCache {
    pub fn new() -> Self {
        Self {
            insight_cache: Arc::new(RwLock::new(lru::LruCache::new(500))),
            query_cache: Arc::new(RwLock::new(lru::LruCache::new(100))),
            embedding_cache: Arc::new(RwLock::new(lru::LruCache::new(1000))),
        }
    }

    pub async fn get_insight(&self, id: Uuid) -> Option<Insight> {
        self.insight_cache.write().await.get(&id).cloned()
    }

    pub async fn put_insight(&self, insight: Insight) {
        self.insight_cache.write().await.put(insight.id, insight);
    }
}
```

### 9.4 Database Optimization

```sql
-- Partitioning by creation date for large datasets
CREATE TABLE cam_insights_2025_01 PARTITION OF cam_insights
    FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

CREATE TABLE cam_insights_2025_02 PARTITION OF cam_insights
    FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');

-- Vacuum and analyze regularly
VACUUM ANALYZE cam_insights;
VACUUM ANALYZE cam_hyperedges;

-- Monitor query performance
SELECT * FROM pg_stat_statements
WHERE query LIKE '%cam_insights%'
ORDER BY mean_exec_time DESC
LIMIT 10;
```

### 9.5 Scalability Projections

| Metric | 10K Insights | 100K Insights | 1M Insights | 10M Insights |
|--------|-------------|--------------|------------|--------------|
| Storage | ~5 MB | ~50 MB | ~500 MB | ~5 GB |
| Query Time (semantic) | <20ms | <50ms | <100ms | <200ms |
| Extraction Overhead | <100ms | <150ms | <200ms | <300ms |
| Daily Validations | 100 | 500 | 2000 | 10000 |

---

## 10. Phased Implementation Plan

### Phase 1: MVP (4 weeks)

**Goal:** Basic insight storage and retrieval.

**Deliverables:**
1. Database schema (cam_insights, cam_hyperedges)
2. Insight data structure (Rust)
3. Basic storage operations (insert, get, search)
4. Semantic search (Qdrant HNSW)
5. Integration with Stage 6 (async extraction)
6. Integration with Stage 7 (basic query)

**Success Criteria:**
- Insights stored during flow execution
- Stage 7 can retrieve top 3 relevant insights
- Query latency <200ms (no optimization yet)

**TDF Analysis:**
- **COMP:** Core data structures validated
- **SCI:** Basic functionality proven
- **CULT:** Team familiarizes with CAM concepts
- **EXP:** First taste of collective learning

### Phase 2: Validation Pipeline (3 weeks)

**Goal:** Maintain insight quality through validation.

**Deliverables:**
1. Validation schema (cam_validations)
2. Contradiction detection
3. Consensus validation
4. Confidence decay model
5. Background validation scheduler
6. Lifecycle management (Emerging → Validated → Established)

**Success Criteria:**
- 90% of Established insights remain high-confidence after 30 days
- Contradictions detected within 24 hours
- Deprecated insights removed automatically

**TDF Analysis:**
- **SCI:** Empirical quality control
- **COMP:** Automated maintenance
- **CULT:** Trust in collective knowledge

### Phase 3: Hypergraph Relationships (3 weeks)

**Goal:** Rich multi-way relationships between insights.

**Deliverables:**
1. Hyperedge data structure
2. Relationship discovery algorithms
3. Structural query engine
4. Graph traversal (BFS with depth limits)
5. Visualization tools (optional)

**Success Criteria:**
- 70% of insights connected via hyperedges
- Structural queries complete in <150ms
- Meaningful relationships discovered automatically

**TDF Analysis:**
- **COMP:** Graph algorithms implemented
- **META:** Emergent knowledge networks
- **EXP:** Navigating collective understanding

### Phase 4: Advanced Queries (2 weeks)

**Goal:** Sophisticated query capabilities.

**Deliverables:**
1. Domain intersection queries
2. Temporal queries (recent, trending)
3. Oscillation pattern queries
4. Hybrid query engine
5. Query optimization (indexes, caching)

**Success Criteria:**
- All query types <100ms (p95)
- Hybrid queries combine semantic + structural
- Cache hit rate >60%

**TDF Analysis:**
- **COMP:** Performance optimization
- **SCI:** Measurable improvements
- **EXP:** Fluid knowledge access

### Phase 5: Production Hardening (2 weeks)

**Goal:** Production-ready system.

**Deliverables:**
1. Comprehensive error handling
2. Monitoring and alerting (Prometheus/Grafana)
3. Performance benchmarks
4. Security audit (SQL injection, access control)
5. Documentation (API, architecture)
6. Migration guide

**Success Criteria:**
- Zero data loss under failure scenarios
- <0.1% error rate
- Full observability (traces, metrics, logs)
- Security review passed

**TDF Analysis:**
- **COMP:** Robust engineering
- **SCI:** Evidence-based reliability
- **CULT:** Deployment confidence
- **EXP:** Smooth operations

**Total Timeline:** 14 weeks (~3.5 months)

---

## 11. Example Queries & Use Cases

### 11.1 Example Query: Semantic Search

**Scenario:** User asks about "best practices for async Rust"

```rust
let query = CAMQuery::Semantic {
    query_text: "best practices for async Rust programming".to_string(),
    domains: Some(vec![Domain::Computational]),
    min_confidence: 0.6,
    limit: 5,
};

let result = query_engine.execute(query).await?;

// Result might include:
// 1. [0.85] "Use tokio::spawn for CPU-bound tasks to avoid blocking executor"
// 2. [0.78] "Prefer bounded channels to prevent memory leaks in long-running services"
// 3. [0.72] "Always set timeouts on external API calls to prevent hanging"
```

### 11.2 Example Query: Domain Intersection

**Scenario:** Find insights spanning Computational and Experiential domains

```rust
let query = CAMQuery::DomainIntersection {
    domains: vec![Domain::Computational, Domain::Experiential],
    min_confidence: 0.7,
    limit: 10,
};

let result = query_engine.execute(query).await?;

// Result might include:
// 1. "Users perceive 100ms delays as 'slow' in UI interactions, regardless of actual complexity"
// 2. "Error messages with concrete examples reduce user frustration by 60%"
```

### 11.3 Example Query: Oscillation Pattern

**Scenario:** Find insights that emerged during high-frequency CD-ED oscillations

```rust
let query = CAMQuery::OscillationPattern {
    boundary: "CD-ED".to_string(),
    frequency_range: (1.5, 2.5),  // High frequency
    amplitude_range: (0.5, 1.0),  // High amplitude
    limit: 5,
};

let result = query_engine.execute(query).await?;

// Result might include insights about computational-experiential synthesis
```

### 11.4 Example Query: Temporal (Trending)

**Scenario:** What insights are gaining traction this week?

```rust
let query = CAMQuery::Temporal {
    time_range: TimeRange::LastWeek,
    domains: None,
    sort_by: TemporalSort::HighestConfidenceGrowth,
    limit: 10,
};

let result = query_engine.execute(query).await?;

// Result shows insights with rapidly growing observation counts
```

### 11.5 Example Use Case: Multi-Instance Learning

**Scenario:** Instance A discovers a pattern, Instance B validates it

**Instance A (Day 1):**
```rust
// User asks: "Why is my Rust build so slow?"
// Instance A discovers insight during CD-SD oscillation

let insight = Insight::new(
    "Rust incremental compilation breaks when Cargo.lock changes frequently; \
     lock dependencies to stable versions in CI/CD".to_string(),
    Domain::Computational,
    vec![Domain::Scientific],
    instance_a_id,
    oscillation_context,
);

storage.insert_insight(&insight).await?;
// Confidence: 0.5 (Validated)
```

**Instance B (Day 3):**
```rust
// Different user asks similar question
// Instance B queries CAM, finds Instance A's insight

let query = CAMQuery::Semantic {
    query_text: "slow Rust compilation times".to_string(),
    domains: Some(vec![Domain::Computational]),
    min_confidence: 0.4,
    limit: 5,
};

let result = query_engine.execute(query).await?;
// Finds Instance A's insight, includes in response

// Instance B observes same pattern, increments observation count
storage.increment_observation(insight.id).await?;
// Confidence: 0.6 → 0.7 (validated by 2nd instance)
```

**Instance C (Day 7):**
```rust
// Third instance observes same pattern
storage.increment_observation(insight.id).await?;
// Confidence: 0.7 → 0.8 (Established)
// Lifecycle: Validated → Established
```

### 11.6 Example Use Case: Contradiction Detection

**Scenario:** Two insights contradict each other

```rust
// Insight 1: "Always use async for I/O operations"
// Insight 2: "Avoid async for simple file reads; overhead exceeds benefit"

// Validation pipeline detects semantic contradiction
let validation = ValidationPipeline::validate_insight(&insight_2).await?;

if let ValidationOutcome::Contradicted = validation.outcome {
    // Create Contradiction hyperedge
    let hyperedge = Hyperedge::new(
        vec![insight_1.id, insight_2.id],
        RelationshipType::Contradiction,
        0.8,  // Strong contradiction
        validator_instance_id,
        DiscoveryMethod::LLMInference,
    )?;

    storage.insert_hyperedge(&hyperedge).await?;

    // Flag for human review
    // Both insights marked for manual curation
}
```

---

## 12. Risk Analysis & Mitigations

### 12.1 Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|------------|------------|
| **Qdrant scaling beyond 10M vectors** | Medium | Low | Monitor performance; consider sharding/replication if needed; Qdrant designed for 100M+ scale |
| **Insight extraction increases latency** | Medium | High | Async extraction (non-blocking); monitor Stage 6 duration; circuit breaker if >500ms |
| **False contradictions deprecate valid insights** | High | Medium | Human review queue for contradictions; require 3+ instances to agree on deprecation |
| **Database connection pool exhaustion** | High | Low | Dedicated read replicas for queries; connection pooling with max limits; alerting |
| **Embedding API rate limits** | Medium | Medium | Batch embedding generation; local embedding models (Sentence Transformers); caching |

### 12.2 Data Quality Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|------------|------------|
| **Low-quality insights pollute collective memory** | High | High | Minimum confidence threshold (0.5); automatic deprecation if not re-observed; LLM re-validation |
| **Stale insights persist indefinitely** | Medium | High | Confidence decay model; automatic revalidation; lifecycle management |
| **Duplicate insights with different wording** | Low | High | Semantic deduplication (cosine similarity >0.9); merge duplicates; increment observation count |
| **Biased insights from single domain** | Medium | Medium | Encourage multi-domain insights; track domain balance; alert if >80% single domain |

### 12.3 Operational Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|------------|------------|
| **Database migration failures** | High | Low | Comprehensive testing; rollback plan; blue-green deployment; automated backups |
| **Validation pipeline crashes** | Medium | Low | Graceful error handling; retry logic; circuit breaker; monitoring alerts |
| **Insight storage fills disk** | Medium | Medium | Partition by date; archive old insights; compression; monitoring disk usage |
| **Query performance degrades over time** | Medium | Medium | Regular VACUUM ANALYZE; index rebuilding; query optimization; monitoring |

### 12.4 Privacy & Security Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|------------|------------|
| **User data leaks into collective insights** | Critical | Low | Anonymize user_id in insights; PII detection; manual review for Tier 3 insights |
| **SQL injection in query engine** | Critical | Very Low | SQLx compile-time checks; parameterized queries; security audit |
| **Unauthorized access to insights** | High | Low | Row-level security; API authentication; instance_id validation |

---

## Appendix A: Embedding Generation

```rust
pub struct EmbeddingGenerator {
    llm_client: Arc<dyn LLMClient>,
    cache: Arc<RwLock<lru::LruCache<String, Vec<f32>>>>,
}

impl EmbeddingGenerator {
    pub fn new(llm_client: Arc<dyn LLMClient>) -> Self {
        Self {
            llm_client,
            cache: Arc::new(RwLock::new(lru::LruCache::new(1000))),
        }
    }

    pub async fn generate(&self, text: &str) -> Result<Vec<f32>, CAMError> {
        // Check cache
        if let Some(cached) = self.cache.write().await.get(text) {
            return Ok(cached.clone());
        }

        // Generate embedding via LLM API
        let embedding = self.llm_client
            .embed(text)
            .await
            .map_err(|e| CAMError::EmbeddingError(e.to_string()))?;

        // Cache result
        self.cache.write().await.put(text.to_string(), embedding.clone());

        Ok(embedding)
    }

    pub async fn batch_generate(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, CAMError> {
        // Batch API call for efficiency
        let embeddings = self.llm_client
            .embed_batch(texts)
            .await
            .map_err(|e| CAMError::EmbeddingError(e.to_string()))?;

        // Cache results
        for (text, embedding) in texts.iter().zip(&embeddings) {
            self.cache.write().await.put(text.to_string(), embedding.clone());
        }

        Ok(embeddings)
    }
}
```

---

## Appendix B: Migration Scripts

```sql
-- Migration: Add CAM tables to existing database

BEGIN;

-- Create cam_insights table
CREATE TABLE cam_insights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    embedding vector(1536),
    primary_domain TEXT NOT NULL,
    secondary_domains TEXT[] NOT NULL DEFAULT '{}',
    confidence REAL NOT NULL DEFAULT 0.3 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    lifecycle_stage TEXT NOT NULL DEFAULT 'Emerging',
    observation_count INTEGER NOT NULL DEFAULT 1,
    source_instance_id UUID NOT NULL,
    source_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    source_flow_id UUID REFERENCES flow_process_executions(id) ON DELETE SET NULL,
    oscillation_context JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_validated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}'
);

-- Create indexes
CREATE INDEX idx_cam_insights_embedding ON cam_insights
    USING ivfflat (embedding vector_cosine_ops)
    WITH (lists = 100);

CREATE INDEX idx_cam_insights_domain ON cam_insights(primary_domain);
CREATE INDEX idx_cam_insights_lifecycle ON cam_insights(lifecycle_stage);
CREATE INDEX idx_cam_insights_confidence ON cam_insights(confidence DESC);

-- Create cam_hyperedges table
CREATE TABLE cam_hyperedges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    insight_ids UUID[] NOT NULL,
    relationship_type TEXT NOT NULL,
    strength REAL NOT NULL DEFAULT 0.5 CHECK (strength >= 0.0 AND strength <= 1.0),
    spanning_domains TEXT[] NOT NULL DEFAULT '{}',
    discovery_method TEXT NOT NULL,
    discovered_by UUID NOT NULL,
    observation_count INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}',
    CONSTRAINT hyperedge_min_insights CHECK (array_length(insight_ids, 1) >= 2)
);

CREATE INDEX idx_cam_hyperedges_insights ON cam_hyperedges USING GIN(insight_ids);

-- Create cam_validations table
CREATE TABLE cam_validations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    insight_id UUID NOT NULL REFERENCES cam_insights(id) ON DELETE CASCADE,
    validator_instance_id UUID NOT NULL,
    validation_method TEXT NOT NULL,
    passed BOOLEAN NOT NULL,
    new_confidence REAL CHECK (new_confidence >= 0.0 AND new_confidence <= 1.0),
    findings JSONB NOT NULL DEFAULT '{}',
    contradictions UUID[] DEFAULT '{}',
    validated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_cam_validations_insight ON cam_validations(insight_id);

COMMIT;
```

---

## Conclusion

The **Collective Associative Memory (CAM)** system represents a novel approach to continuous AI learning without retraining. By capturing insights at the moment of cross-domain oscillation and storing them in a hypergraph structure, we enable:

1. **Collective Intelligence:** All AI instances learn from each other's discoveries
2. **Personality Preservation:** Three-tier memory ensures individual identity remains intact
3. **Quality Maintenance:** Automated validation prevents knowledge decay
4. **Emergent Understanding:** Hypergraph relationships reveal hidden connections
5. **Performance at Scale:** <100ms queries even at 10M+ insights

**Next Steps:**
1. Review this design with team (TDF multi-domain perspective)
2. Approve Phase 1 implementation (4 weeks)
3. Set up development environment (PostgreSQL + Qdrant via Docker)
4. Begin Rust module structure (api/src/cam/)

**TDF Reflection:**
This design harmonizes computational rigor (COMP), empirical validation (SCI), philosophical depth (META), contextual awareness (CULT), and experiential elegance (EXP). The CAM system doesn't just store facts—it cultivates collective wisdom that emerges from the recursive interplay of boundary dissolution and identity preservation.

---

**Document End**
