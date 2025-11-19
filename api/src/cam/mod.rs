// CAM (Collective Associative Memory) - Public API
// Phase 3: Hypergraph-based knowledge system for Recursive Light Framework

pub mod embeddings;
pub mod manager;
pub mod qdrant_storage;
pub mod storage;
pub mod types;

// Re-export core types for convenience
pub use types::{
    CAMError, CAMQuery, CAMQueryResult, DiscoveryMethod, Domain, Hyperedge, Insight,
    LifecycleStage, OscillationContext, PhenomenologicalQualities, QueryFilters, QueryMetadata,
    RelationshipType, TemporalSort, TimeRange,
};

// Re-export primary API (most users should use CAMManager)
pub use manager::CAMManager;

// Re-export storage layers (advanced usage)
pub use qdrant_storage::QdrantVectorStorage;
pub use storage::CAMStorage;
