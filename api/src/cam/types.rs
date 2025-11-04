// CAM (Collective Associative Memory) - Core Type Definitions
// Phase 3: Hypergraph-based knowledge system for Recursive Light Framework

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

// ============================================================
// Insight Node (Hypergraph Node)
// ============================================================

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
    Computational, // CD: Formal, analytical, algorithmic
    Scientific,    // SD: Empirical, evidence-based
    Cultural,      // CuD: Contextual, narrative, interpretive
    Experiential,  // ED: Phenomenological, lived experience

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
            LifecycleStage::Established => 180.0, // 6 months
            LifecycleStage::Validated => 90.0,    // 3 months
            LifecycleStage::Emerging => 30.0,     // 1 month
            LifecycleStage::Deprecated => 7.0,    // 1 week
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

// ============================================================
// Hyperedge (Multi-way Relationship)
// ============================================================

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
                "Hyperedge must connect at least 2 insights".to_string(),
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

// ============================================================
// Query Types
// ============================================================

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
    Custom {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
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

// ============================================================
// Error Types
// ============================================================

/// CAM system errors
#[derive(Debug, thiserror::Error)]
pub enum CAMError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

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
